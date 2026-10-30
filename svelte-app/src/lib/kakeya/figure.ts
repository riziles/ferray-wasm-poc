// Turning the eight rotated copies of a Perron tree into a measurable set, and
// finding where a unit needle can sit. Rotating eight rasters is eight times
// cheaper than rasterising eight trees, and one copy plus a rotation is exactly
// what the construction is: the tree, repeated under the rotations that tile the
// circle.

import {
	boxOf,
	rasterize,
	squareBox,
	signedDistance,
	type CellSet,
	sampleDist,
	type Box,
	type Poly,
	type Raster
} from './geom';

export interface SetRaster {
	n: number;
	box: Box;
	cell: Uint8Array;
	/** Which copy covered this cell (0..copies-1), or -1. */
	copy: Int8Array;
}

/**
 * The union of `copies` rotations of `src` about the origin, on an `outN` grid
 * covering the whole rotated figure. `src` must already be a raster of the
 * unrotated piece in its own box.
 */
export function rotationalUnion(
	src: Raster,
	copies: number,
	step: number,
	outN: number
): SetRaster {
	let radius = 0;
	for (const x of [src.box.x0, src.box.x1]) {
		for (const y of [src.box.y0, src.box.y1]) {
			radius = Math.max(radius, Math.hypot(x, y));
		}
	}
	const box: Box = { x0: -radius, y0: -radius, x1: radius, y1: radius };
	const cell = new Uint8Array(outN * outN);
	const copy = new Int8Array(outN * outN).fill(-1);
	const w = (box.x1 - box.x0) / outN;
	const cosT: number[] = [];
	const sinT: number[] = [];
	for (let c = 0; c < copies; c++) {
		cosT.push(Math.cos(c * step));
		sinT.push(Math.sin(c * step));
	}

	const sw = (src.box.x1 - src.box.x0) / src.n;
	const sh = (src.box.y1 - src.box.y0) / src.n;

	const sample = (x: number, y: number): number => {
		// Bilinear sample of the 0/1 occupancy, so a 45° rotation does not comb
		// the figure into stripes.
		const fx = (x - src.box.x0) / sw - 0.5;
		const fy = (y - src.box.y0) / sh - 0.5;
		const i = Math.floor(fx);
		const j = Math.floor(fy);
		if (i < -1 || j < -1 || i > src.n - 1 || j > src.n - 1) return 0;
		const tx = fx - i;
		const ty = fy - j;
		const at = (ii: number, jj: number) => {
			if (ii < 0 || jj < 0 || ii >= src.n || jj >= src.n) return 0;
			return src.cell[jj * src.n + ii];
		};
		const top = at(i, j) * (1 - tx) + at(i + 1, j) * tx;
		const bot = at(i, j + 1) * (1 - tx) + at(i + 1, j + 1) * tx;
		return top * (1 - ty) + bot * ty;
	};

	for (let j = 0; j < outN; j++) {
		const y = box.y0 + (j + 0.5) * w;
		for (let i = 0; i < outN; i++) {
			const x = box.x0 + (i + 0.5) * w;
			for (let c = 0; c < copies; c++) {
				// Rotate the sample back by c*step (trig hoisted: 8 rotations per cell).
				const ct = cosT[c];
				const st = sinT[c];
				const rx = x * ct + y * st;
				const ry = -x * st + y * ct;
				if (sample(rx, ry) > 0.5) {
					const k = j * outN + i;
					cell[k] = 1;
					copy[k] = c;
					break;
				}
			}
		}
	}
	return { n: outN, box, cell, copy };
}

/** Raster of a plain polygon set, at `outN` cells across its own bounding box. */
export function polysToRaster(polys: Poly[], outN: number, pad = 0.02): Raster {
	return rasterize(polys, outN, squareBox(expand(boxOf(polys), pad)));
}

