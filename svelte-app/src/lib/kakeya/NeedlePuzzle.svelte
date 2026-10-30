<script lang="ts">
// The 1917 question, as a toy you can push around: find, for each direction, a
// place to put a unit needle inside the shape. Two shapes succeed, two fail —
// and the one that fails hardest is not the smallest.

import { onMount } from 'svelte';
import { diskPoly, equilateralPoly, rectPoly, rasterize, boxOf, padBox, cellSize, squareBox } from './geom';
import { deepestPlacement, distanceField, needleInside } from './figure';
import { INK, clear, clamp, fit, fmt, tracePoly, viewFor } from './util';
import type { Box, Poly } from './geom';
import type { Placement } from './figure';
import type { View } from './util';

interface Region {
	id: string;
	label: string;
	polys: Poly[];
	/** Exact area, when it has a closed form. */
	area: number;
	areaNote: string;
	works: boolean;
	note: string;
	box: Box;
}

function sliver(): Poly {
	const hw = Math.tan((10 * Math.PI) / 180);
	return [
		[0, 0.5],
		[hw, -0.5],
		[-hw, -0.5]
	];
}

const REGIONS: Region[] = [
	{
		id: 'disk',
		label: 'Disk, radius ½',
		polys: [diskPoly(0.5)],
		area: Math.PI / 4,
		areaNote: 'π/4',
		works: true,
		note: 'A needle pivoting about its centre sweeps exactly this disk. Nothing to discover.',
		box: { x0: -0.55, y0: -0.55, x1: 0.55, y1: 0.55 }
	},
	{
		id: 'triangle',
		label: 'Equilateral triangle, height 1',
		polys: [equilateralPoly()],
		area: 1 / Math.sqrt(3),
		areaNote: '1/√3',
		works: true,
		note: 'Pál (1921): the smallest CONVEX room. Every width of this triangle is at least 1, so every direction fits — but you have to translate as you turn.',
		box: { x0: -0.62, y0: -0.55, x1: 0.62, y1: 0.58 }
	},
	{
		id: 'rect',
		label: 'Rectangle, 1 × ½',
		polys: [rectPoly(0, 0, 1, 0.5, 0)],
		area: 0.5,
		areaNote: '0.500',
		works: false,
		note: 'Smaller than the triangle — and useless: its width across the short axis is ½, so a unit needle simply cannot point that way.',
		box: { x0: -0.58, y0: -0.32, x1: 0.58, y1: 0.32 }
	},
	{
		id: 'sliver',
		label: 'Sliver, apex angle 20°',
		polys: [sliver()],
		area: Math.tan((10 * Math.PI) / 180),
		areaNote: 'tan 10°',
		works: false,
		note: 'Even smaller, even more useless. Area was never the constraint — directions are.',
		box: { x0: -0.26, y0: -0.55, x1: 0.26, y1: 0.55 }
	}
];

const BUCKETS = 72;

let regionId = $state('triangle');
let angle = $state(0); // radians, needle direction
let centre = $state<[number, number]>([0, 0]);
let magnet = $state(true);
let spin = $state(false);
let found = $state<boolean[]>(new Array(BUCKETS).fill(false));
let baked = $state<{ view: View; box: Box } | null>(null);

let canvas: HTMLCanvasElement;
let raf = 0;

// Per-region raster + distance field, built lazily and cached.
const cache = new Map<string, { set: ReturnType<typeof rasterize>; sdf: Float32Array; box: Box }>();
const placements = new Map<string, Placement>();

function region(): Region {
	return REGIONS.find((r) => r.id === regionId) ?? REGIONS[0];
}

function prepared() {
	const r = region();
	let hit = cache.get(r.id);
	if (!hit) {
		const box = squareBox(padBox(boxOf(r.polys), 0.02));
		const set = rasterize(r.polys, 640, box);
		hit = { set, sdf: distanceField(set), box };
		cache.set(r.id, hit);
	}
	return hit;
}

function aimed(): [number, number] {
	const p = prepared();
	if (magnet) {
		const bucket = Math.round((angle / (Math.PI * 2)) * BUCKETS) % BUCKETS;
		const key = `${regionId}:${bucket}:${p.set.n}`;
		let hit = placements.get(key);
		if (!hit) {
			hit = deepestPlacement(p.set, p.sdf, angle, 1, 25, cellSize(p.set) * 8);
			placements.set(key, hit);
		}
		return hit.ok ? [hit.cx, hit.cy] : [centre[0], centre[1]];
	}
	return [centre[0], centre[1]];
}

