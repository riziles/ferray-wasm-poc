<script lang="ts">
  // Theorem 1.1, drawn. The amber curve is the sharp Sobolev order at a fixed
  // time; the cyan curve is what suffices after averaging over the time
  // interval. Their gap is the whole point of the theorem, and it is exactly
  // 1/p once p clears the critical exponent.

  import { onMount } from 'svelte';
  import { criticalP, fixedTimeOrder, smoothingOrder } from './exponents';
  import { INK, clear, fit } from './util';

  let n = $state(3);
  let p = $state(3);
  let canvas: HTMLCanvasElement;
  let observer: ResizeObserver | null = null;

  const pc = $derived(criticalP(n));

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h, '#0b1020');

    const pMin = 2;
    const pMax = 12;
    const yMax = Math.max(0.9, fixedTimeOrder(n, pMax) * 1.15);
    const X = (q: number) => 40 + ((q - pMin) / (pMax - pMin)) * (w - 56);
    const Y = (v: number) => h - 26 - (v / yMax) * (h - 44);

    // axes
    ctx.strokeStyle = 'rgba(148,163,184,0.3)';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(40, h - 26);
    ctx.lineTo(w - 8, h - 26);
    ctx.moveTo(40, h - 26);
    ctx.lineTo(40, 8);
    ctx.stroke();

    // the critical exponent
    if (pc >= pMin && pc <= pMax) {
      ctx.beginPath();
      ctx.moveTo(X(pc), 8);
      ctx.lineTo(X(pc), h - 26);
      ctx.strokeStyle = 'rgba(248,113,113,0.6)';
      ctx.setLineDash([3, 3]);
      ctx.stroke();
      ctx.setLineDash([]);
      ctx.fillStyle = 'rgba(248,113,113,0.9)';
      ctx.font = '10px ui-monospace, monospace';
      ctx.fillText(`p_c = ${pc.toFixed(2)}`, X(pc) + 4, 18);
    }

    // shade the gain between the two orders, for p >= p_c
    const shade = (from: number, to: number) => {
      ctx.beginPath();
      for (let i = 0; i <= 80; i++) {
        const q = from + ((to - from) * i) / 80;
        if (i === 0) ctx.moveTo(X(q), Y(fixedTimeOrder(n, q)));
        else ctx.lineTo(X(q), Y(fixedTimeOrder(n, q)));
      }
      for (let i = 80; i >= 0; i--) {
        const q = from + ((to - from) * i) / 80;
        ctx.lineTo(X(q), Y(smoothingOrder(n, q)));
      }
      ctx.closePath();
      ctx.fillStyle = 'rgba(74,222,128,0.13)';
      ctx.fill();
    };
    shade(pc, pMax);

    const curve = (f: (q: number) => number, colour: string, width = 2) => {
      ctx.beginPath();
      for (let i = 0; i <= 160; i++) {
        const q = pMin + ((pMax - pMin) * i) / 160;
        if (i === 0) ctx.moveTo(X(q), Y(f(q)));
        else ctx.lineTo(X(q), Y(f(q)));
      }
      ctx.strokeStyle = colour;
      ctx.lineWidth = width;
      ctx.stroke();
    };
    curve((q) => fixedTimeOrder(n, q), 'rgba(251,191,36,0.95)');
    curve((q) => smoothingOrder(n, q), 'rgba(56,189,248,0.95)');

    // the chosen p
    ctx.beginPath();
    ctx.moveTo(X(p), 8);
    ctx.lineTo(X(p), h - 26);
    ctx.strokeStyle = 'rgba(226,232,240,0.35)';
    ctx.stroke();
    for (const [v, colour] of [
      [fixedTimeOrder(n, p), '#fbbf24'],
      [smoothingOrder(n, p), '#38bdf8']
    ] as [number, string][]) {
      ctx.beginPath();
      ctx.arc(X(p), Y(v), 4, 0, Math.PI * 2);
      ctx.fillStyle = colour;
      ctx.fill();
    }

    ctx.fillStyle = INK.text;
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText('p', w - 16, h - 10);
    ctx.fillText('derivatives', 6, 14);
    ctx.fillText('2', 34, h - 12);
    ctx.fillText(`${pMax}`, w - 22, h - 12);
  }

  onMount(() => {
    draw();
    observer = new ResizeObserver(() => draw());
    if (canvas) observer.observe(canvas);
    return () => observer?.disconnect();
  });

  $effect(() => {
    n;
    p;
    draw();
  });
</script>

<div class="rounded-lg border border-surface-700 bg-surface-950/40 overflow-hidden">
  <canvas bind:this={canvas} class="block w-full h-[260px]"></canvas>
  <div class="p-3 border-t border-surface-800 space-y-3">
    <div class="grid sm:grid-cols-2 gap-3">
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap w-14">space dim n</span>
        <input type="range" min="2" max="6" step="1" bind:value={n} class="flex-1" />
        <span class="font-mono w-4 text-right">{n}</span>
      </label>
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap w-14">exponent p</span>
        <input type="range" min="2" max="12" step="0.05" bind:value={p} class="flex-1" />
        <span class="font-mono w-10 text-right">{p.toFixed(2)}</span>
      </label>
    </div>
    <div class="grid sm:grid-cols-2 gap-x-6 gap-y-1 text-xs text-surface-400">
      <div>
        critical p<sub>c</sub> = 2n/(n−1)
        <span class="font-mono text-surface-200 ml-1">{pc.toFixed(3)}</span>
      </div>
      <div>
        fixed-time order <span class="font-mono text-warning-300">{fixedTimeOrder(n, p).toFixed(3)}</span>
      </div>
      <div>
        after averaging <span class="font-mono text-primary-300">{smoothingOrder(n, p).toFixed(3)}</span>
      </div>
      <div>
        gain
        <span class="font-mono text-success-300">
          {p >= pc ? `1/p = ${(1 / p).toFixed(3)}` : 'all of it'}
        </span>
      </div>
    </div>
    <p class="text-xs text-surface-500 leading-relaxed">
      {#if p >= pc}
        At p = {p.toFixed(2)} ≥ p<sub>c</sub> the averaging in time buys exactly 1/p of a derivative —
        the theorem's claim, and nothing about it is a loss in ε.
      {:else}
        Below p<sub>c</sub> = {pc.toFixed(2)} the limit is zero: averaging in time makes all of the
        fixed-time order unnecessary. The critical exponent is where the two statements meet, and it
        is the only case with a logarithm in it.
      {/if}
    </p>
  </div>
</div>
