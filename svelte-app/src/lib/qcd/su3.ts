// Minimal complex 3x3 linear algebra, enough for the colour-charge demo:
// random SU(3) elements (via the matrix exponential of a Hermitian generator),
// application to colour 3-vectors, and the colour-singlet invariant.

export interface Cx {
	re: number;
	im: number;
}
export interface M3 {
	re: number[][];
	im: number[][];
}
export type Vec3 = { re: number[]; im: number[] };

export const cx = (re: number, im = 0): Cx => ({ re, im });
export const cAdd = (a: Cx, b: Cx): Cx => ({ re: a.re + b.re, im: a.im + b.im });
export const cMul = (a: Cx, b: Cx): Cx => ({
	re: a.re * b.re - a.im * b.im,
	im: a.re * b.im + a.im * b.re
});
export const cAbs2 = (a: Cx) => a.re * a.re + a.im * a.im;

export function ident(): M3 {
	return {
		re: [
			[1, 0, 0],
			[0, 1, 0],
			[0, 0, 1]
		],
		im: [
			[0, 0, 0],
			[0, 0, 0],
			[0, 0, 0]
		]
	};
}

export function mMul(A: M3, B: M3): M3 {
	const re: number[][] = [];
	const im: number[][] = [];
	for (let i = 0; i < 3; i++) {
		re.push([]);
		im.push([]);
		for (let j = 0; j < 3; j++) {
			let r = 0;
			let m = 0;
			for (let k = 0; k < 3; k++) {
				const p = cMul(cx(A.re[i][k], A.im[i][k]), cx(B.re[k][j], B.im[k][j]));
				r += p.re;
				m += p.im;
			}
			re[i].push(r);
			im[i].push(m);
		}
	}
	return { re, im };
}

export function mScale(A: M3, s: number): M3 {
	return {
		re: A.re.map((row) => row.map((x) => x * s)),
		im: A.im.map((row) => row.map((x) => x * s))
	};
}

export function mAdd(A: M3, B: M3): M3 {
	return {
		re: A.re.map((row, i) => row.map((x, j) => x + B.re[i][j])),
		im: A.im.map((row, i) => row.map((x, j) => x + B.im[i][j]))
	};
}

/** exp(M) by scaling-and-squaring plus a short Taylor series (M is tiny after scaling). */
export function mExp(M: M3, terms = 10): M3 {
	// rescale so the norm is small
	let maxAbs = 0;
	for (let i = 0; i < 3; i++) for (let j = 0; j < 3; j++) maxAbs = Math.max(maxAbs, Math.abs(M.re[i][j]), Math.abs(M.im[i][j]));
	let squarings = 0;
	while (maxAbs > 0.25) {
		maxAbs /= 2;
		squarings++;
	}
	const S = mScale(M, 1 / Math.pow(2, squarings));

	let sum = ident();
	let term = ident();
	for (let n = 1; n <= terms; n++) {
		term = mScale(mMul(term, S), 1 / n);
		sum = mAdd(sum, term);
	}
	for (let i = 0; i < squarings; i++) sum = mMul(sum, sum);
	return sum;
}

/**
 * A random traceless Hermitian 3x3 matrix, Frobenius-normalised to ||H||_F = 1.
 * These are the generators of SU(3) — 8 real parameters, i.e. the 8 gluons.
 */
