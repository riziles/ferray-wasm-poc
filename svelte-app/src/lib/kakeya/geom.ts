// Plane geometry for the Kakeya page: polygons, rasterisation, and a rough
// signed distance field. Everything here is plain TypeScript — the page must
// render without the WASM binary (see wasmOptionalRoutes in +layout.svelte).
//
// Rasterisation is how most of the numbers on this page are *measured* rather
// than quoted: the Perron tree's area, the dimension estimate, and the union
// area of a bush of tubes are all counted off a grid, at a stated resolution.

export type Pt = [number, number];
export type Poly = Pt[];

export interface Box {
	x0: number;
	y0: number;
	x1: number;
	y1: number;
}

/** Anything that behaves like a raster: the box-counting and distance code only needs these three fields. */
export type CellSet = Pick<Raster, 'n' | 'box' | 'cell'>;

/** A square grid of cells covering `box`. `cell[j * n + i]` is 1 when occupied. */
export interface Raster {
	n: number;
	box: Box;
	cell: Uint8Array;
}

export function boxOf(polys: Poly[], pad = 0): Box {
	let x0 = Infinity;
	let y0 = Infinity;
	let x1 = -Infinity;
	let y1 = -Infinity;
	for (const poly of polys) {
		for (const [x, y] of poly) {
			if (x < x0) x0 = x;
			if (y < y0) y0 = y;
			if (x > x1) x1 = x;
			if (y > y1) y1 = y;
		}
	}
	return { x0: x0 - pad, y0: y0 - pad, x1: x1 + pad, y1: y1 + pad };
}

/** Grow the shorter side so the box is square: keeps grid cells isotropic. */
export function squareBox(b: Box): Box {
	const side = Math.max(b.x1 - b.x0, b.y1 - b.y0);
	const cx = (b.x0 + b.x1) / 2;
	const cy = (b.y0 + b.y1) / 2;
	return { x0: cx - side / 2, y0: cy - side / 2, x1: cx + side / 2, y1: cy + side / 2 };
}

export function padBox(b: Box, pad: number): Box {
	return { x0: b.x0 - pad, y0: b.y0 - pad, x1: b.x1 + pad, y1: b.y1 + pad };
}

/** Even-odd polygon area (positive for counter-clockwise winding). */
export function polyArea(poly: Poly): number {
	let a = 0;
	for (let i = 0, j = poly.length - 1; i < poly.length; j = i++) {
		a += poly[j][0] * poly[i][1] - poly[i][0] * poly[j][1];
	}
	return Math.abs(a) / 2;
}

export function rotate(p: Pt, ang: number): Pt {
	const c = Math.cos(ang);
	const s = Math.sin(ang);
	return [c * p[0] - s * p[1], s * p[0] + c * p[1]];
}

export function rasterize(polys: Poly[], n: number, box: Box): Raster {
	const cell = new Uint8Array(n * n);
	const sx = n / (box.x1 - box.x0);
	const sy = n / (box.y1 - box.y0);

	for (const poly of polys) {
		let ymin = Infinity;
		let ymax = -Infinity;
		for (const p of poly) {
			if (p[1] < ymin) ymin = p[1];
			if (p[1] > ymax) ymax = p[1];
		}
		const j0 = Math.max(0, Math.floor((ymin - box.y0) * sy));
		const j1 = Math.min(n - 1, Math.ceil((ymax - box.y0) * sy));

		for (let j = j0; j <= j1; j++) {
			const py = box.y0 + (j + 0.5) / sy;
			const xs: number[] = [];
			for (let e = 0, f = poly.length - 1; e < poly.length; f = e++) {
				const [xa, ya] = poly[f];
				const [xb, yb] = poly[e];
				if (ya > py !== yb > py) xs.push(xa + ((xb - xa) * (py - ya)) / (yb - ya));
			}
			if (xs.length < 2) continue;
			xs.sort((a, b) => a - b);
			for (let k = 0; k + 1 < xs.length; k += 2) {
				// Expand by half a cell on both sides: a triangle thinner than one
				// cell still has to leave a mark, or the deep levels of the Perron
				// tree would silently vanish from the picture.
				const i0 = Math.max(0, Math.ceil((xs[k] - box.x0) * sx - 0.5));
				const i1 = Math.min(n - 1, Math.floor((xs[k + 1] - box.x0) * sx - 0.5));
				for (let i = i0; i <= i1; i++) cell[j * n + i] = 1;
			}
		}
	}
	return { n, box, cell };
}

export function rasterArea(r: Raster): number {
	let c = 0;
	for (let i = 0; i < r.cell.length; i++) c += r.cell[i];
	const w = (r.box.x1 - r.box.x0) / r.n;
	const h = (r.box.y1 - r.box.y0) / r.n;
	return c * w * h;
}

export function occupied(r: Raster): number {
	let c = 0;
	for (let i = 0; i < r.cell.length; i++) c += r.cell[i];
	return c;
}

/**
 * Approximate signed distance from each cell centre to the boundary, in world
 * units: positive inside, negative outside. A two-pass chamfer transform, run
 * once inside and once outside — cheap, and accurate enough to rank placements
 * by "how much room is left", and to let a needle sit exactly on the boundary
 * (which is where the tightest configurations live).
 */
