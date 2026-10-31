// The recovery step (Section 2.5), in exact arithmetic and small enough to check.
//
// The algorithm never sees the count directly. What it can compute are the
// Taylor coefficients of H(T) = Σ_x Π_e Φ_e(replacements at T) at the points
// t = 1, …, Q — because that is where the replacement forms are designed to be
// summable. The count is H(0), one point away from all of them.
//
// The paper's argument is that Q points × J coefficients is QJ numbers, and a
// polynomial of degree < QJ is pinned down by them, so H(0) is determined. In
// fact what is needed is only that the functional "value at 0" lies in the span
// of the sampling functionals: then there are weights λ, independent of H, with
//
//     H(0) = Σ_{t,j} λ_{t,j} · [z^j] H(t + z).
//
// That is the linear system the paper solves, and the weights are what the
// algorithm would use. This module computes λ exactly over the rationals, and
// produces the counterexample when QJ ≤ degree.

/** A rational number, kept in lowest terms with a positive denominator. */
export interface Frac {
	n: bigint;
	d: bigint;
}

function abs(x: bigint): bigint {
	return x < 0n ? -x : x;
}

export function gcd(a: bigint, b: bigint): bigint {
	a = abs(a);
	b = abs(b);
	while (b !== 0n) {
		const t = a % b;
		a = b;
		b = t;
	}
	return a;
}

export function frac(n: bigint | number, d: bigint | number = 1): Frac {
	let nn = BigInt(n);
	let dd = BigInt(d);
	if (dd === 0n) throw new Error('zero denominator');
	if (dd < 0n) {
		nn = -nn;
		dd = -dd;
	}
	const g = gcd(nn, dd) || 1n;
	return { n: nn / g, d: dd / g };
}

export const ZERO = frac(0);
export const ONE = frac(1);

const isZero = (a: Frac) => a.n === 0n;
const add = (a: Frac, b: Frac) => frac(a.n * b.d + b.n * a.d, a.d * b.d);
const sub = (a: Frac, b: Frac) => frac(a.n * b.d - b.n * a.d, a.d * b.d);
const mul = (a: Frac, b: Frac) => frac(a.n * b.n, a.d * b.d);
const div = (a: Frac, b: Frac) => frac(a.n * b.d, a.d * b.n);

export function fracToNumber(a: Frac): number {
	return Number(a.n) / Number(a.d);
}

/** "3/8", or "−3/8"; integers print bare. */
export function fracToText(a: Frac): string {
	if (a.d === 1n) return a.n.toString();
	return `${a.n}/${a.d}`;
}

/** Exact binomial coefficient. */
export function binom(k: number, j: number): bigint {
	if (j < 0 || j > k) return 0n;
	let r = 1n;
	for (let i = 1; i <= j; i++) r = (r * BigInt(k - j + i)) / BigInt(i);
	return r;
}

/**
 * The constraint matrix M: rows are the sampling functionals (t, j), columns are
 * the coefficients of a polynomial of degree ≤ d. The datum [z^j] T^k(t + z) is
 * C(k, j)·t^(k−j).
 */
export function constraintMatrix(d: number, Q: number, J: number): Frac[][] {
	const rows: Frac[][] = [];
	for (let t = 1; t <= Q; t++) {
		for (let j = 0; j < J; j++) {
			const row: Frac[] = [];
			for (let k = 0; k <= d; k++) {
				row.push(k < j ? ZERO : frac(binom(k, j) * BigInt(t) ** BigInt(k - j)));
			}
			rows.push(row);
		}
	}
	return rows;
}

/** The data the algorithm would see, for a polynomial given by its coefficients. */
export function dataOf(coeffs: bigint[], Q: number, J: number): bigint[] {
	const out: bigint[] = [];
	for (let t = 1; t <= Q; t++) {
		for (let j = 0; j < J; j++) {
			let v = 0n;
			for (let k = j; k < coeffs.length; k++) {
				v += coeffs[k] * binom(k, j) * BigInt(t) ** BigInt(k - j);
			}
			out.push(v);
		}
	}
	return out;
}

/** Gauss–Jordan over the rationals; returns the reduced matrix and the pivot columns. */
function rref(rows: Frac[][]): { m: Frac[][]; pivots: number[] } {
	const m = rows.map((r) => r.slice());
	const pivots: number[] = [];
	let row = 0;
	const cols = m[0]?.length ?? 0;
	for (let col = 0; col < cols && row < m.length; col++) {
		let sel = -1;
		for (let r = row; r < m.length; r++) {
			if (!isZero(m[r][col])) {
				sel = r;
				break;
			}
		}
		if (sel < 0) continue;
		[m[row], m[sel]] = [m[sel], m[row]];
		const p = m[row][col];
		m[row] = m[row].map((v) => div(v, p));
		for (let r = 0; r < m.length; r++) {
			if (r === row || isZero(m[r][col])) continue;
			const f = m[r][col];
			m[r] = m[r].map((v, i) => sub(v, mul(f, m[row][i])));
		}
		pivots.push(col);
		row++;
	}
	return { m, pivots };
}

