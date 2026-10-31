// The three exponents on the ETH page, and the arithmetic that makes the
// "galactic algorithm" point concrete.
//
// There are three because the paper and its formalisation state different
// bounds, and the difference is exactly the kind of thing this site likes to
// put on screen:
//
//   abstract          n / (log n)^(1/2 + o(1))   ->  1 / sqrt(log n)
//   Theorem 1.1       n log log n / sqrt(log n)  ->  log log n / sqrt(log n)
//   Lean (378.md)     n (log log n)^2 / sqrt(log n)
//
// The third is the only version a machine has checked, and it is weaker than
// the first two by a factor of log log n. `log` is the natural logarithm, as
// the paper says explicitly.

/** Base-2 exponent per variable: the abstract's 1/(log n)^(1/2+o(1)). */
export function alphaAbstract(n: number): number {
	if (n <= 2) return 1;
	return 1 / Math.sqrt(Math.log(n));
}

/** Base-2 exponent per variable in Theorem 1.1: log log n / sqrt(log n). */
export function alphaManuscript(n: number): number {
	const l = Math.log(n);
	if (l <= 1) return 1;
	const ll = Math.log(l);
	if (ll <= 0) return 0;
	return ll / Math.sqrt(l);
}

/** Base-2 exponent per variable in the Lean development: (log log n)^2 / sqrt(log n). */
export function alphaLean(n: number): number {
	const l = Math.log(n);
	if (l <= 1) return 1;
	const ll = Math.log(l);
	if (ll <= 0) return 0;
	return (ll * ll) / Math.sqrt(l);
}

/** log10 of 2^(alpha*n): a time of 2^(alpha*n) reads as 10^(log10Time(alpha,n)). */
export function log10Time(alpha: number, n: number): number {
	return alpha * n * Math.LOG10E * Math.LN2;
}

/** Base-2 exponent of a c^n algorithm, e.g. 1.308 -> 0.387. */
export function exponentOfBase(c: number): number {
	return Math.log(c) / Math.LN2;
}

// The n at which each exponent falls to a target, returned as log10 n, because
// these numbers overflow a double: n = 10^2390 is not representable as a float.

/** 1/sqrt(log n) = t  =>  log n = 1/t^2. */
export function log10NForAbstractExponent(t: number): number {
	return 1 / (t * t * Math.LN10);
}

/**
 * log log n / sqrt(log n) = t. Put s = sqrt(log n): the equation is
 * 2 log s / s = t, which decreases for s > e, so bisect there.
 */
export function log10NForManuscriptExponent(t: number): number {
	const g = (s: number) => (2 * Math.log(s)) / s;
	const peak = g(Math.E);
	if (t >= peak) return Infinity;
	let lo = Math.E;
	let hi = 1e12;
	for (let i = 0; i < 200; i++) {
		const mid = Math.sqrt(lo * hi);
		if (g(mid) > t) lo = mid;
		else hi = mid;
	}
	const s = Math.sqrt(lo * hi);
	return (s * s) / Math.LN10;
}

/**
 * (log log n)^2 / sqrt(log n) = t, solved on x = log n directly so nothing
 * overflows: f(x) = (log x)^2 / sqrt(x). Not monotone — it climbs to a peak and
 * then falls — so find the peak, then bisect above it.
 */
export function log10NForLeanExponent(t: number, xMax = 1e7): number {
	const f = (x: number) => {
		if (x <= 1) return 0;
		const lx = Math.log(x);
		return lx <= 0 ? 0 : (lx * lx) / Math.sqrt(x);
	};
	let peakX = 3;
	let peak = 0;
	for (let x = 3; x < Math.min(xMax, 1e4); x *= 1.05) {
		const v = f(x);
		if (v > peak) {
			peak = v;
			peakX = x;
		}
	}
	if (t >= peak) return Infinity;
	if (f(xMax) > t) return Infinity;
	let lo = peakX;
	let hi = xMax;
	for (let i = 0; i < 200; i++) {
		const mid = Math.sqrt(lo * hi);
		if (f(mid) > t) lo = mid;
		else hi = mid;
	}
	return Math.sqrt(lo * hi) / Math.LN10;
}

const SUP = '⁰¹²³⁴⁵⁶⁷⁸⁹';

/** 2390 -> "²³⁹⁰". */
export function superscript(e: number): string {
	if (e < 0) return '⁻' + superscript(-e);
	return String(e)
		.split('')
		.map((d) => SUP[Number(d)])
		.join('');
}

/** 3.2e17 -> "3.2 × 10¹⁷"; 780 -> "780". */
export function fmtCount(n: number): string {
	if (!isFinite(n)) return '∞';
	if (n < 10000) return Math.round(n).toLocaleString('en-US');
	const e = Math.floor(Math.log10(n));
	const m = n / Math.pow(10, e);
	return `${m.toFixed(m < 10 ? 1 : 0)} × 10${superscript(e)}`;
}

/** A power of ten, written for prose: 2390 -> "10²³⁹⁰". */
export function fmtLog10(l: number): string {
	if (!isFinite(l)) return '∞';
	if (Math.abs(l) < 4) return fmtCount(Math.pow(10, l));
	return `10${superscript(Math.round(l))}`;
}
