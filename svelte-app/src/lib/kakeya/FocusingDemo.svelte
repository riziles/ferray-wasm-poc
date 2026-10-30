<script lang="ts">
  // Why a statement about waves implies a statement about needles. A converging
  // ring of waves focuses: its L^4 norm (the critical exponent for the plane,
  // p = 2n/(n-1) = 4) blows up at the focus — but only for an instant. Sogge's
  // local smoothing conjecture is that the norm averaged over time needs no
  // derivative gain at all. That single inequality implies Kakeya.

  import { onMount } from 'svelte';
  import { INK, clear, fit, fmt } from './util';

  const G = 168;
  const DOM = 2; // domain is [-1, 1]^2
  const H = DOM / (G - 1);
  const COURANT = 0.4;
  const STEPS = 460;
  const P = 4; // 2n/(n-1) with n = 2

  let canvas: HTMLCanvasElement;
  let chart: HTMLCanvasElement;
  let observer: ResizeObserver | null = null;

  let step = $state(200);
  let curve = $state<number[]>([]);
  let peak = $state(0);
  let median = $state(0);
  let now = $state(0);

  const x = (i: number) => -1 + i * H;

  // precomputed sponge, so the inner loop stays a pure stencil
  const damp = new Float32Array(G * G);
  for (let j = 0; j < G; j++) {
    for (let i = 0; i < G; i++) {
      const r = Math.hypot(x(i), x(j));
      damp[j * G + i] = r > 0.82 ? 1 - 0.03 * ((r - 0.82) / 0.18) ** 2 : 1;
    }
  }

  function initField(): { u: Float32Array<ArrayBuffer>; prev: Float32Array<ArrayBuffer> } {
    const u = new Float32Array(G * G);
    const prev = new Float32Array(G * G);
    for (let j = 0; j < G; j++) {
      for (let i = 0; i < G; i++) {
        const px = x(i);
        const py = x(j);
        const r = Math.hypot(px, py);
        const g = Math.exp(-(((r - 0.52) / 0.05) ** 2));
        const dr = (-2 * (r - 0.52)) / 0.05 ** 2 * g;
        u[j * G + i] = g;
        // u_t = c * du/dr: the ring travels inward and focuses near the centre
        prev[j * G + i] = g - 0.4 * H * dr;
      }
    }
    return { u, prev };
  }

  function norm4(u: Float32Array): number {
    let s = 0;
    for (let i = 0; i < u.length; i++) {
      const v = u[i];
      s += v * v * v * v;
    }
    return Math.pow(s * H * H, 1 / P);
  }

  /** Run the scheme to `target` steps; returns the field and (optionally) the norm curve. */
  function run(target: number, record: boolean) {
    const { u, prev } = initField();
    let cur: Float32Array<ArrayBuffer> = u;
    let old: Float32Array<ArrayBuffer> = prev;
    let next: Float32Array<ArrayBuffer> = new Float32Array(u.length);
    const norms: number[] = record ? [norm4(cur)] : [];
    const c2 = (COURANT * COURANT) / 1;

    for (let s = 0; s < target; s++) {
      for (let j = 1; j < G - 1; j++) {
        for (let i = 1; i < G - 1; i++) {
          const k = j * G + i;
          const lap = cur[k - 1] + cur[k + 1] + cur[k - G] + cur[k + G] - 4 * cur[k];
          // sponge: absorb at the edge so reflections do not fake a second focus
          next[k] = (2 * cur[k] - old[k] + c2 * lap) * damp[k];
        }
      }
      const t = old;
      old = cur;
      cur = next;
      next = t;
      if (record) norms.push(norm4(cur));
    }
    return { field: cur, norms };
  }

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h, '#0b1020');
    const { field } = run(step, false);
    now = norm4(field);

    let m = 0;
    for (let i = 0; i < field.length; i++) m = Math.max(m, Math.abs(field[i]));
    m = Math.max(m, 1e-6);

    const off = document.createElement('canvas');
    off.width = G;
    off.height = G;
    const octx = off.getContext('2d')!;
    const img = octx.createImageData(G, G);
    for (let j = 0; j < G; j++) {
      for (let i = 0; i < G; i++) {
        const v = field[j * G + i] / m;
        const a = Math.min(1, Math.sqrt(Math.abs(v)));
        const o = ((G - 1 - j) * G + i) * 4;
        if (v >= 0) {
          img.data[o] = 251;
          img.data[o + 1] = 191;
          img.data[o + 2] = 36;
        } else {
          img.data[o] = 56;
          img.data[o + 1] = 189;
          img.data[o + 2] = 248;
        }
        img.data[o + 3] = Math.round(a * 255);
      }
    }
    octx.putImageData(img, 0, 0);
    const side = Math.min(w, h) * 0.96;
    ctx.imageSmoothingEnabled = true;
    ctx.drawImage(off, (w - side) / 2, (h - side) / 2, side, side);
    ctx.strokeStyle = 'rgba(148,163,184,0.25)';
    ctx.lineWidth = 1;
    ctx.strokeRect((w - side) / 2, (h - side) / 2, side, side);

    ctx.fillStyle = INK.text;
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText(`t = ${fmt(step * COURANT * H, 3)}`, 8, 16);

    drawChart();
  }

  function drawChart() {
    if (!chart || !curve.length) return;
    const { ctx, w, h } = fit(chart);
    clear(ctx, w, h, '#0b1020');
    const yMax = Math.max(...curve) * 1.05;
    const X = (i: number) => 30 + (i / (curve.length - 1)) * (w - 40);
    const Y = (v: number) => h - 14 - (v / yMax) * (h - 26);

    const med = [...curve].sort((a, b) => a - b)[Math.floor(curve.length / 2)];
    ctx.beginPath();
    ctx.moveTo(X(0), Y(med));
    ctx.lineTo(X(curve.length - 1), Y(med));
    ctx.strokeStyle = 'rgba(74,222,128,0.6)';
    ctx.setLineDash([4, 3]);
    ctx.lineWidth = 1;
    ctx.stroke();
    ctx.setLineDash([]);
    ctx.fillStyle = 'rgba(74,222,128,0.85)';
    ctx.font = '9px ui-monospace, monospace';
    ctx.fillText('median over time', X(4), Y(med) - 4);

    ctx.beginPath();
    for (let i = 0; i < curve.length; i++) {
      if (i === 0) ctx.moveTo(X(i), Y(curve[i]));
      else ctx.lineTo(X(i), Y(curve[i]));
    }
    ctx.strokeStyle = 'rgba(251,191,36,0.9)';
    ctx.lineWidth = 1.5;
    ctx.stroke();

    ctx.beginPath();
    ctx.arc(X(step), Y(now), 4, 0, Math.PI * 2);
    ctx.fillStyle = '#fef08a';
    ctx.fill();

    ctx.fillStyle = 'rgba(148,163,184,0.7)';
    ctx.font = '9px ui-monospace, monospace';
    ctx.fillText('‖u(t)‖₄', 2, Y(yMax) + 8);
    ctx.fillText('time →', w - 42, h - 3);
  }

  onMount(() => {
    const { norms } = run(STEPS, true);
    curve = norms;
    peak = Math.max(...norms);
    median = [...norms].sort((a, b) => a - b)[Math.floor(norms.length / 2)];
    draw();
    observer = new ResizeObserver(() => draw());
    if (canvas) observer.observe(canvas);
    return () => observer?.disconnect();
  });

  $effect(() => {
    step;
    draw();
  });
