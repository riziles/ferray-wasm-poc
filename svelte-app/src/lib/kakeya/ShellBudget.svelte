<script lang="ts">
  // Why the theorem's loss is a logarithm. In the focusing example of
  // Proposition 8.1 the mass at distance ~u from the focus contributes
  // u^(d - dp/2) to the p-th power of the spacetime norm. At p = p_c that
  // exponent is exactly -1, so every dyadic shell contributes the same amount
  // and the total grows like log(1/u) — no Sobolev order can remove it.

  import { onMount } from 'svelte';
  import { criticalP, shellExponent, shellSummable, coneDim } from './exponents';
  import { INK, clear, fit } from './util';

  let n = $state(3);
  let p = $state(2.6);
  let shells = 22;
  let canvas: HTMLCanvasElement;
  let observer: ResizeObserver | null = null;

  const pc = $derived(criticalP(n));
  const ex = $derived(shellExponent(n, p));

  function masses(): number[] {
    // mass of the dyadic shell at u = 2^-k, in units of u^(d - dp/2)
    const out: number[] = [];
    for (let k = 1; k <= shells; k++) out.push(Math.pow(2, -k * ex));
    return out;
  }

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h, '#0b1020');
    const m = masses();
    const peak = Math.max(...m);
    const left = 34;
    const right = w - 8;
    const base = h - 30;
    const top = 10;
    const bw = (right - left) / shells;

    for (let k = 1; k <= shells; k++) {
      const value = m[k - 1] / peak;
      const height = Math.max(1, value * (base - top));
      const x = left + (k - 0.5) * bw;
      ctx.fillStyle = Math.abs(ex + 1) < 1e-9 ? 'rgba(248,113,113,0.75)' : 'rgba(56,189,248,0.55)';
      ctx.fillRect(x - bw * 0.34, base - height, bw * 0.68, height);
    }

    // levels for reference
    ctx.strokeStyle = 'rgba(148,163,184,0.25)';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(left, base);
    ctx.lineTo(right, base);
    ctx.stroke();

    ctx.fillStyle = INK.text;
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText('u = ½ (far)', left, h - 12);
    ctx.fillText('dyadic distance from the focus →', left + 74, h - 12);
    ctx.fillText('mass', 4, top + 8);
    ctx.fillText('wavelength', right - 62, h - 12);

    // cumulative curve, rescaled into the panel
    const cumulative: number[] = [];
    let acc = 0;
    for (const v of m) {
      acc += v;
      cumulative.push(acc);
    }
    const total = cumulative[cumulative.length - 1];
    ctx.beginPath();
    for (let k = 0; k < shells; k++) {
      const x = left + (k + 0.5) * bw;
      const y = base - (cumulative[k] / total) * (base - top) * 0.95;
      if (k === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.strokeStyle = 'rgba(254,240,138,0.95)';
    ctx.lineWidth = 2;
    ctx.stroke();
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
  <canvas bind:this={canvas} class="block w-full h-[210px]"></canvas>
  <div class="p-3 border-t border-surface-800 space-y-3">
    <div class="grid sm:grid-cols-2 gap-3">
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap w-14">space dim n</span>
        <input type="range" min="2" max="6" step="1" bind:value={n} class="flex-1" />
        <span class="font-mono w-4 text-right">{n}</span>
      </label>
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap w-14">exponent p</span>
        <input type="range" min="2.05" max="6" step="0.01" bind:value={p} class="flex-1" />
        <span class="font-mono w-10 text-right">{p.toFixed(2)}</span>
      </label>
    </div>
    <div class="flex flex-wrap gap-x-6 gap-y-1 text-xs text-surface-400">
      <span>
        p<sub>c</sub> <span class="font-mono text-surface-200">{pc.toFixed(3)}</span>
      </span>
      <span>
        shell exponent d − dp/2
        <span class="font-mono {Math.abs(ex + 1) < 1e-9 ? 'text-error-300' : 'text-surface-200'}">{ex.toFixed(3)}</span>
      </span>
      <span>
        {#if shellSummable(n, p)}
          <span class="text-success-400">the shell sum converges</span>
        {:else if Math.abs(ex + 1) < 1e-9}
          <span class="text-error-400">harmonic — the sum grows like log(1/u)</span>
        {:else}
          <span class="text-error-400">the shell sum diverges, fast</span>
        {/if}
      </span>
    </div>
    <button
      class="badge preset-tonal"
      onclick={() => {
        p = pc;
        n = n;
      }}
    >
      snap to p = p_c
    </button>
    <p class="text-xs text-surface-500 leading-relaxed">
      Each bar is one dyadic distance from the focus, in the radial example of Proposition 8.1. Slide
      p away from p<sub>c</sub> and the bars decay or blow up; exactly at p<sub>c</sub> (d = n − 1 and
      the exponent is −1) they are all the same height, so the total is the number of scales — a
      logarithm. That is why the theorem carries log²(2 + N) and why no power of the frequency can
      replace it.
    </p>
  </div>
</div>