export interface Weights {
	/** True when H(0) is a fixed linear combination of the data. */
	determined: boolean;
	/** One such combination, when it exists: H(0) = Σ λ_i · datum_i. */
	lambda: Frac[] | null;
	/** QJ, the number of numbers the algorithm computes. */
	dataCount: number;
	/** d + 1, the number of coefficients to pin down. */
	unknowns: number;
}

/**
 * Solve Mᵀλ = e₀ for λ, i.e. find the fixed weights that turn the sampled Taylor
 * coefficients into the value at 0. Under-determined systems get the particular
 * solution with the free weights set to zero.
 */
export function recoveryWeights(d: number, Q: number, J: number): Weights {
	const M = constraintMatrix(d, Q, J);
	// Mᵀλ = e₀: one equation per coefficient, one unknown per datum.
	const rows: Frac[][] = [];
	for (let k = 0; k <= d; k++) {
		const row: Frac[] = [];
		for (let r = 0; r < M.length; r++) row.push(M[r][k]);
		row.push(k === 0 ? ONE : ZERO);
		rows.push(row);
	}
	const { m, pivots } = rref(rows);
	// Inconsistent when a row is all zeros on the left and nonzero on the right.
	for (const r of m) {
		const zeroLeft = r.slice(0, M.length).every(isZero);
		if (zeroLeft && !isZero(r[M.length])) {
			return { determined: false, lambda: null, dataCount: Q * J, unknowns: d + 1 };
		}
	}
	const lambda: Frac[] = new Array(M.length).fill(ZERO);
	for (let i = 0; i < pivots.length; i++) {
		const col = pivots[i];
		if (col < M.length) lambda[col] = m[i][M.length];
	}
	return { determined: true, lambda, dataCount: Q * J, unknowns: d + 1 };
}

/** Σ λ_i b_i, exactly. */
export function applyWeights(lambda: Frac[], data: bigint[]): Frac {
	let acc = ZERO;
	for (let i = 0; i < lambda.length && i < data.length; i++) {
		acc = add(acc, mul(lambda[i], frac(data[i])));
	}
	return acc;
}

/** Multiply two polynomials given by their coefficient vectors (low → high). */
export function polyMul(a: bigint[], b: bigint[]): bigint[] {
	const out = new Array(a.length + b.length - 1).fill(0n);
	for (let i = 0; i < a.length; i++) {
		for (let j = 0; j < b.length; j++) out[i + j] += a[i] * b[j];
	}
	return out;
}

/**
 * The witness that recovery fails when QJ ≤ d: Π_{t=1..Q} (T − t)^J.
 * It vanishes, with all J−1 derivatives, at every sample point — so H and
 * H + w produce identical data — and its value at 0 is Π(−t)^J ≠ 0, so their
 * counts differ.
 */
export function vanishingPolynomial(Q: number, J: number): bigint[] {
	let p = [1n];
	for (let t = 1; t <= Q; t++) {
		for (let j = 0; j < J; j++) p = polyMul(p, [-BigInt(t), 1n]);
	}
	return p;
}

/** The rational value of Π(−t)^J, i.e. w(0). */
export function vanishingValue(Q: number, J: number): bigint {
	let v = 1n;
	for (let t = 1; t <= Q; t++) v *= BigInt(-t) ** BigInt(J);
	return v;
}

/** Evaluate a polynomial with integer coefficients at a real point. */
export function evalPoly(coeffs: bigint[], x: number): number {
	let acc = 0;
	for (let k = coeffs.length - 1; k >= 0; k--) acc = acc * x + Number(coeffs[k]);
	return acc;
}

function mulberry32(seed: number) {
	return function next() {
		seed |= 0;
		seed = (seed + 0x6d2b79f5) | 0;
		let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
		t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
		return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
	};
}

/** A pseudo-random polynomial of degree d with small integer coefficients, seeded. */
export function randomPoly(d: number, seed: number): bigint[] {
	const rnd = mulberry32(seed);
	return Array.from({ length: d + 1 }, () => BigInt(Math.floor(rnd() * 11) - 5));
}
