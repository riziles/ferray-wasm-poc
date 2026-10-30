// The exponent bookkeeping of the local smoothing theorem, as formulas rather
// than numbers: Theorem 1.1 of the preprint plus the critical exponent that
// Theorem 1.2 is stated at.
//
//   s₊(p) = max(0, (n-1)/2 - n/p),   p_c = 2n/(n-1).
//
// At a fixed time the sharp Sobolev order is (n-1)(1/2 - 1/p); averaging over
// the time interval gains exactly 1/p once p >= p_c, and below p_c it gains
// everything, since s₊ vanishes there.

export const criticalP = (n: number) => (2 * n) / (n - 1);

export const fixedTimeOrder = (n: number, p: number) => (n - 1) * (0.5 - 1 / p);

export const smoothingOrder = (n: number, p: number) => Math.max(0, (n - 1) / 2 - n / p);

/**
 * The gain from averaging in time: at p >= p_c it is 1/p; below p_c the
 * averaging condition is free and the whole fixed-time order is gained.
 */
export const smoothingGain = (n: number, p: number) =>
	fixedTimeOrder(n, p) - smoothingOrder(n, p);

/** Spatial dimension of the light cone: d = n - 1. */
export const coneDim = (n: number) => n - 1;

/**
 * The exponent of the shell contribution in the focusing example: the mass at
 * distance ~u from the focus contributes u^(d - dp/2). At p = p_c this is
 * exactly -1, so every dyadic scale contributes the same amount and the total
 * grows like log(1/u) — the logarithm the theorem cannot remove.
 */
export const shellExponent = (n: number, p: number) => coneDim(n) - (coneDim(n) * p) / 2;

/** Does the shell series converge at this p? (exponent > -1 means summable) */
export const shellSummable = (n: number, p: number) => shellExponent(n, p) > -1;

export function fmtP(x: number, digits = 3) {
	return x.toFixed(digits);
}