function endpoints(): [[number, number], [number, number]] {
	const [cx, cy] = aimed();
	const dx = (Math.cos(angle) * 1) / 2;
	const dy = (Math.sin(angle) * 1) / 2;
	return [
		[cx - dx, cy - dy],
		[cx + dx, cy + dy]
	];
}

function legal(): boolean {
	const p = prepared();
	const [a, b] = endpoints();
	return needleInside(p.set, p.sdf, a, b, 41, cellSize(p.set) * 8);
}

function markFound() {
	const bucket = Math.round((angle / (Math.PI * 2)) * BUCKETS) % BUCKETS;
	if (!found[bucket] && legal()) {
		const next = found.slice();
		next[(bucket + BUCKETS) % BUCKETS] = true;
		found = next;
	}
}

const count = $derived(found.filter(Boolean).length);
const all = $derived(count === BUCKETS);

function setAngle(a: number) {
	angle = ((a % (Math.PI * 2)) + Math.PI * 2) % (Math.PI * 2);
	const p = prepared();
	if (magnet) {
		const f = aimed();
		centre = f;
	} else {
		centre = [clamp(centre[0], p.box.x0, p.box.x1), clamp(centre[1], p.box.y0, p.box.y1)];
	}
	markFound();
}

function pick(id: string) {
	regionId = id;
	found = new Array(BUCKETS).fill(false);
	placements.clear();
	centre = [0, 0];
	setAngle(angle);
}

function draw() {
	if (!canvas) return;
	const { ctx, w, h } = fit(canvas);
	clear(ctx, w, h);
	const r = region();
	const view = viewFor(w, h, r.box, 0.84);
	baked = { view, box: r.box };

	// region
	ctx.beginPath();
	for (const poly of r.polys) tracePoly(ctx, poly, view);
	ctx.fillStyle = all ? 'rgba(74,222,128,0.16)' : 'rgba(56,189,248,0.10)';
	ctx.fill();
	ctx.strokeStyle = all ? INK.good : 'rgba(56,189,248,0.55)';
	ctx.lineWidth = 1.5;
	ctx.stroke();

	// needle
	const [a, b] = endpoints();
	const ok = needleInside(prepared().set, prepared().sdf, a, b, 41, cellSize(prepared().set) * 8);
	ctx.beginPath();
	ctx.moveTo(view.toX(a[0]), view.toY(a[1]));
	ctx.lineTo(view.toX(b[0]), view.toY(b[1]));
	ctx.lineCap = 'round';
	ctx.strokeStyle = ok ? INK.good : INK.bad;
	ctx.lineWidth = 4;
	ctx.shadowColor = ok ? INK.good : INK.bad;
	ctx.shadowBlur = 12;
	ctx.stroke();
	ctx.shadowBlur = 0;

	// endpoints
	for (const p of [a, b]) {
		ctx.beginPath();
		ctx.arc(view.toX(p[0]), view.toY(p[1]), 3.5, 0, Math.PI * 2);
		ctx.fillStyle = ok ? '#0b1020' : INK.bad;
		ctx.fill();
		ctx.strokeStyle = ok ? INK.good : INK.bad;
		ctx.lineWidth = 1.5;
		ctx.stroke();
	}

	// the direction dial: 72 ticks, lit when a legal placement was found.
	// On narrow canvases it sits in the top-right corner, clear of the shape.
	const compact = w < 560;
	const cx = w - (compact ? 38 : 54);
	const cy = compact ? 42 : h - 54;
	const rr = compact ? 24 : 34;
	for (let i = 0; i < BUCKETS; i++) {
		const th = (i / BUCKETS) * Math.PI * 2;
		const inner = rr - (found[i] ? 12 : 7);
		ctx.beginPath();
		ctx.moveTo(cx + Math.cos(th) * inner, cy - Math.sin(th) * inner);
		ctx.lineTo(cx + Math.cos(th) * rr, cy - Math.sin(th) * rr);
		ctx.strokeStyle = found[i] ? INK.good : INK.dim;
		ctx.lineWidth = found[i] ? 2.4 : 1.4;
		ctx.stroke();
	}
	// current direction
	ctx.beginPath();
	ctx.moveTo(cx, cy);
	ctx.lineTo(cx + Math.cos(angle) * (rr + 6), cy - Math.sin(angle) * (rr + 6));
	ctx.strokeStyle = ok ? INK.primary : INK.bad;
	ctx.lineWidth = 2;
	ctx.stroke();
	ctx.beginPath();
	ctx.arc(cx, cy, 2.5, 0, Math.PI * 2);
	ctx.fillStyle = INK.text;
	ctx.fill();

	ctx.fillStyle = INK.text;
	ctx.font = '10px ui-monospace, monospace';
	ctx.textAlign = 'center';
	ctx.fillText(`${count}/${BUCKETS}`, cx, cy - rr - 5);
	ctx.textAlign = 'left';
}

