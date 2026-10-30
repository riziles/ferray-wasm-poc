<script lang="ts">
  // The Kakeya maximal function, made countable: one tube of length 1 and width
  // δ per direction, directions δ apart. Their total area is fixed by δ. How
  // small can their union be? The bush is the extreme case, and it saves only a
  // constant factor — which is the whole content of the maximal conjecture.

  import { onMount } from 'svelte';
  import { rasterize, rectPoly, squareBox, boxOf, padBox } from './geom';
  import type { Poly } from './geom';
  import { INK, clear, fit, fmt, viewFor, tracePoly } from './util';

  let delta = $state(0.08);
  let spread = $state(1); // 0 = all through one point, 1 = pushed apart
  let canvas: HTMLCanvasElement;
  let observer: ResizeObserver | null = null;

  const N = $derived(Math.max(6, Math.round((2 * Math.PI) / delta)));
  const total = $derived(N * delta);
  let measured = $state(0);
  let bush = $state(0);

  function placements(spreadValue: number): Poly[] {
    // At spread = 1 the ring is long enough to hold all the tubes side by side,
    // so the union there is the sum of the areas and the compression is 1.
    const ring = spreadValue * Math.max(0.9, (N * delta) / (2 * Math.PI)) * 1.02;
    const out: Poly[] = [];
    for (let i = 0; i < N; i++) {
      const th = (i / N) * Math.PI * 2;
      const cx = Math.cos(th + Math.PI / 2) * ring;
      const cy = Math.sin(th + Math.PI / 2) * ring;
      out.push(rectPoly(cx, cy, 1, delta, th));
    }
    return out;
  }

  function unionArea(polys: Poly[]): number {
    const box = squareBox(padBox(boxOf(polys), 0.03));
    const raster = rasterize(polys, 520, box);
    let occupied = 0;
    for (let i = 0; i < raster.cell.length; i++) occupied += raster.cell[i];
    return occupied * ((box.x1 - box.x0) / 520) ** 2;
  }

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h, '#0b1020');

    const polys = placements(spread);
    const box = squareBox(padBox(boxOf(placements(1)), 0.03));
    const view = viewFor(w, h, box, 0.95);

    measured = unionArea(polys);
    bush = unionArea(placements(0));

    for (const poly of polys) {
      ctx.beginPath();
      tracePoly(ctx, poly, view);
      ctx.fillStyle = 'rgba(56,189,248,0.13)';
      ctx.fill();
      ctx.strokeStyle = 'rgba(125,211,252,0.4)';
      ctx.lineWidth = 0.8;
      ctx.stroke();
    }

    ctx.fillStyle = INK.text;
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText(`${N} tubes, each 1 × ${fmt(delta, 3)}`, 8, 16);
  }

  onMount(() => {
    draw();
    observer = new ResizeObserver(() => draw());
    if (canvas) observer.observe(canvas);
    return () => observer?.disconnect();
  });

  $effect(() => {
    delta;
    spread;
    draw();
  });
</script>

<div class="rounded-lg border border-surface-700 bg-surface-950/40 overflow-hidden">
  <canvas bind:this={canvas} class="block w-full h-[340px]"></canvas>
  <div class="p-3 border-t border-surface-800 space-y-3">
    <div class="grid sm:grid-cols-2 gap-3">
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap w-20">tube width δ</span>
        <input type="range" min="0.02" max="0.2" step="0.005" bind:value={delta} class="flex-1" />
        <span class="font-mono w-12 text-right">{delta.toFixed(3)}</span>
      </label>
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap w-20">arrangement</span>
        <input type="range" min="0" max="1" step="0.01" bind:value={spread} class="flex-1" />
        <span class="font-mono w-12 text-right">{spread < 0.05 ? 'bush' : spread > 0.95 ? 'spread' : spread.toFixed(2)}</span>
      </label>
    </div>
    <div class="flex flex-wrap gap-x-6 gap-y-1 text-xs text-surface-400">
      <span>
        total area <span class="font-mono text-surface-200">{fmt(total, 2)}</span>
        <span class="text-surface-600">= N·δ, fixed</span>
      </span>
      <span>
        union <span class="font-mono text-surface-200">{fmt(measured, 2)}</span>
        → compression
        <span class="font-mono text-warning-300">{fmt(total / Math.max(measured, 1e-6), 1)}×</span>
      </span>
      <span>
        the bush manages
        <span class="font-mono text-error-300">{fmt(total / Math.max(bush, 1e-6), 1)}×</span>
      </span>
    </div>
    <p class="text-xs text-surface-500 leading-relaxed">
      The direction list is fixed, so the total area N·δ ≈ 2π is fixed; the only freedom is where to
      put the tubes. The bush's union is the unit disk, area π/4 — exactly 8× less — and that factor
      never improves, however thin the tubes get. Shrink δ and watch: neither arrangement's
      compression grows. The claim that <em>no</em> arrangement ever beats a constant is the Kakeya
      <em>maximal</em> conjecture, and it implies the set problem above.
    </p>
  </div>
</div>