export function randomGenerator(): M3 {
	// H = sum_a c_a lambda_a / 2, lambda = Gell-Mann matrices
	const g = [
		[
			[0, 1, 0],
			[1, 0, 0],
			[0, 0, 0]
		],
		[
			[0, -1, 0],
			[1, 0, 0],
			[0, 0, 0]
		],
		[
			[1, 0, 0],
			[0, -1, 0],
			[0, 0, 0]
		],
		[
			[0, 0, 1],
			[0, 0, 0],
			[1, 0, 0]
		],
		[
			[0, 0, -1],
			[0, 0, 0],
			[1, 0, 0]
		],
		[
			[0, 0, 0],
			[0, 0, 1],
			[0, 1, 0]
		],
		[
			[0, 0, 0],
			[0, 0, -1],
			[0, 1, 0]
		],
		[
			[1, 0, 0],
			[0, 1, 0],
			[0, 0, -2]
		]
	];
	// imaginary parts for the antisymmetric Gell-Mann matrices (2,5,7)
	const anti = [false, true, false, false, true, false, true, false];

	const re = [
		[0, 0, 0],
		[0, 0, 0],
		[0, 0, 0]
	];
	const im = [
		[0, 0, 0],
		[0, 0, 0],
		[0, 0, 0]
	];
	for (let a = 0; a < 8; a++) {
		const c = Math.random() * 2 - 1;
		for (let i = 0; i < 3; i++) {
			for (let j = 0; j < 3; j++) {
				const v = (g[a][i][j] * c) / 2;
				if (anti[a]) im[i][j] += v;
				else re[i][j] += v;
			}
		}
	}
	// normalise
	let n = 0;
	for (let i = 0; i < 3; i++) for (let j = 0; j < 3; j++) n += re[i][j] * re[i][j] + im[i][j] * im[i][j];
	n = Math.sqrt(n) || 1;
	return mScale({ re, im }, 1 / n);
}

/** U(t) = exp(i t H)  — a one-parameter family of SU(3) elements. */
export function su3At(H: M3, t: number): M3 {
	// i*H  =>  re = -im(H), im = re(H)
	return mExp(
		{
			re: H.im.map((row) => row.map((x) => -x * t)),
			im: H.re.map((row) => row.map((x) => x * t))
		},
		12
	);
}

export function applyU(U: M3, v: Vec3): Vec3 {
	const re: number[] = [];
	const im: number[] = [];
	for (let i = 0; i < 3; i++) {
		let r = 0;
		let m = 0;
		for (let k = 0; k < 3; k++) {
			const p = cMul(cx(U.re[i][k], U.im[i][k]), cx(v.re[k], v.im[k]));
			r += p.re;
			m += p.im;
		}
		re.push(r);
		im.push(m);
	}
	return { re, im };
}

/** det of a complex 3x3 */
export function det(M: M3): Cx {
	const a = (i: number, j: number) => cx(M.re[i][j], M.im[i][j]);
	const t1 = cMul(a(0, 0), cAdd(cMul(a(1, 1), a(2, 2)), cMul(cx(-1), cMul(a(1, 2), a(2, 1)))));
	const t2 = cMul(a(0, 1), cAdd(cMul(a(1, 0), a(2, 2)), cMul(cx(-1), cMul(a(1, 2), a(2, 0)))));
	const t3 = cMul(a(0, 2), cAdd(cMul(a(1, 0), a(2, 1)), cMul(cx(-1), cMul(a(1, 1), a(2, 0)))));
	return cAdd(cAdd(t1, cMul(cx(-1), t2)), t3);
}

/**
 * The only physically meaningful number in the colour-charge demo:
 * |eps_ijk v1^i v2^j v3^k|^2, i.e. |det[v1 v2 v3]|^2, the baryon colour singlet.
 */
export function singletWeight(v1: Vec3, v2: Vec3, v3: Vec3): number {
	const M: M3 = {
		re: [
			[0, 1, 2].map((i) => v1.re[i]),
			[0, 1, 2].map((i) => v2.re[i]),
			[0, 1, 2].map((i) => v3.re[i])
		],
		im: [
			[0, 1, 2].map((i) => v1.im[i]),
			[0, 1, 2].map((i) => v2.im[i]),
			[0, 1, 2].map((i) => v3.im[i])
		]
	};
	const d = det(M);
	return cAbs2(d);
}

/** Start from the canonical r,g,b basis. */
export function canonicalQuarks(): Vec3[] {
	return [
		{ re: [1, 0, 0], im: [0, 0, 0] },
		{ re: [0, 1, 0], im: [0, 0, 0] },
		{ re: [0, 0, 1], im: [0, 0, 0] }
	];
}
