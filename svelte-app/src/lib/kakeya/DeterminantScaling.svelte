<script lang="ts">
  // The determinant equation of Section 4, in the scalar model that carries the
  // scaling. On a spatial fibre the potential u solves
  //
  //     det H(λ|Φ|² + u) = λμ,   u = 0 on the boundary,
  //
  // where the background λ|Φ|² supplies exactly ONE direction of curvature,
  // of size β ≍ λ. Writing α for the size of the Hessian of u, the determinant
  // is (β + α)α^(n-1) — and that single eigenvalue is what moves the exponent
  // from 1/n (plain Monge–Ampère) to 1/(n-1), which doubled gives 2n/(n-1).

  import { onMount } from 'svelte';
  import { criticalP } from './exponents';
  import { INK, clear, fit, fmt } from './util';

  let n = $state(3);
  let canvas: HTMLCanvasElement;
  let observer: ResizeObserver | null = null;

  const MU = { lo: -6, hi: 3 }; // log10 of the density

  /** solve (beta + a) a^(n-1) = mu for a > 0 (left side is increasing). */
  function alphaOf(mu: number, beta = 1): number {
    let lo = 1e-12;
    let hi = Math.max(1, Math.pow(mu, 1 / (n - 1)), Math.pow(mu, 1 / n)) * 4;
    for (let i = 0; i < 200; i++) {
      const mid = Math.sqrt(lo * hi);
      const f = (beta + mid) * Math.pow(mid, n - 1) - mu;
      if (f > 0) hi = mid;
      else lo = mid;
    }
    return Math.sqrt(lo * hi);
  }

  const samples = $derived.by(() => {
    const out: [number, number][] = [];
    for (let i = 0; i <= 220; i++) {
      const lm = MU.lo + ((MU.hi - MU.lo) * i) / 220;
      const mu = Math.pow(10, lm);
      out.push([lm, Math.log10(alphaOf(mu))]);
    }
    return out;
  });

  const slopeSmall = $derived.by(() => fitSlope(samples.filter(([lm]) => lm < -2)));
  const slopeLarge = $derived.by(() => fitSlope(samples.filter(([lm]) => lm > 2)));

  function fitSlope(pts: [number, number][]) {
    if (pts.length < 2) return NaN;
    const mx = pts.reduce((a, p) => a + p[0], 0) / pts.length;
    const my = pts.reduce((a, p) => a + p[1], 0) / pts.length;
    let num = 0;
    let den = 0;
    for (const [x, y] of pts) {
      num += (x - mx) * (y - my);
      den += (x - mx) ** 2;
    }
    return num / den;
  }

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h, '#0b1020');
    const ylo = -6.5;
    const yhi = 1.5;
    const X = (lm: number) => 42 + ((lm - MU.lo) / (MU.hi - MU.lo)) * (w - 56);
    const Y = (la: number) => h - 26 - ((la - ylo) / (yhi - ylo)) * (h - 42);

    // axis frame
    ctx.strokeStyle = 'rgba(148,163,184,0.3)';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(42, h - 26);
    ctx.lineTo(w - 8, h - 26);
    ctx.moveTo(42, h - 26);
    ctx.lineTo(42, 8);
    ctx.stroke();

    // Reference slopes, anchored to the computed curve at either end so the
    // comparison is visible rather than approximate.
    const anchor = (lm: number, slope: number, colour: string, label: string, dy = -3) => {
      const la = Math.log10(alphaOf(Math.pow(10, lm)));
      ctx.beginPath();
      ctx.moveTo(X(lm), Y(la));
      ctx.lineTo(X(MU.hi), Y(la + slope * (MU.hi - lm)));
      ctx.strokeStyle = colour;
      ctx.setLineDash([4, 4]);
      ctx.lineWidth = 1;
      ctx.stroke();
      ctx.setLineDash([]);
      ctx.fillStyle = colour;
      ctx.font = '9px ui-monospace, monospace';
      ctx.fillText(label, X(lm + 0.35), Y(la + slope * 0.35) + dy);
    };
    anchor(MU.lo, 1 / (n - 1), 'rgba(56,189,248,0.7)', `1/(n−1) = ${(1 / (n - 1)).toFixed(3)}`);
    anchor(1, 1 / n, 'rgba(148,163,184,0.7)', `1/n = ${(1 / n).toFixed(3)}`, 12);

    // the computed curve
    ctx.beginPath();
    samples.forEach(([lm, la], i) => {
      if (i === 0) ctx.moveTo(X(lm), Y(la));
      else ctx.lineTo(X(lm), Y(la));
    });
    ctx.strokeStyle = 'rgba(254,240,138,0.95)';
    ctx.lineWidth = 2;
    ctx.stroke();

    // the region the construction uses (alpha small compared with the background)
    ctx.fillStyle = 'rgba(56,189,248,0.06)';
    ctx.fillRect(X(MU.lo), 8, X(-1) - X(MU.lo), h - 34);
    ctx.fillStyle = 'rgba(148,163,184,0.7)';
    ctx.font = '9px ui-monospace, monospace';
    ctx.fillText('regime of the construction', X(MU.lo) + 6, 34);

    ctx.fillStyle = INK.text;
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText('log α', 6, 12);
    ctx.fillText('log μ →', w - 52, 12);
    ctx.fillText('−6', 34, h - 14);
    ctx.fillText('3', w - 18, h - 14);
  }

  onMount(() => {
    draw();
    observer = new ResizeObserver(() => draw());
    if (canvas) observer.observe(canvas);
    return () => observer?.disconnect();
  });

  $effect(() => {
    n;
    draw();
  });

  const pc = $derived(criticalP(n));
