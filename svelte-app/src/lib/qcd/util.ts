// Small shared helpers for the QCD page demos. No WASM, no dependencies.

export function randn(): number {
	let u = 0;
	let v = 0;
	while (u === 0) u = Math.random();
	while (v === 0) v = Math.random();
	return Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * v);
}

/** Deterministic PRNG so animations don't flicker between frames. */
export function mulberry32(seed: number): () => number {
	let a = seed >>> 0;
	return function () {
		a = (a + 0x6d2b79f5) >>> 0;
		let t = Math.imul(a ^ (a >>> 15), 1 | a);
		t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
		return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
	};
}

export interface Fitted {
	ctx: CanvasRenderingContext2D;
	w: number;
	h: number;
}

/**
 * Resize the backing store of a canvas to its CSS box at device pixel ratio,
 * and return a context whose coordinate system is CSS pixels.
 */
export function fit(canvas: HTMLCanvasElement): Fitted {
	const dpr = Math.min(window.devicePixelRatio || 1, 2);
	const w = Math.max(1, canvas.clientWidth);
	const h = Math.max(1, canvas.clientHeight);
	const bw = Math.round(w * dpr);
	const bh = Math.round(h * dpr);
	if (canvas.width !== bw || canvas.height !== bh) {
		canvas.width = bw;
		canvas.height = bh;
	}
	const ctx = canvas.getContext('2d')!;
	ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
	return { ctx, w, h };
}

export function clear(ctx: CanvasRenderingContext2D, w: number, h: number, bg = '#0b1020') {
	ctx.fillStyle = bg;
	ctx.fillRect(0, 0, w, h);
}

/** r,g,b weights -> css colour, normalised. */
export function mixCss(weights: number[], gamma = 0.75): string {
	const s = weights[0] + weights[1] + weights[2] || 1;
	const ch = weights.map((x) => Math.round(255 * Math.pow(Math.max(0, x) / s, gamma)));
	return `rgb(${ch[0]},${ch[1]},${ch[2]})`;
}

export const QUARK_COLOURS = ['#f87171', '#4ade80', '#60a5fa'];
export const QUARK_NAMES = ['red', 'green', 'blue'];
export const ANTI_COLOURS = ['#fca5a5', '#86efac', '#93c5fd'];

export function clamp(x: number, a: number, b: number) {
	return Math.min(b, Math.max(a, x));
}

export function lerp(a: number, b: number, t: number) {
	return a + (b - a) * t;
}

export function fmt(x: number, digits = 2) {
	if (!isFinite(x)) return '∞';
	if (Math.abs(x) >= 1e5 || (Math.abs(x) < 1e-3 && x !== 0)) return x.toExponential(1);
	return x.toFixed(digits);
}
