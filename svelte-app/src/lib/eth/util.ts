// Small canvas helpers for the ETH page. Deliberately duplicated from the
// Kakeya page's util rather than shared: the two pages are unrelated, and the
// Kakeya helpers are shaped around its own demos.

export interface Fitted {
	ctx: CanvasRenderingContext2D;
	w: number;
	h: number;
}

/** Size the backing store to the CSS box at device pixel ratio, in CSS pixels. */
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

export const INK = {
	dim: '#334155',
	line: '#475569',
	text: '#94a3b8',
	bright: '#e2e8f0',
	primary: '#38bdf8',
	secondary: '#c084fc',
	warn: '#fbbf24',
	bad: '#f87171',
	good: '#4ade80'
};