</script>

<div class="rounded-lg border border-surface-700 bg-surface-950/40 overflow-hidden">
  <canvas bind:this={canvas} class="block w-full h-[280px]"></canvas>
  <div class="p-3 border-t border-surface-800 space-y-3">
    <label class="flex items-center gap-2 text-xs">
      <span class="text-surface-400 whitespace-nowrap w-14">space dim n</span>
      <input type="range" min="2" max="6" step="1" bind:value={n} class="flex-1" />
      <span class="font-mono w-4 text-right">{n}</span>
    </label>
    <div class="grid sm:grid-cols-2 gap-x-6 gap-y-1 text-xs text-surface-400">
      <div>
        fitted slope at small μ (the background dominates)
        <span class="font-mono text-primary-300">{isNaN(slopeSmall) ? '—' : slopeSmall.toFixed(3)}</span>
        <span class="text-surface-600">vs 1/(n−1) = {(1 / (n - 1)).toFixed(3)}</span>
      </div>
      <div>
        fitted slope at large μ (plain Monge–Ampère)
        <span class="font-mono text-surface-300">{isNaN(slopeLarge) ? '—' : slopeLarge.toFixed(3)}</span>
        <span class="text-surface-600">vs 1/n = {(1 / n).toFixed(3)}</span>
      </div>
    </div>
    <div class="rounded border border-surface-700 bg-surface-900/60 p-3 font-mono text-[11px] text-surface-400 space-y-1">
      <div>μ ~ (wave)² <span class="text-surface-600">← the current's time component</span></div>
      <div>depth (−u) ~ α R² <span class="text-surface-600">← slope 1/(n−1) = {fmt(1 / (n - 1), 3)}</span></div>
      <div>∫(−u)μ ~ μ^(1 + 1/(n−1)) = μ^{fmt(n / (n - 1), 3)} <span class="text-surface-600">← slope n/(n−1)</span></div>
      <div>
        so the wave exponent is 2 × {fmt(n / (n - 1), 3)} = <span class="text-warning-300">{fmt(pc, 3)} = 2n/(n−1)</span>
      </div>
    </div>
    <p class="text-xs text-surface-500 leading-relaxed">
      The dark-blue band is where the construction lives: the Hessian of the potential is small
      against the one background eigenvalue, so the determinant is β·α<sup>n−1</sup> and the depth
      scales as μ<sup>1/(n−1)</sup>. Drop the background eigenvalue and you get ordinary
      Monge–Ampère, μ<sup>1/n</sup> — an exponent too small to give p<sub>c</sub>. The rank-one
      background is doing real work.
    </p>
  </div>
</div>
