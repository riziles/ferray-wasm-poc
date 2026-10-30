<script lang="ts">
  // Perron's construction: split, slide, repeat. The area falls towards 1-alpha
  // of the original triangle while every direction keeps its own needle. The
  // eight rotated copies are what turns "45° of directions" into a Kakeya set.

  import { onMount } from 'svelte';
  import {
    fanPolys,
    treePolys,
    witness,
    optimalAlpha,
    areaFraction,
    perronLeaves,
    leafPoly,
    COPY_ANGLE,
    COPIES
  } from './perron';
  import { rotate } from './geom';
  import { polysToRaster, rotationalUnion, setArea } from './figure';
  import { INK, clamp, clear, fit, fmt, viewFor } from './util';
  import type { View } from './util';

  let n = $state(6);
  let alpha = $state(optimalAlpha(6));
  let angle = $state(0.35);
  let sweep = $state(true);
  let canvas: HTMLCanvasElement;
  let chart: HTMLCanvasElement;
  let raf = 0;

  let bakedKey = '';
  let baked: HTMLCanvasElement | null = null;
  let bakedView: View | null = null;
  let measured = $state(0);

  const leaves = $derived(1 << n);
  const fraction = $derived(areaFraction(n, alpha));
  const best = $derived(optimalAlpha(n));

  /** Smallest depth whose best-case area is under `target`. */
  function depthFor(target: number): number {
    for (let k = 1; k <= 4000; k++) if (areaFraction(k, optimalAlpha(k)) <= target) return k;
    return -1;
  }

  const depth1pct = $derived(depthFor(0.01));

  function rebuild() {
    if (!canvas) return;
    const { w, h } = fit(canvas);
    const polys = fanPolys(n, alpha);
    let radius = 0;
    for (const p of polys) for (const [x, y] of p) radius = Math.max(radius, Math.hypot(x, y));
    const box = { x0: -radius, y0: -radius, x1: radius, y1: radius };
    const view = viewFor(w, h, box, 0.92);
    bakedView = view;

    const off = document.createElement('canvas');
    const dpr = Math.min(window.devicePixelRatio || 1, 2);
    off.width = Math.round(w * dpr);
    off.height = Math.round(h * dpr);
    const octx = off.getContext('2d')!;
    octx.setTransform(dpr, 0, 0, dpr, 0, 0);
    octx.fillStyle = '#0b1020';
    octx.fillRect(0, 0, w, h);

    // Build the copy in WORLD units, then let the canvas transform do the
    // eight rotations. Building the path in screen pixels and rotating the
    // canvas would apply the view offset twice.
    const one = treePolys(n, alpha);
    const path = new Path2D();
    for (const poly of one) {
      path.moveTo(poly[0][0], poly[0][1]);
      for (let i = 1; i < poly.length; i++) path.lineTo(poly[i][0], poly[i][1]);
      path.closePath();
    }
    // the original triangle of the construction, for scale
    const rootPath = new Path2D();
    rootPath.moveTo(0, 0);
    rootPath.lineTo(0, -1);
    rootPath.lineTo(1, -1);
    rootPath.closePath();

    const base = () => {
      octx.setTransform(dpr, 0, 0, dpr, 0, 0);
      octx.translate(view.cx, view.cy);
      octx.scale(view.scale, -view.scale);
    };

    octx.setTransform(dpr, 0, 0, dpr, 0, 0);
    octx.fillStyle = '#0b1020';
    octx.fillRect(0, 0, w, h);

    for (let c = 0; c < COPIES; c++) {
      base();
      octx.rotate(c * COPY_ANGLE);
      octx.setLineDash([4 / view.scale, 4 / view.scale]);
      octx.lineWidth = 1 / view.scale;
      octx.strokeStyle = 'rgba(148,163,184,0.30)';
      octx.stroke(rootPath);
      octx.setLineDash([]);
    }

    // The leaves are needles 2^-n wide, so outline as well as fill: a fill at
    // alpha alone is nearly invisible at depth, which would misrepresent it.
    for (let c = 0; c < COPIES; c++) {
      base();
      octx.rotate(c * COPY_ANGLE);
      const hue = (c / COPIES) * 300 + 185;
      octx.fillStyle = `hsla(${hue}, 88%, 64%, 0.34)`;
      octx.strokeStyle = `hsla(${hue}, 92%, 78%, 0.8)`;
      octx.lineWidth = 0.8 / view.scale;
      octx.fill(path);
      octx.stroke(path);
    }

    baked = off;
    bakedKey = `${n}:${alpha}:${w}x${h}`;

    // Measure the union of the eight copies (not the tree alone).
    const raster = polysToRaster(one, 560);
    measured = setArea(rotationalUnion(raster, COPIES, COPY_ANGLE, 560)) / (8 * 0.5);
  }

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    const key = `${n}:${alpha}:${w}x${h}`;
    if (!baked || key !== bakedKey) rebuild();
    clear(ctx, w, h);
    if (baked) ctx.drawImage(baked, 0, 0, w, h);
    const view = bakedView;
    if (!view) return;

    // The witness: one leaf's own needle, in the direction the dial asks for.
    // Highlight the leaf that holds it, so the leap of logic is visible: the
    // needle is inside a piece because the piece is a translate of a slice of
    // the original triangle.
    const wi = witness(angle, n, alpha);
    const leaf = leafPoly(perronLeaves(n, alpha)[wi.leaf]).map((p) =>
      rotate(p, wi.copy * COPY_ANGLE)
    );
    ctx.beginPath();
    ctx.moveTo(view.toX(leaf[0][0]), view.toY(leaf[0][1]));
    for (let i = 1; i < leaf.length; i++) ctx.lineTo(view.toX(leaf[i][0]), view.toY(leaf[i][1]));
    ctx.closePath();
    // The arms blend to near-white where all eight copies overlap, so the
    // witness gets a light halo and a dark core: it has to read on both.
    ctx.strokeStyle = 'rgba(11,16,32,0.85)';
    ctx.lineWidth = 1.5;
    ctx.stroke();

    ctx.lineCap = 'round';
    ctx.beginPath();
    ctx.moveTo(view.toX(wi.a[0]), view.toY(wi.a[1]));
    ctx.lineTo(view.toX(wi.b[0]), view.toY(wi.b[1]));
    ctx.strokeStyle = 'rgba(248,250,252,0.98)';
    ctx.lineWidth = 7;
    ctx.stroke();
    ctx.beginPath();
    ctx.moveTo(view.toX(wi.a[0]), view.toY(wi.a[1]));
    ctx.lineTo(view.toX(wi.b[0]), view.toY(wi.b[1]));
    ctx.strokeStyle = '#0b1020';
    ctx.lineWidth = 3;
    ctx.stroke();
    ctx.beginPath();
    ctx.arc(view.toX(wi.a[0]), view.toY(wi.a[1]), 2.5, 0, Math.PI * 2);
    ctx.fillStyle = '#fef08a';
    ctx.fill();

    // direction indicator
    ctx.beginPath();
    ctx.moveTo(16, 16);
    ctx.lineTo(16 + Math.cos(angle) * 22, 16 - Math.sin(angle) * 22);
    ctx.strokeStyle = 'rgba(254,240,138,0.85)';
    ctx.lineWidth = 2;
    ctx.stroke();
    ctx.fillStyle = INK.text;
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText(`${Math.round((angle * 180) / Math.PI) % 360}°`, 44, 20);

    drawChart();
  }

  let lastChartKey = '';

  function drawChart() {
    if (!chart) return;
    const { ctx, w, h } = fit(chart);
    const key = `${w}x${h}`;
    if (key !== lastChartKey) {
      clear(ctx, w, h, '#0b1020');
      lastChartKey = key;
    }
    clear(ctx, w, h, '#0b1020');

    const nMax = 40;
    const yMin = 1e-3;
    const yMax = 1;
    const X = (k: number) => 24 + ((k - 1) / (nMax - 1)) * (w - 34);
    const Yv = (v: number) => {
      const t = Math.log10(clamp(v, yMin, yMax) / yMin) / Math.log10(yMax / yMin);
      return h - 14 - t * (h - 26);
    };

    // gridlines at 1, 10%, 1%, 0.1%
    for (const v of [1, 0.1, 0.01, 0.001]) {
      ctx.beginPath();
      ctx.moveTo(24, Yv(v));
      ctx.lineTo(w - 6, Yv(v));
      ctx.strokeStyle = 'rgba(148,163,184,0.16)';
      ctx.lineWidth = 1;
      ctx.stroke();
      ctx.fillStyle = 'rgba(148,163,184,0.6)';
      ctx.font = '9px ui-monospace, monospace';
      ctx.fillText(v === 1 ? '100%' : `${v * 100}%`, 2, Yv(v) + 3);
    }

    // best achievable area at each depth, and the line for the current alpha
    const curve = (f: (k: number) => number, colour: string) => {
      ctx.beginPath();
      for (let k = 1; k <= nMax; k++) {
        const y = Yv(f(k));
        if (k === 1) ctx.moveTo(X(k), y);
        else ctx.lineTo(X(k), y);
      }
      ctx.strokeStyle = colour;
      ctx.lineWidth = 1.5;
      ctx.stroke();
    };
    curve((k) => areaFraction(k, optimalAlpha(k)), 'rgba(52,211,153,0.9)');
    curve((k) => areaFraction(k, alpha), 'rgba(56,189,248,0.55)');

    // current depth
    ctx.beginPath();
    ctx.arc(X(n), Yv(fraction), 3.5, 0, Math.PI * 2);
    ctx.fillStyle = '#fef08a';
    ctx.fill();
    ctx.strokeStyle = 'rgba(254,240,138,0.35)';
    ctx.beginPath();
    ctx.moveTo(X(n), Yv(fraction));
    ctx.lineTo(X(n), h - 14);
    ctx.stroke();

    ctx.fillStyle = INK.text;
    ctx.font = '9px ui-monospace, monospace';
    ctx.fillText('1', 24, h - 4);
    ctx.fillText(`${nMax}`, w - 18, h - 4);
    ctx.fillText('levels n', w / 2 - 22, h - 4);
  }

  function loop() {
    if (sweep) angle = (angle + 0.004) % (Math.PI * 2);
    draw();
    raf = requestAnimationFrame(loop);
  }

  onMount(() => {
    loop();
    return () => cancelAnimationFrame(raf);
  });
