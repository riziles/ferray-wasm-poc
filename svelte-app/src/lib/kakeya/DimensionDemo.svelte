<script lang="ts">
  // Area is the wrong question. Cover the set with boxes of side δ, count them,
  // and watch N(δ) grow like δ^-d: that exponent d is the dimension, and it is
  // what the Kakeya conjecture is actually about.

  import { onMount } from 'svelte';
  import { boxCount, diskPoly, rectPoly } from './geom';
  import { polysToRaster, rotationalUnion } from './figure';
  import { treePolys, optimalAlpha, COPY_ANGLE, COPIES } from './perron';
  import { INK, clear, fit, fmt } from './util';

  interface Shape {
    id: string;
    label: string;
    blurb: string;
    build: () => { n: number; box: { x0: number; y0: number; x1: number; y1: number }; cell: Uint8Array };
  }

  const SHAPES: Shape[] = [
    {
      id: 'needle',
      label: 'one needle',
      blurb: 'A single segment, thickened to 1% so it can be drawn. Its boxes grow like δ⁻¹.',
      build: () => polysToRaster([rectPoly(0, 0, 1, 0.01, 0.4)], 512)
    },
    {
      id: 'fan',
      label: 'Perron fan, n = 6',
      blurb: 'The construction above: small area, and boxes that grow like δ⁻² anyway.',
      build: () => {
        const a = optimalAlpha(6);
        const one = polysToRaster(treePolys(6, a), 560);
        return rotationalUnion(one, COPIES, COPY_ANGLE, 512);
      }
    },
    {
      id: 'disk',
      label: 'disk',
      blurb: 'The friendly case, for comparison: a solid region, dimension 2, no small-scale structure.',
      build: () => polysToRaster([diskPoly(0.5, 260)], 512)
    }
  ];

  const KS = [1, 2, 4, 8, 16, 32, 64];

  let shapeId = $state('fan');
  let kIndex = $state(2);
  let canvas: HTMLCanvasElement;
  let chart: HTMLCanvasElement;
  let observer: ResizeObserver | null = null;

  const cache = new Map<string, { set: ReturnType<Shape['build']>; counts: number[]; cell: number }>();

  function data() {
    const shape = SHAPES.find((s) => s.id === shapeId) ?? SHAPES[0];
    let hit = cache.get(shape.id);
    if (!hit) {
      const set = shape.build();
      const counts = KS.map((k) => boxCount(set as never, k));
      const cell = (set.box.x1 - set.box.x0) / set.n;
      hit = { set, counts, cell };
      cache.set(shape.id, hit);
    }
    return hit;
  }

  const delta = $derived(data().cell * KS[kIndex]);
  const count = $derived(data().counts[kIndex]);
  const localSlope = $derived.by(() => {
    const c = data().counts;
    const i = kIndex;
    const next = i + 1 < c.length ? c[i + 1] : null;
    if (next === null) return NaN;
    return -Math.log(next / c[i]) / Math.log(2);
  });
  const fitted = $derived.by(() => {
    const c = data().counts;
    // the finest four rulers: the estimate is a limit, so it is the small-δ
    // end that matters
    const tail = c.slice(-4);
    const xs = KS.slice(-4).map((k) => Math.log(1 / k));
    const ys = tail.map((v) => Math.log(v + 1e-9));
    const mx = xs.reduce((a, b) => a + b, 0) / xs.length;
    const my = ys.reduce((a, b) => a + b, 0) / ys.length;
    let num = 0;
    let den = 0;
    for (let i = 0; i < xs.length; i++) {
      num += (xs[i] - mx) * (ys[i] - my);
      den += (xs[i] - mx) ** 2;
    }
    return num / den;
  });

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h, '#0b1020');
    const { set, cell } = data();
    const k = KS[kIndex];
    const m = Math.floor(set.n / k);

    // The boxes, as one blocky image: nearest-neighbour upscaling keeps the
    // grid honest (each pixel here IS one δ-box).
    const off = document.createElement('canvas');
    off.width = m;
    off.height = m;
    const octx = off.getContext('2d')!;
    const img = octx.createImageData(m, m);
    let seen = 0;
    for (let bj = 0; bj < m; bj++) {
      for (let bi = 0; bi < m; bi++) {
        let any = 0;
        for (let j = 0; j < k && !any; j++) {
          const row = (bj * k + j) * set.n + bi * k;
          for (let i = 0; i < k; i++) {
            if (set.cell[row + i]) {
              any = 1;
              break;
            }
          }
        }
        if (any) {
          seen++;
          const o = (bj * m + bi) * 4;
          img.data[o] = 56;
          img.data[o + 1] = 189;
          img.data[o + 2] = 248;
          img.data[o + 3] = 235;
        }
      }
    }
    octx.putImageData(img, 0, 0);

    const side = Math.min(w, h) * 0.92;
    ctx.imageSmoothingEnabled = false;
    ctx.drawImage(off, (w - side) / 2, (h - side) / 2, side, side);
    ctx.imageSmoothingEnabled = true;

    ctx.fillStyle = INK.text;
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText(`δ = ${fmt(cell * k, 4)}   N = ${seen.toLocaleString()} boxes`, 8, h - 8);

    drawChart();
  }

  function drawChart() {
    if (!chart) return;
    const { ctx, w, h } = fit(chart);
    clear(ctx, w, h, '#0b1020');
    const { counts } = data();
    const xs = KS.map((k) => Math.log10(1 / k));
    const ys = counts.map((c) => Math.log10(c));
    const x0 = Math.min(...xs);
    const x1 = Math.max(...xs);
    const y0 = Math.min(...ys);
    const y1 = Math.max(...ys);
    const X = (x: number) => 34 + ((x - x0) / (x1 - x0)) * (w - 46);
    const Y = (y: number) => h - 16 - ((y - y0) / (y1 - y0)) * (h - 28);

    // the fiducial slopes, anchored at the finest point
    const anchorX = xs[0];
    const anchorY = ys[0];
    for (const [slope, colour, label] of [
      [1, 'rgba(251,191,36,0.75)', 'δ⁻¹'],
      [2, 'rgba(74,222,128,0.75)', 'δ⁻²']
    ] as [number, string, string][]) {
      ctx.beginPath();
      ctx.moveTo(X(anchorX), Y(anchorY));
      ctx.lineTo(X(x1), Y(anchorY + slope * (x1 - anchorX)));
      ctx.strokeStyle = colour;
      ctx.setLineDash([3, 3]);
      ctx.lineWidth = 1;
      ctx.stroke();
      ctx.setLineDash([]);
      ctx.fillStyle = colour;
      ctx.font = '9px ui-monospace, monospace';
      ctx.fillText(label, X(x1) - 22, Y(anchorY + slope * (x1 - anchorX)) - 3);
    }

    ctx.beginPath();
    for (let i = 0; i < xs.length; i++) {
      if (i === 0) ctx.moveTo(X(xs[i]), Y(ys[i]));
      else ctx.lineTo(X(xs[i]), Y(ys[i]));
    }
    ctx.strokeStyle = 'rgba(192,132,252,0.85)';
    ctx.lineWidth = 1.5;
    ctx.stroke();
    for (let i = 0; i < xs.length; i++) {
      ctx.beginPath();
      ctx.arc(X(xs[i]), Y(ys[i]), i === kIndex ? 4 : 2.2, 0, Math.PI * 2);
      ctx.fillStyle = i === kIndex ? '#fef08a' : 'rgba(192,132,252,0.9)';
      ctx.fill();
    }
    ctx.fillStyle = 'rgba(148,163,184,0.7)';
    ctx.font = '9px ui-monospace, monospace';
    ctx.fillText('log N', 2, Y(y1) + 8);
    ctx.fillText('log 1/δ', w - 44, h - 4);
  }

  onMount(() => {
    draw();
    observer = new ResizeObserver(() => draw());
    if (canvas) observer.observe(canvas);
    return () => observer?.disconnect();
  });

  $effect(() => {
    // redraw whenever the shape or the ruler changes
    shapeId;
    kIndex;
    draw();
  });
