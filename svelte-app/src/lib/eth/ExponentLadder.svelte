<script lang="ts">
  // The page's central quantitative point, drawn. "Subexponential" is a claim
  // about the limit of the exponent, and three different bounds in play here
  // approach the limit at wildly different rates:
  //
  //   cyan   1/sqrt(log n)                 the abstract
  //   green  log log n / sqrt(log n)       Theorem 1.1
  //   amber  (log log n)^2 / sqrt(log n)   the Lean development
  //
  // The amber one is above 1 — worse than brute force — for every n below
  // about 10^2390, and it is the only version a machine has checked.

  import { onMount } from 'svelte';
  import { INK, clear, fit } from './util';
  import {
    alphaAbstract,
    alphaLean,
    alphaManuscript,
    exponentOfBase,
    fmtCount,
    fmtLog10,
    log10NForAbstractExponent,
    log10NForLeanExponent,
    log10NForManuscriptExponent,
    log10Time,
    superscript
  } from './numerics';

  let lg = $state(3.2);
  let canvas: HTMLCanvasElement;
  let observer: ResizeObserver | null = null;

  const n = $derived(Math.pow(10, lg));
  const aAbs = $derived(alphaAbstract(n));
  const aMan = $derived(alphaManuscript(n));
  const aLean = $derived(alphaLean(n));
  const lgTimeAbs = $derived(log10Time(aAbs, n));

  const lgLeanBelowOne = log10NForLeanExponent(1);
  const lgManBeats1308 = log10NForManuscriptExponent(exponentOfBase(1.308));
  const lgAbsBeats110 = log10NForAbstractExponent(exponentOfBase(1.1));

  const refs = [
    { label: '#3-SAT counter 1.637ⁿ', a: exponentOfBase(1.637) },
    { label: '3-SAT decider 1.308ⁿ', a: exponentOfBase(1.308) },
    { label: '1.1ⁿ', a: exponentOfBase(1.1) }
  ];

  const yMax = 2.3;
  const L = 46;
  const R = 26;
  const T = 20;
  const B = 30;

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h, '#0b1020');

    const x0 = 0.5;
    const x1 = 15;
    const X = (v: number) => L + ((v - x0) / (x1 - x0)) * (w - L - R);
    const Y = (a: number) => h - B - (Math.min(a, yMax) / yMax) * (h - T - B);
    const nAt = (v: number) => Math.pow(10, v);

    // decade grid; label every third decade so the axis stays readable
    ctx.lineWidth = 1;
    ctx.font = '10px ui-monospace, monospace';
    for (let v = 1; v <= x1; v++) {
      const x = X(v);
      ctx.strokeStyle = 'rgba(148,163,184,0.09)';
      ctx.beginPath();
      ctx.moveTo(x, T);
      ctx.lineTo(x, h - B);
      ctx.stroke();
      if (v % 3 === 0) {
        ctx.fillStyle = INK.dim;
        ctx.textAlign = 'center';
        ctx.fillText(`10${superscript(v)}`, x, h - B + 14);
        ctx.textAlign = 'left';
      }
    }

    // axes
    ctx.strokeStyle = 'rgba(148,163,184,0.35)';
    ctx.beginPath();
    ctx.moveTo(L, h - B);
    ctx.lineTo(w - R + 6, h - B);
    ctx.moveTo(L, h - B);
    ctx.lineTo(L, T);
    ctx.stroke();

    // brute force + reference exponents, labelled inside the frame on the right
    const hline = (a: number, colour: string, dash: number[], label: string, bold = false) => {
      ctx.setLineDash(dash);
      ctx.strokeStyle = colour;
      ctx.lineWidth = bold ? 1.4 : 1;
      ctx.beginPath();
      ctx.moveTo(L, Y(a));
      ctx.lineTo(w - R + 6, Y(a));
      ctx.stroke();
      ctx.setLineDash([]);
      if (label) {
        ctx.fillStyle = colour;
        // Lines close to the x-axis get their label on the left, where there is
        // room; the rest are right-aligned against the frame.
        const low = a < 0.25;
        ctx.textAlign = low ? 'left' : 'right';
        ctx.fillText(label, low ? L + 8 : w - R - 4, Y(a) - 5);
        ctx.textAlign = 'left';
      }
    };
    hline(1, 'rgba(226,232,240,0.6)', [2, 4], 'brute force 2ⁿ', true);
    for (const r of refs) hline(r.a, 'rgba(148,163,184,0.55)', [5, 5], r.label);

    const curve = (f: (x: number) => number, colour: string, width = 2.5) => {
      ctx.beginPath();
      for (let i = 0; i <= 260; i++) {
        const v = x0 + ((x1 - x0) * i) / 260;
        const y = Y(f(nAt(v)));
        if (i === 0) ctx.moveTo(X(v), y);
        else ctx.lineTo(X(v), y);
      }
      ctx.strokeStyle = colour;
      ctx.lineWidth = width;
      ctx.stroke();
    };
    curve(alphaAbstract, INK.primary);
    curve(alphaManuscript, INK.good);
    curve(alphaLean, INK.warn);

    // the chosen n
    ctx.strokeStyle = 'rgba(226,232,240,0.28)';
    ctx.setLineDash([3, 3]);
    ctx.beginPath();
    ctx.moveTo(X(lg), T);
    ctx.lineTo(X(lg), h - B);
    ctx.stroke();
    ctx.setLineDash([]);

    const dot = (a: number, colour: string) => {
      ctx.beginPath();
      ctx.arc(X(lg), Y(a), 3.5, 0, Math.PI * 2);
      ctx.fillStyle = colour;
      ctx.fill();
    };
    dot(aAbs, INK.primary);
    dot(aMan, INK.good);
    dot(aLean, INK.warn);

    // axis labels
    ctx.fillStyle = INK.text;
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText('α, where time = 2^(α·n)', L - 40, T - 6);
    ctx.textAlign = 'right';
    ctx.fillText('n →', w - R + 6, h - B + 14);
    ctx.textAlign = 'left';
  }

  onMount(() => {
    draw();
    observer = new ResizeObserver(() => draw());
    if (canvas) observer.observe(canvas);
    return () => observer?.disconnect();
  });

  $effect(() => {
    lg;
    draw();
  });

  const presets = [
    { label: 'n = 100', v: 2 },
    { label: 'n = 10⁶', v: 6 },
    { label: 'n = 10¹²', v: 12 }
  ];