</script>

<div class="rounded-lg border border-surface-700 bg-surface-950/40 overflow-hidden">
  <div class="relative">
    <canvas bind:this={canvas} class="block w-full h-[380px]"></canvas>
    <div class="absolute top-2 right-3 text-[10px] uppercase tracking-wider text-surface-500">
      {COPIES} copies × {leaves.toLocaleString()} leaves
    </div>
  </div>

  <div class="p-3 border-t border-surface-800 space-y-3">
    <div class="grid sm:grid-cols-2 gap-3">
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap w-16">levels n</span>
        <input
          type="range"
          min="1"
          max="12"
          step="1"
          bind:value={n}
          class="flex-1"
        />
        <span class="font-mono w-6 text-right">{n}</span>
      </label>
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap w-16">slide α</span>
        <input
          type="range"
          min="0.3"
          max="0.98"
          step="0.005"
          bind:value={alpha}
          class="flex-1"
        />
        <span class="font-mono w-12 text-right">{alpha.toFixed(3)}</span>
      </label>
    </div>

    <div class="flex flex-wrap items-center gap-2 text-xs">
      <button
        class="badge preset-tonal"
        onclick={() => {
          n = clamp(n, 1, 12);
          alpha = optimalAlpha(n);
        }}
      >
        best α for this n ({best.toFixed(3)})
      </button>
      <button class="badge {sweep ? 'preset-filled-warning-500' : 'preset-tonal'}" onclick={() => (sweep = !sweep)}>
        {sweep ? 'stop sweep' : 'sweep'}
      </button>
      <span class="text-surface-500">the needle is one leaf's own needle — from its apex</span>
    </div>

    <div class="grid sm:grid-cols-2 gap-4 items-start">
      <div class="space-y-1 text-xs text-surface-400">
        <div>
          pieces <span class="font-mono text-surface-200">{leaves.toLocaleString()}</span> per copy,
          <span class="font-mono text-surface-200">{(leaves * COPIES).toLocaleString()}</span> in all
        </div>
        <div>
          tree area <span class="font-mono text-surface-200">{fmt(fraction * 100, 1)}%</span> of the
          original triangle
          <span class="text-surface-600">= (1−α) + α^(2n)</span>
        </div>
        <div>
          measured fan <span class="font-mono text-surface-200">{fmt(measured, 3)}</span>
          <span class="text-surface-600">× the original triangle's area</span>
        </div>
        <div class="text-surface-500 pt-1">
          Drawing it costs {leaves.toLocaleString()} pieces per copy. Reaching
          <span class="text-surface-300">1%</span> of the triangle's area needs about
          <span class="font-mono text-surface-300">n = {depth1pct}</span> levels, i.e.
          <span class="font-mono text-surface-300">2^{depth1pct} ≈ 10^{Math.round(depth1pct * 0.30103)}</span>
          pieces per copy.
        </div>
      </div>
      <div class="space-y-1">
        <canvas bind:this={chart} class="block w-full h-[120px] rounded border border-surface-800"></canvas>
        <div class="text-[10px] text-surface-500">
          area against depth: green = best α at each n, blue = the α you picked
        </div>
      </div>
    </div>
  </div>
</div>