function expand(b: Box, pad: number): Box {
	const p = pad * Math.max(b.x1 - b.x0, b.y1 - b.y0);
	return { x0: b.x0 - p, y0: b.y0 - p, x1: b.x1 + p, y1: b.y1 + p };
}

export function setArea(s: { n: number; box: Box; cell: Uint8Array }): number {
	let c = 0;
	for (let i = 0; i < s.cell.length; i++) c += s.cell[i] ? 1 : 0;
	const w = (s.box.x1 - s.box.x0) / s.n;
	const h = (s.box.y1 - s.box.y0) / s.n;
	return c * w * h;
}

export interface Placement {
	ok: boolean;
	/** Needle centre and endpoints, world coordinates. */
	cx: number;
	cy: number;
	a: [number, number];
	b: [number, number];
	/** Smallest inside-distance along the needle: how much room is left. */
	margin: number;
}

/**
 * Find the placement of a unit needle with direction `theta` that sits deepest
 * inside the set. A coarse sweep followed by a local refinement — the answer is
 * used as a "magnet" in the puzzle, so it only has to be the best placement,
 * not a continuous path.
 */
export function deepestPlacement(
	set: { n: number; box: Box; cell: Uint8Array },
	sdf: Float32Array,
	theta: number,
	len = 1,
	samples = 15,
	tol = 0
): Placement {
	const { n, box } = set;
	const dx = (Math.cos(theta) * len) / 2;
	const dy = (Math.sin(theta) * len) / 2;
	const cellW = (box.x1 - box.x0) / n;
	const cellH = (box.y1 - box.y0) / n;
	const half = len / 2;
	const offs: [number, number][] = [];
	for (let s = 0; s < samples; s++) {
		const u = -half + (len * s) / (samples - 1);
		offs.push([Math.cos(theta) * u, Math.sin(theta) * u]);
	}

	const score = (i: number, j: number): number => {
		const x = box.x0 + (i + 0.5) * cellW;
		const y = box.y0 + (j + 0.5) * cellH;
		let m = Infinity;
		for (const [ox, oy] of offs) {
			const v = sampleDist(sdf, set, x + ox, y + oy);
			if (v < m) m = v;
			if (m <= -tol) return m;
		}
		return m;
	};

	let best = -Infinity;
	let bi = -1;
	let bj = -1;
	const stride = Math.max(1, Math.round(n / 90));
	for (let j = 0; j < n; j += stride) {
		for (let i = 0; i < n; i += stride) {
			const v = score(i, j);
			if (v > best) {
				best = v;
				bi = i;
				bj = j;
			}
		}
	}
	if (bi < 0) bi = 0;
	if (bj < 0) bj = 0;
	for (let dj = -stride; dj <= stride; dj++) {
		for (let di = -stride; di <= stride; di++) {
			const i = bi + di;
			const j = bj + dj;
			if (i < 0 || j < 0 || i >= n || j >= n) continue;
			const v = score(i, j);
			if (v > best) {
				best = v;
				bi = i;
				bj = j;
			}
		}
	}
	const cx = box.x0 + (bi + 0.5) * cellW;
	const cy = box.y0 + (bj + 0.5) * cellH;
	return {
		ok: best > -tol,
		cx,
		cy,
		a: [cx - dx, cy - dy],
		b: [cx + dx, cy + dy],
		margin: best
	};
}

/** Is the needle at these endpoints fully inside? */
export function needleInside(
	set: { n: number; box: Box; cell: Uint8Array },
	sdf: Float32Array,
	a: [number, number],
	b: [number, number],
	samples = 41,
	tol = 0
): boolean {
	for (let s = 0; s < samples; s++) {
		const u = s / (samples - 1);
		const x = a[0] + (b[0] - a[0]) * u;
		const y = a[1] + (b[1] - a[1]) * u;
		if (sampleDist(sdf, set, x, y) <= -tol) return false;
	}
	return true;
}

export function distanceField(set: CellSet): Float32Array {
	return signedDistance(set);
}