</script>

<div class="rounded-lg border border-surface-700 bg-surface-950/40 overflow-hidden">
  <div class="flex flex-wrap gap-x-4 gap-y-1 px-3 pt-3 text-[11px] font-mono">
    <span class="text-primary-300">— abstract 1/√ln n</span>
    <span class="text-success-300">— Thm 1.1 ln ln n/√ln n</span>
    <span class="text-warning-300">— Lean (ln ln n)²/√ln n</span>
  </div>
  <canvas bind:this={canvas} class="block w-full h-[290px]"></canvas>
  <div class="p-3 border-t border-surface-800 space-y-3">
    <div class="flex flex-wrap items-center gap-2">
      <label class="flex items-center gap-2 text-xs flex-1 min-w-[220px]">
        <span class="text-surface-400 whitespace-nowrap">variables n</span>
        <input type="range" min="0.5" max="15" step="0.005" bind:value={lg} class="flex-1" />
        <span class="font-mono w-24 text-right text-surface-200">{fmtCount(n)}</span>
      </label>
      {#each presets as p}
        <button
          class="badge preset-tonal hover:preset-filled-primary-500 transition-colors"
          onclick={() => (lg = p.v)}>{p.label}</button
        >
      {/each}
    </div>

    <div class="grid sm:grid-cols-2 gap-x-6 gap-y-1 text-xs text-surface-400">
      <div>
        <span class="text-primary-300 font-mono">abstract</span> 1/√ln n
        <span class="font-mono text-surface-200 ml-1">{aAbs.toFixed(3)}</span>
      </div>
      <div>
        <span class="text-success-300 font-mono">Thm 1.1</span> ln ln n/√ln n
        <span class="font-mono text-surface-200 ml-1">{aMan.toFixed(3)}</span>
      </div>
      <div>
        <span class="text-warning-300 font-mono">Lean</span> (ln ln n)²/√ln n
        <span
          class="font-mono ml-1"
          class:text-success-300={aLean <= 1}
          class:text-error-400={aLean > 1}>{aLean.toFixed(3)}</span
        >
      </div>
      <div>
        brute force <span class="font-mono text-surface-200 ml-1">1.000</span>
      </div>
      <div>
        abstract time, base-2 exponent
        <span class="font-mono text-surface-200 ml-1">{(aAbs * n).toExponential(2)}</span>
      </div>
      <div>
        abstract time as a number
        <span class="font-mono text-surface-200 ml-1">{fmtLog10(lgTimeAbs)}</span>
      </div>
    </div>

    <p class="text-xs text-surface-500 leading-relaxed">
      At n = {fmtCount(n)} the abstract's exponent is {aAbs.toFixed(3)} and Theorem 1.1's is
      {aMan.toFixed(3)}; both are below 1, and neither is small. Crossing the dashed lines is not the
      same as being faster: reading the exponents off the chart says the abstract's crosses 1.308ⁿ at
      n ≈ 780, which is nonsense as a claim about running time, because the O<sub>k</sub>(1) polynomial
      and the design parameter D = r<sup>2Q</sup> hide enormous factors. The formalised exponent, on the
      other hand, is above 1 — worse than exhaustive search — until n ≈ {fmtLog10(lgLeanBelowOne)};
      Theorem 1.1's does not drop below 1.308ⁿ's until n ≈ {fmtLog10(lgManBeats1308)}, and the
      abstract's does not drop below 1.1ⁿ's until n ≈ {fmtLog10(lgAbsBeats110)}. The exponent is not
      the story at any scale anyone can run. It is the story <em>in the limit</em>, which is exactly
      what ETH is about.
    </p>
  </div>
</div>