function loop() {
	if (spin) {
		setAngle(angle + 0.012);
	}
	draw();
	raf = requestAnimationFrame(loop);
}

function toWorld(ev: PointerEvent): [number, number] | null {
	if (!baked || !canvas) return null;
	const rect = canvas.getBoundingClientRect();
	const px = ev.clientX - rect.left;
	const py = ev.clientY - rect.top;
	const { view } = baked;
	return [(px - view.cx) / view.scale, (view.cy - py) / view.scale];
}

let dragging = false;

function onDown(ev: PointerEvent) {
	const p = toWorld(ev);
	if (!p) return;
	dragging = true;
	magnet = false;
	canvas.setPointerCapture(ev.pointerId);
	centre = p;
	markFound();
}

function onMove(ev: PointerEvent) {
	if (!dragging) return;
	const p = toWorld(ev);
	if (!p) return;
	const b = prepared().box;
	centre = [clamp(p[0], b.x0, b.x1), clamp(p[1], b.y0, b.y1)];
	markFound();
}

function onUp() {
	dragging = false;
}

onMount(() => {
	setAngle(angle);
	loop();
	return () => cancelAnimationFrame(raf);
});

$effect(() => {
	// recompute legality whenever the inputs change without a pointer event
	regionId;
	angle;
	magnet;
	legal();
});
</script>

<div class="rounded-lg border border-surface-700 bg-surface-950/40 overflow-hidden">
	<div class="flex flex-wrap items-center gap-2 p-3 border-b border-surface-800">
		{#each REGIONS as r}
			<button
				class="badge {regionId === r.id ? 'preset-filled-primary-500' : 'preset-tonal'}"
				onclick={() => pick(r.id)}
			>
				{r.label}
			</button>
		{/each}
	</div>

	<div class="relative">
		<canvas
			bind:this={canvas}
			class="block w-full h-[360px] touch-none cursor-crosshair"
			onpointerdown={onDown}
			onpointermove={onMove}
			onpointerup={onUp}
			onpointercancel={onUp}
		></canvas>
	</div>

	<div class="p-3 border-t border-surface-800 space-y-3">
		<div class="flex flex-wrap items-center gap-3 text-xs">
			<label class="flex items-center gap-2 flex-1 min-w-[220px]">
				<span class="text-surface-400 whitespace-nowrap">needle direction</span>
				<input
					type="range"
					min="0"
					max="360"
					step="1"
					value={Math.round((angle * 180) / Math.PI)}
					oninput={(e) => setAngle((Number(e.currentTarget.value) * Math.PI) / 180)}
					class="flex-1"
				/>
				<span class="font-mono w-10 text-right">{Math.round((angle * 180) / Math.PI)}°</span>
			</label>
			<button class="badge {magnet ? 'preset-filled-primary-500' : 'preset-tonal'}" onclick={() => { magnet = !magnet; setAngle(angle); }}>
				magnet {magnet ? 'on' : 'off'}
			</button>
			<button class="badge {spin ? 'preset-filled-warning-500' : 'preset-tonal'}" onclick={() => (spin = !spin)}>
				{spin ? 'stop' : 'spin'}
			</button>
		</div>

		<div class="flex flex-wrap items-center gap-x-4 gap-y-1 text-xs text-surface-400">
			<span>
				area <span class="font-mono text-surface-200">{fmt(region().area, 3)}</span>
				<span class="text-surface-600">({region().areaNote})</span>
			<span class="text-surface-600">· grid tolerance ≈ 8 cells</span>
			</span>
			<span>
				directions with a legal needle
				<span class="font-mono {all ? 'text-success-400' : 'text-surface-200'}">{count}/{BUCKETS}</span>
			</span>
			{#if all}
				<span class="text-success-400">✓ every direction — a Kakeya set</span>
			{:else if !legal()}
				<span class="text-error-400">no legal placement at this angle</span>
			{/if}
		</div>

		<p class="text-xs text-surface-400 leading-relaxed">{region().note}</p>
		{#if all}
			<p class="text-xs text-success-400/90 leading-relaxed">
				You have found a unit needle pointing every way inside {region().label.toLowerCase()},
				whose area is {fmt(region().area, 3)}. The magnet found <em>a</em> placement per
				direction, not necessarily a continuous path — for that, Pál had to show the needle can
				be slid from one placement to the next.
			</p>
		{/if}
	</div>
</div>
