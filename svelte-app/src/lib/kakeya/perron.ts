// Perron's 1928 construction (the "Perron tree"), in the compact form used by
// Terence Tao's 1998 applet: divide the base of a right triangle of height 1
// into 2^n pieces, keep every piece's apex at height 1, and slide the pieces
// horizontally by a geometric factor alpha. The union keeps a unit segment
// through the apex of every piece — one direction per piece — while the pieces
// overlap more and more.
//
// Conventions: the root triangle has its apex at the origin, its base on the
// line y = -1 and its base from x = 0 to x = 1. Directions are measured from
// straight down (0 rad = -y) toward +x, so a piece covering the base interval
// [k/2^n, (k+1)/2^n] covers exactly the directions atan(k/2^n) .. atan((k+1)/2^n).
// Eight rotated copies, 45° apart, therefore cover the whole circle exactly.

import { rotate, type Poly, type Pt } from './geom';

export interface Leaf {
	/** Apex, in world coordinates. */
	apex: Pt;
	/** Left and right base corners, at y = -1. */
	bl: Pt;
	br: Pt;
	/** Angular interval of directions this piece holds a unit needle for. */
	t0: number;
	t1: number;
}

export const COPIES = 8;
export const COPY_ANGLE = Math.PI / 4;

/**
 * Horizontal offset of piece k's apex, following Tao's index formula.
 * `n2 = 2^n`, `alpha` is the slide factor per level.
 */
function apexOffset(k: number, n: number, alpha: number): number {
	const n2 = 1 << n;
	let x = 0;
	let m = k;
	let j2 = n2 / 2;
	let aj = 1;
	for (let j = 0; j < n; j++) {
		x += (aj / j2) * Math.floor((m + 1) / 2);
		aj *= alpha;
		j2 = Math.floor(j2 / 2);
		m = Math.floor(m / 2);
	}
	return x * (alpha - 1) + 1 - Math.pow(alpha, n);
}

/** The 2^n leaves of the Perron tree. */
export function perronLeaves(n: number, alpha: number): Leaf[] {
	const n2 = 1 << n;
	const out: Leaf[] = [];
	for (let k = 0; k < n2; k++) {
		const x = apexOffset(k, n, alpha);
		out.push({
			apex: [x, 0],
			bl: [x + k / n2, -1],
			br: [x + (k + 1) / n2, -1],
			t0: Math.atan(k / n2),
			t1: Math.atan((k + 1) / n2)
		});
	}
	return out;
}

export function leafPoly(l: Leaf): Poly {
	return [l.apex, l.bl, l.br];
}

/** Leaf triangles for one copy. */
export function treePolys(n: number, alpha: number): Poly[] {
	return perronLeaves(n, alpha).map(leafPoly);
}

/** Leaf triangles for all eight rotated copies. */
export function fanPolys(n: number, alpha: number): Poly[] {
	const base = treePolys(n, alpha);
	const out: Poly[] = [];
	for (let c = 0; c < COPIES; c++) {
		const a = c * COPY_ANGLE;
		for (const poly of base) out.push(poly.map((p) => rotate(p, a)));
	}
	return out;
}

/**
 * The area of the tree as a fraction of the root triangle. This is Perron's
 * two competing terms: `1 - alpha` is the thickness each piece keeps, and
 * `alpha^(2n)` is the residue of the original fan. Minimising over alpha for a
 * fixed n gives the optimum below — and shows why the area only falls like
 * log(n)/n, so that an area of epsilon needs roughly 1/epsilon levels.
 */
export function areaFraction(n: number, alpha: number): number {
	return 1 - alpha + Math.pow(alpha, 2 * n);
}

export function optimalAlpha(n: number): number {
	// d/da [1 - a + a^(2n)] = 0  =>  a^(2n-1) = 1/(2n)
	if (n <= 1) return 0.5;
	return Math.pow(2 * n, -1 / (2 * n - 1));
}

/** Evenly spaced alphas, for the "best area at this depth" line. */
export function bestArea(n: number): number {
	return areaFraction(n, optimalAlpha(n));
}

export interface Witness {
	/** Unit needle that lies inside the tree, as a segment. */
	a: Pt;
	b: Pt;
	/** Which copy (0..7) and which leaf it came from. */
	copy: number;
	leaf: number;
	/** The exact direction the tree holds, within one piece-width of the request. */
	aim: number;
}

/**
 * A unit needle in the given world direction, sitting inside the tree. Every
 * leaf holds one from its own apex for the directions in its angular interval,
 * so this is a proof-by-construction rather than a search.
 *
 * `angle` is measured in world coordinates: 0 points straight down (-y),
 * increasing toward +x, and the figure is symmetric under 45° rotations.
 */
export function witness(angle: number, n: number, alpha: number): Witness {
	const twoPi = Math.PI * 2;
	const norm = ((angle % twoPi) + twoPi) % twoPi;
	const copy = Math.min(COPIES - 1, Math.floor(norm / COPY_ANGLE));
	const local = norm - copy * COPY_ANGLE;
	const n2 = 1 << n;
	// Invert atan(k/n2) <= local < atan((k+1)/n2), then aim at the *middle* of
	// the piece: a needle aimed exactly at a piece's edge sits on the boundary,
	// which is not where the construction puts it.
	const leaf = Math.min(n2 - 1, Math.max(0, Math.floor(Math.tan(local) * n2)));
	const aim = Math.atan((leaf + 0.5) / n2);
	const l = perronLeaves(n, alpha)[leaf];
	const a = rotate(l.apex, copy * COPY_ANGLE);
	// Unit vector for the requested direction: down is (0,-1), +x is (1,0).
	const dir = rotate([Math.sin(aim), -Math.cos(aim)], copy * COPY_ANGLE);
	const b: Pt = [a[0] + dir[0], a[1] + dir[1]];
	return { a, b, copy, leaf, aim: copy * COPY_ANGLE + aim };
}

/** Do the eight copies cover every direction? True by construction, kept as a check. */
export function coverageGap(n: number): number {
	const leaves = perronLeaves(n, 1);
	let lo = Infinity;
	let hi = -Infinity;
	for (const l of leaves) {
		lo = Math.min(lo, l.t0);
		hi = Math.max(hi, l.t1);
	}
	return COPY_ANGLE - (hi - lo);
}
