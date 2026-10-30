// Small canvas helpers for the Kakeya page. Deliberately duplicated from the
// QCD page's util rather than imported: the two pages are unrelated, and the
// QCD helpers are shaped around its own demos.

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

export function clamp(x: number, a: number, b: number) {
	return Math.min(b, Math.max(a, x));
}

export function fmt(x: number, digits = 3) {
	if (!isFinite(x)) return '∞';
	if (Math.abs(x) !== 0 && (Math.abs(x) >= 1e5 || Math.abs(x) < 1e-3)) return x.toExponential(2);
	return x.toFixed(digits);
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

/** A world→screen mapping that keeps the y axis pointing up. */
export interface View {
	toX: (x: number) => number;
	toY: (y: number) => number;
	scale: number;
	cx: number;
	cy: number;
}

export function viewFor(
	w: number,
	h: number,
	box: { x0: number; y0: number; x1: number; y1: number },
	fraction = 0.86
): View {
	const scale = (Math.min(w, h) * fraction) / Math.max(box.x1 - box.x0, box.y1 - box.y0);
	const cx = w / 2 - ((box.x0 + box.x1) / 2) * scale;
	const cy = h / 2 + ((box.y0 + box.y1) / 2) * scale;
	return { toX: (x) => cx + x * scale, toY: (y) => cy - y * scale, scale, cx, cy };
}

export function tracePoly(ctx: CanvasRenderingContext2D, poly: [number, number][], view: View) {
	ctx.moveTo(view.toX(poly[0][0]), view.toY(poly[0][1]));
	for (let i = 1; i < poly.length; i++) ctx.lineTo(view.toX(poly[i][0]), view.toY(poly[i][1]));
	ctx.closePath();
}