export function signedDistance(r: CellSet): Float32Array {
	const { n, cell, box } = r;
	const scale = Math.min((box.x1 - box.x0) / n, (box.y1 - box.y0) / n);
	const diag = 1.4142;

	const chamfer = (seed: Uint8Array): Float32Array => {
		const d = new Float32Array(n * n);
		const far = 1e9;
		for (let i = 0; i < d.length; i++) d[i] = seed[i] ? far : 0;
		for (let j = 0; j < n; j++) {
			for (let i = 0; i < n; i++) {
				const k = j * n + i;
				if (d[k] === 0) continue;
				let v = d[k];
				if (j > 0) {
					v = Math.min(v, d[k - n] + 1);
					if (i > 0) v = Math.min(v, d[k - n - 1] + diag);
					if (i < n - 1) v = Math.min(v, d[k - n + 1] + diag);
				}
				if (i > 0) v = Math.min(v, d[k - 1] + 1);
				d[k] = v;
			}
		}
		for (let j = n - 1; j >= 0; j--) {
			for (let i = n - 1; i >= 0; i--) {
				const k = j * n + i;
				if (d[k] === 0) continue;
				let v = d[k];
				if (j < n - 1) {
					v = Math.min(v, d[k + n] + 1);
					if (i > 0) v = Math.min(v, d[k + n - 1] + diag);
					if (i < n - 1) v = Math.min(v, d[k + n + 1] + diag);
				}
				if (i < n - 1) v = Math.min(v, d[k + 1] + 1);
				d[k] = v;
			}
		}
		return d;
	};

	const fromInside = chamfer(cell);
	const complement = new Uint8Array(cell.length);
	for (let i = 0; i < cell.length; i++) complement[i] = cell[i] ? 0 : 1;
	const fromOutside = chamfer(complement);

	const out = new Float32Array(cell.length);
	for (let i = 0; i < cell.length; i++) {
		out[i] = cell[i] ? fromInside[i] * scale : -fromOutside[i] * scale;
	}
	return out;
}

/** Signed distance at a world point, nearest-cell sampling. */
export function sampleDist(field: Float32Array, r: CellSet, x: number, y: number): number {
	const { n, box } = r;
	const i = Math.floor(((x - box.x0) / (box.x1 - box.x0)) * n);
	const j = Math.floor(((y - box.y0) / (box.y1 - box.y0)) * n);
	if (i < 0 || j < 0 || i >= n || j >= n) return -1;
	return field[j * n + i];
}

/** One grid cell, in world units. */
export function cellSize(r: CellSet): number {
	return Math.min((r.box.x1 - r.box.x0) / r.n, (r.box.y1 - r.box.y0) / r.n);
}

/** Number of occupied cells when the grid is coarsened by a factor `k`. */
export function boxCount(r: CellSet, k: number): number {
	const m = Math.floor(r.n / k);
	let count = 0;
	for (let bj = 0; bj < m; bj++) {
		for (let bi = 0; bi < m; bi++) {
			let any = 0;
			for (let j = 0; j < k && !any; j++) {
				const row = (bj * k + j) * r.n + bi * k;
				for (let i = 0; i < k; i++) {
					if (r.cell[row + i]) {
						any = 1;
						break;
					}
				}
			}
			count += any;
		}
	}
	return count;
}

/** Least-squares slope of log N(δ) against log(1/δ): the box-counting dimension. */
export function boxDimension(r: CellSet, ks: number[]): { slope: number; points: [number, number][] } {
	const points = ks.map((k) => {
		const nb = Math.floor(r.n / k);
		const delta = (r.box.x1 - r.box.x0) / nb;
		return [Math.log(1 / delta), Math.log(boxCount(r, k) + 1e-9)] as [number, number];
	});
	const mx = points.reduce((s, p) => s + p[0], 0) / points.length;
	const my = points.reduce((s, p) => s + p[1], 0) / points.length;
	let num = 0;
	let den = 0;
	for (const [x, y] of points) {
		num += (x - mx) * (y - my);
		den += (x - mx) * (x - mx);
	}
	return { slope: den > 0 ? num / den : 0, points };
}

export function diskPoly(r: number, steps = 220): Poly {
	const out: Poly = [];
	for (let i = 0; i < steps; i++) {
		const t = (i / steps) * Math.PI * 2;
		out.push([r * Math.cos(t), r * Math.sin(t)]);
	}
	return out;
}

/** Equilateral triangle of height 1, base on the line y = -1/2, apex up. */
export function equilateralPoly(): Poly {
	const h = 1;
	const half = h / Math.sqrt(3); // base half-width = h/√3, base = 2h/√3
	return [
		[0, h / 2],
		[half, -h / 2],
		[-half, -h / 2]
	];
}

export function rectPoly(cx: number, cy: number, len: number, wid: number, ang: number): Poly {
	const c = Math.cos(ang);
	const s = Math.sin(ang);
	const hl = len / 2;
	const hw = wid / 2;
	const corners: Pt[] = [
		[-hl, -hw],
		[hl, -hw],
		[hl, hw],
		[-hl, hw]
	];
	return corners.map(([x, y]) => [cx + c * x - s * y, cy + s * x + c * y] as Pt);
}