</script>

<div class="rounded-lg border border-surface-700 bg-surface-950/40 overflow-hidden">
  <div class="grid md:grid-cols-[1fr_1fr]">
    <canvas bind:this={canvas} class="block w-full h-[300px]"></canvas>
    <div class="border-t md:border-t-0 md:border-l border-surface-800 p-3 space-y-2">
      <canvas bind:this={chart} class="block w-full h-[150px] rounded border border-surface-800"></canvas>
      <div class="space-y-1 text-xs text-surface-400">
        <div>
          ‖u(t)‖₄ now <span class="font-mono text-surface-200">{fmt(now, 2)}</span>
          <span class="text-surface-600">of peak {fmt(peak, 2)}</span>
        </div>
        <div>
          peak is <span class="font-mono text-warning-300">{fmt(peak / Math.max(median, 1e-9), 1)}×</span>
          the median over time
        </div>
      </div>
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap">time</span>
        <input type="range" min="0" max={STEPS} step="2" bind:value={step} class="flex-1" />
      </label>
      <p class="text-xs text-surface-500 leading-relaxed">
        A ring of waves converging on a point, run with an explicit finite-difference scheme on a
        168×168 grid. Watch the L⁴ norm as it crosses the focus: the spike is real, and it is over in
        the time it takes the wave to travel its own width. Sogge's conjecture is the statement that
        this transient is all you ever get.
      </p>
    </div>
  </div>
</div>