</script>

<div class="rounded-lg border border-surface-700 bg-surface-950/40 overflow-hidden">
  <div class="flex flex-wrap items-center gap-2 p-3 border-b border-surface-800">
    {#each SHAPES as s}
      <button
        class="badge {shapeId === s.id ? 'preset-filled-primary-500' : 'preset-tonal'}"
        onclick={() => (shapeId = s.id)}
      >
        {s.label}
      </button>
    {/each}
    <label class="flex items-center gap-2 text-xs flex-1 min-w-[180px]">
      <span class="text-surface-400 whitespace-nowrap">ruler δ</span>
      <input type="range" min="0" max={KS.length - 1} step="1" bind:value={kIndex} class="flex-1" />
      <span class="font-mono w-10 text-right">{KS[kIndex]}×</span>
    </label>
  </div>

  <div class="grid md:grid-cols-[1fr_320px]">
    <canvas bind:this={canvas} class="block w-full h-[330px]"></canvas>
    <div class="border-t md:border-t-0 md:border-l border-surface-800 p-3 space-y-2">
      <canvas bind:this={chart} class="block w-full h-[150px] rounded border border-surface-800"></canvas>
      <div class="space-y-1 text-xs text-surface-400">
        <div>
          N(δ) = <span class="font-mono text-surface-200">{count.toLocaleString()}</span>
          at δ = <span class="font-mono text-surface-200">{fmt(delta, 4)}</span>
        </div>
        <div>
          slope between this ruler and the next:
          <span class="font-mono text-warning-300">{isNaN(localSlope) ? '—' : localSlope.toFixed(2)}</span>
        </div>
        <div>
          fitted at the finest four rulers:
          <span class="font-mono text-success-300">{fitted.toFixed(2)}</span>
        </div>
      </div>
      <p class="text-xs text-surface-500 leading-relaxed">
        {SHAPES.find((s) => s.id === shapeId)?.blurb}
      </p>
    </div>
  </div>
</div>
