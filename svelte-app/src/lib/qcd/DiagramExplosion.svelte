<script lang="ts">
  import { onMount } from 'svelte';
  import { fit, clear, clamp } from './util';

  // Dyson's argument, in cartoon form: the perturbative series in QCD is
  // asymptotic. Diagram counts grow like n!, so the terms α^n·n! eventually
  // grow. The best you can do is truncate near n ≈ 1/α and accept an error
  // of order e^(−1/α). Beyond that, "more precision" means "less accuracy".

  let alpha = $state(0.12);
  let canvas: HTMLCanvasElement;
  let raf = 0;

  // term_n ∝ α_s^n · n!  (schematically; the true coefficients are worse)
  function term(n: number) {
    let f = 1;
    for (let k = 2; k <= n; k++) f *= k;
    return Math.pow(alpha, n) * f;
  }
  function partial(n: number) {
    let s = 0;
    let f = 1;
    for (let k = 1; k <= n; k++) {
      f *= k;
      s += Math.pow(alpha, k) * f;
    }
    return s;
  }

  const NMAX = 40;
  const terms = $derived(Array.from({ length: NMAX + 1 }, (_, n) => term(n)));
  const sums = $derived(Array.from({ length: NMAX + 1 }, (_, n) => partial(n)));
  const nOpt = $derived(clamp(Math.round(1 / alpha), 1, NMAX));
  const best = $derived(partials()[nOpt]);
  const err = $derived(Math.exp(-1 / alpha));

  function partials() {
    return sums;
  }

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h);

    const padL = 56;
    const padR = 20;
    const padT = 22;
    const padB = 36;
    const plotW = w - padL - padR;
    const plotH = h - padT - padB;

    const xOf = (n: number) => padL + (plotW * n) / NMAX;

    // log10 of |term| — the whole point is that terms shrink, then explode
    const lg = (v: number) => Math.log10(Math.max(1e-14, Math.abs(v)));
    const LO = -14;
    const HI = 12;
    const yOf = (v: number) => padT + plotH * (1 - (clamp(lg(v), LO, HI) - LO) / (HI - LO));

    // grid
    ctx.strokeStyle = 'rgba(51,65,85,0.6)';
    ctx.fillStyle = 'rgba(100,116,139,0.9)';
    ctx.font = '10px ui-monospace, monospace';
    ctx.lineWidth = 1;
    for (let n = 0; n <= NMAX; n += 5) {
      const x = xOf(n);
      ctx.beginPath();
      ctx.moveTo(x, padT);
      ctx.lineTo(x, padT + plotH);
      ctx.stroke();
      ctx.textAlign = 'center';
      ctx.fillText(String(n), x, h - padB + 16);
    }
    for (let e = -12; e <= 12; e += 4) {
      const y = padT + plotH * (1 - (e - LO) / (HI - LO));
      ctx.beginPath();
      ctx.moveTo(padL, y);
      ctx.lineTo(padL + plotW, y);
      ctx.stroke();
      ctx.textAlign = 'right';
      ctx.fillText(`1e${e >= 0 ? '+' : ''}${e}`, padL - 8, y + 3);
    }

    // best-truncation marker
    const bx = xOf(nOpt);
    ctx.fillStyle = 'rgba(56,189,248,0.07)';
    ctx.fillRect(padL, padT, bx - padL, plotH);
    ctx.strokeStyle = 'rgba(56,189,248,0.7)';
    ctx.setLineDash([4, 4]);
    ctx.beginPath();
    ctx.moveTo(bx, padT);
    ctx.lineTo(bx, padT + plotH);
    ctx.stroke();
    ctx.setLineDash([]);
    ctx.fillStyle = 'rgba(56,189,248,0.9)';
    ctx.textAlign = 'right';
    ctx.fillText(`stop here: n ≈ 1/α = ${nOpt}`, bx - 6, padT - 6);

    // |term_n| curve
    ctx.beginPath();
    terms.forEach((v, n) => {
      const x = xOf(n);
      const y = yOf(v);
      if (n === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    });
    ctx.strokeStyle = '#fbbf24';
    ctx.lineWidth = 2.2;
    ctx.shadowColor = 'rgba(251,191,36,0.5)';
    ctx.shadowBlur = 8;
    ctx.stroke();
    ctx.shadowBlur = 0;

    terms.forEach((v, n) => {
      const isBest = n === nOpt;
      ctx.beginPath();
      ctx.arc(xOf(n), yOf(v), isBest ? 4.5 : 1.8, 0, Math.PI * 2);
      ctx.fillStyle = isBest ? '#38bdf8' : 'rgba(251,191,36,0.85)';
      ctx.fill();
    });

    // the smallest term
    ctx.fillStyle = 'rgba(56,189,248,0.95)';
    ctx.textAlign = 'center';
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText(`smallest term ≈ ${err.toExponential(1)}`, bx, yOf(terms[nOpt]) + 18);

    ctx.textAlign = 'left';
    ctx.fillStyle = 'rgba(148,163,184,0.9)';
    ctx.fillText('size of the n-th term (log scale)', padL + 4, padT + plotH - 8);
    ctx.fillStyle = 'rgba(148,163,184,0.75)';
    ctx.textAlign = 'right';
    ctx.fillText('perturbation order n →', padL + plotW, h - 6);

    raf = requestAnimationFrame(draw);
  }

  onMount(() => {
    const ro = new ResizeObserver(() => draw());
    ro.observe(canvas);
    draw();
    return () => ro.disconnect();
  });

  $effect(() => {
    void alpha;
    void terms;
    draw();
  });
</script>

<div class="card card-demo p-6 space-y-4">
  <label class="label">
    <span class="text-xs">strong coupling α_s = {alpha.toFixed(3)}</span>
    <input type="range" class="input" min="0.04" max="0.6" step="0.005" bind:value={alpha} />
  </label>

  <canvas bind:this={canvas} class="w-full rounded border border-surface-700" style="height:300px"
  ></canvas>

  <div class="flex flex-wrap gap-2 text-xs">
    <span class="badge preset-tonal-warning font-mono">best partial sum ≈ {best.toFixed(3)}</span>
    <span class="badge preset-tonal-error font-mono">irreducible error ≈ e^(−1/α) ≈ {err.toExponential(2)}</span>
    <span class="badge preset-tonal">terms grow like n!</span>
  </div>

  <p class="text-xs text-surface-500">
    QCD's perturbative expansion is not a convergent series. It is an <em>asymptotic</em> series: the
    first few terms get better, then they turn around and explode. Adding diagrams past n ≈ 1/α makes
    your prediction <em>worse</em>. There is a hard floor on precision — of order e<sup>−1/α</sup> —
    that no amount of cleverness removes. This is Dyson's argument generalised: the series in the
    coupling would have to blow up at negative coupling (a world where like charges attract), so it
    cannot converge. The only way to compute the strong force honestly is to not use perturbation
    theory at all — which means lattice QCD, supercomputers, and Monte Carlo.
  </p>
</div>
