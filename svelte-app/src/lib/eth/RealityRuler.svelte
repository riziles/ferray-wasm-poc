<script lang="ts">
  // The exponent chart says α falls; this says where the crossings sit against
  // things that exist. One row per mark (a lollipop chart) rather than labels
  // crowded around a single axis, because the marks span 10^23 to 10^2390 and
  // would otherwise all land in the same pixel.
  //
  // The x axis is log10(log10 n): an ordinary log axis crams the interesting
  // range into the right edge.

  import { onMount } from 'svelte';
  import { INK, clear, fit } from './util';
  import {
    exponentOfBase,
    log10NForAbstractExponent,
    log10NForLeanExponent,
    log10NForManuscriptExponent,
    superscript
  } from './numerics';

  interface Mark {
    /** log10 of n. */
    lg: number;
    label: string;
    kind: 'math' | 'physical';
  }

  const lgAbstract110 = log10NForAbstractExponent(exponentOfBase(1.1));
  const lgManuscript1308 = log10NForManuscriptExponent(exponentOfBase(1.308));
  const lgLeanOne = log10NForLeanExponent(1);

  const marks: Mark[] = (
    [
      { lg: Math.log10(55), label: 'n = 55 — α falls to ½', kind: 'math' },
      { lg: Math.log10(8.9e6), label: 'n ≈ 10⁷ — α falls to ¼', kind: 'math' },
      { lg: 27, label: '10²⁷ atoms — a person', kind: 'physical' },
      { lg: 47, label: '10⁴⁷ ops — every computer, ever', kind: 'physical' },
      { lg: 50, label: '10⁵⁰ atoms — the Earth', kind: 'physical' },
      {
        lg: lgAbstract110,
        label: `n ≈ 10${superscript(Math.round(lgAbstract110))} — abstract beats 1.1ⁿ`,
        kind: 'math'
      },
      {
        lg: lgManuscript1308,
        label: `n ≈ 10${superscript(Math.round(lgManuscript1308))} — Thm 1.1 beats 1.308ⁿ`,
        kind: 'math'
      },
      { lg: 80, label: '10⁸⁰ atoms — observable universe', kind: 'physical' },
      { lg: 185, label: '10¹⁸⁵ Planck volumes — universe', kind: 'physical' },
      {
        lg: lgLeanOne,
        label: `n ≈ 10${superscript(Math.round(lgLeanOne))} — Lean beats 2ⁿ`,
        kind: 'math'
      }
    ] satisfies Mark[]
  ).sort((a, b) => a.lg - b.lg);

  let canvas: HTMLCanvasElement;
  let observer: ResizeObserver | null = null;

  const rowGap = 22;
  const topPad = 16;
  const bottomPad = 34;

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h, '#0b1020');

    const labelW = Math.min(400, Math.max(170, w * 0.52));
    const x0 = labelW + 18;
    const x1 = w - 26;
    const lo = 0.45;
    const hi = 3.45;
    const X = (lg: number) => {
      const v = Math.log10(Math.max(lg, 0.2));
      return x0 + ((v - lo) / (hi - lo)) * (x1 - x0);
    };

    // "more variables than atoms in the observable universe"
    ctx.fillStyle = 'rgba(248,113,113,0.06)';
    ctx.fillRect(X(80), topPad, x1 - X(80) + 10, h - topPad - bottomPad);

    // decade guides
    ctx.font = '10px ui-monospace, monospace';
    for (const lg of [3, 10, 30, 100, 300, 1000, 2400]) {
      const x = X(lg);
      ctx.strokeStyle = 'rgba(148,163,184,0.10)';
      ctx.lineWidth = 1;
      ctx.beginPath();
      ctx.moveTo(x, topPad);
      ctx.lineTo(x, h - bottomPad);
      ctx.stroke();
      ctx.fillStyle = INK.dim;
      ctx.textAlign = 'center';
      ctx.fillText(`10${superscript(lg)}`, x, h - bottomPad + 14);
      ctx.textAlign = 'left';
    }

    // baseline
    ctx.strokeStyle = 'rgba(148,163,184,0.35)';
    ctx.beginPath();
    ctx.moveTo(x0 - 8, h - bottomPad);
    ctx.lineTo(x1 + 10, h - bottomPad);
    ctx.stroke();

    marks.forEach((mk, i) => {
      const y = topPad + 8 + i * rowGap;
      // keep the dot clear of the label column, even for the smallest n
      const x = Math.max(X(mk.lg), labelW + 14);
      const colour = mk.kind === 'math' ? INK.primary : 'rgba(148,163,184,0.6)';

      // leader from the label column to the dot
      ctx.strokeStyle = mk.kind === 'math' ? 'rgba(56,189,248,0.35)' : 'rgba(148,163,184,0.18)';
      ctx.lineWidth = 1;
      if (x > labelW + 16) {
        ctx.beginPath();
        ctx.moveTo(labelW + 6, y - 3);
        ctx.lineTo(x, y - 3);
        ctx.stroke();
      }

      ctx.beginPath();
      ctx.arc(x, y - 3, mk.kind === 'math' ? 3.4 : 2.6, 0, Math.PI * 2);
      ctx.fillStyle = colour;
      ctx.fill();

      // math marks also get a vertical drop to the baseline, so their position reads
      if (mk.kind === 'math') {
        ctx.strokeStyle = 'rgba(56,189,248,0.16)';
        ctx.beginPath();
        ctx.moveTo(x, y - 3);
        ctx.lineTo(x, h - bottomPad);
        ctx.stroke();
      }

      ctx.fillStyle = mk.kind === 'math' ? '#bae6fd' : 'rgba(148,163,184,0.9)';
      ctx.font = '11px ui-monospace, monospace';
      ctx.textAlign = 'right';
      ctx.fillText(mk.label, labelW, y);
      ctx.textAlign = 'left';
    });

    // region label
    ctx.save();
    ctx.translate(x1 - 4, topPad + 6);
    ctx.fillStyle = 'rgba(248,113,113,0.55)';
    ctx.font = '10px ui-monospace, monospace';
    ctx.textAlign = 'right';
    ctx.fillText('more variables than atoms in the universe →', 0, 0);
    ctx.restore();

    ctx.fillStyle = INK.text;
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText('n, on a log₁₀ log₁₀ scale →', 10, h - 6);
  }

  onMount(() => {
    draw();
    observer = new ResizeObserver(() => draw());
    if (canvas) observer.observe(canvas);
    return () => observer?.disconnect();
  });
</script>

<div class="rounded-lg border border-surface-700 bg-surface-950/40 overflow-hidden">
  <canvas
    bind:this={canvas}
    class="block w-full h-[300px]"
    aria-label="Marks for the paper's crossover points and for physical quantities, on a log-log axis of n"
  ></canvas>
  <div class="p-3 border-t border-surface-800">
    <p class="text-xs text-surface-500 leading-relaxed">
      The axis is <span class="font-mono">log₁₀ log₁₀ n</span>, because these marks span 10²³ to 10²³⁹⁰ and
      an ordinary log axis would stack them on the last pixel. Cyan marks are the paper's crossovers;
      grey ones are physical quantities, and they are here to be reassuringly wrong. Every computer that
      has ever run has performed at most about 10⁴⁷ operations (10³⁰ per second since the Big Bang), and
      the machine-checked bound does not beat exhaustive search until n is larger than the number of
      Planck volumes in the observable universe. None of that refutes the paper — the claim it refutes is
      asymptotic — but it is what “subexponential” looks like once the o is measured against something.
    </p>
  </div>
</div>
