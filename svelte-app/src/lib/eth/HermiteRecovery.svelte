<script lang="ts">
  // Step 4 of the mechanism, made checkable: the count is H(0), and the only
  // numbers the algorithm can compute are the Taylor coefficients of H at
  // t = 1…Q. Whether that determines H(0) is a statement about linear algebra,
  // so this panel does the linear algebra — exactly, over the rationals — and
  // shows both the weights that recover the count and the polynomial that
  // destroys them when there are too few numbers.

  import { onMount } from 'svelte';
  import { INK, clear, fit } from './util';
  import {
    applyWeights,
    dataOf,
    evalPoly,
    fracToText,
    randomPoly,
    recoveryWeights,
    vanishingPolynomial,
    vanishingValue
  } from './hermite';

  let d = $state(6);
  let Q = $state(3);
  let J = $state(3);

  const weights = $derived(recoveryWeights(d, Q, J));
  const determined = $derived(weights.determined);

  // Two unrelated polynomials of the same degree bound: the weights are
  // supposed to recover the constant term of either one.
  const polyA = $derived(randomPoly(d, 17));
  const polyB = $derived(randomPoly(d, 41));
  const dataA = $derived(dataOf(polyA, Q, J));
  const dataB = $derived(dataOf(polyB, Q, J));
  const recoveredA = $derived(weights.lambda ? applyWeights(weights.lambda, dataA) : null);
  const recoveredB = $derived(weights.lambda ? applyWeights(weights.lambda, dataB) : null);
  const trueA = $derived(polyA[0]);
  const trueB = $derived(polyB[0]);

  // The witness: Π(T − t)^J vanishes, with all J−1 derivatives, at every sample
  // point, and does not vanish at 0.
  const witness = $derived(vanishingPolynomial(Q, J));
  const witnessData = $derived(dataOf(witness, Q, J));
  const witnessAtZero = $derived(vanishingValue(Q, J));
  const witnessFits = $derived(witness.length - 1 <= d);
  const witnessClean = $derived(witnessData.every((x) => x === 0n));

  const paperJ = $derived(Math.floor(d / Q) + 1);

  const ordinal = (j: number) => (j === 1 ? '1st' : j === 2 ? '2nd' : j === 3 ? '3rd' : `${j}th`);

  let canvas: HTMLCanvasElement;
  let observer: ResizeObserver | null = null;

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h, '#0b1020');

    const xMax = Q + 0.75;
    const L = 40;
    const R = 46;
    const T = 16;
    const B = 28;
    const samples = 260;

    // The curves to draw: one in the determined case, two in the failing case.
    const curves: { coeffs: bigint[]; colour: string; dash: number[] }[] = [
      { coeffs: polyA, colour: INK.primary, dash: [] }
    ];
    if (!determined && witnessFits) {
      const sum = polyA.slice();
      for (let i = 0; i < witness.length; i++) {
        sum[i] = (sum[i] ?? 0n) + witness[i];
      }
      curves.push({ coeffs: sum, colour: INK.warn, dash: [6, 4] });
    }

    const pts = curves.map((c) => {
      const ys: number[] = [];
      for (let i = 0; i <= samples; i++) ys.push(evalPoly(c.coeffs, (xMax * i) / samples));
      return ys;
    });
    const all = pts.flat().concat(dataOf(polyA, Q, 1).map(Number));
    let yMin = Math.min(...all);
    let yMax = Math.max(...all);
    if (yMax - yMin < 1e-9) {
      yMax += 1;
      yMin -= 1;
    }
    const pad = (yMax - yMin) * 0.12;
    yMin -= pad;
    yMax += pad;

    const X = (x: number) => L + (x / xMax) * (w - L - R);
    const Y = (y: number) => h - B - ((y - yMin) / (yMax - yMin)) * (h - T - B);

    // axes
    ctx.strokeStyle = 'rgba(148,163,184,0.3)';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(L, h - B);
    ctx.lineTo(w - R, h - B);
    ctx.stroke();
    if (yMin < 0 && yMax > 0) {
      ctx.strokeStyle = 'rgba(148,163,184,0.16)';
      ctx.beginPath();
      ctx.moveTo(L, Y(0));
      ctx.lineTo(w - R, Y(0));
      ctx.stroke();
    }

    // T = 0 is the point nobody can evaluate
    ctx.strokeStyle = 'rgba(248,113,113,0.5)';
    ctx.setLineDash([3, 4]);
    ctx.beginPath();
    ctx.moveTo(X(0), T);
    ctx.lineTo(X(0), h - B);
    ctx.stroke();
    ctx.setLineDash([]);
    ctx.fillStyle = 'rgba(248,113,113,0.85)';
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText('T = 0', X(0) + 4, T + 8);

    // sample points: the data the algorithm actually has
    for (let t = 1; t <= Q; t++) {
      const y = evalPoly(polyA, t);
      ctx.beginPath();
      ctx.arc(X(t), Y(y), 3, 0, Math.PI * 2);
      ctx.fillStyle = 'rgba(226,232,240,0.9)';
      ctx.fill();
      ctx.fillStyle = INK.dim;
      ctx.fillText(`${t}`, X(t) - 3, h - B + 13);
    }

    // the curves
    curves.forEach((c, ci) => {
      ctx.beginPath();
      for (let i = 0; i <= samples; i++) {
        const x = (xMax * i) / samples;
        const y = pts[ci][i];
        if (i === 0) ctx.moveTo(X(x), Y(y));
        else ctx.lineTo(X(x), Y(y));
      }
      ctx.strokeStyle = c.colour;
      ctx.lineWidth = 2;
      ctx.setLineDash(c.dash);
      ctx.stroke();
      ctx.setLineDash([]);
    });

    // the value(s) at zero, with the numbers printed — the gap is small on a
    // scale set by the polynomial's values, so the label carries it
    const yA = Y(Number(polyA[0]));
    ctx.beginPath();
    ctx.arc(X(0), yA, 3.5, 0, Math.PI * 2);
    ctx.fillStyle = INK.primary;
    ctx.fill();
    if (!determined && witnessFits) {
      const yB = Y(Number(polyA[0]) + Number(witnessAtZero));
      ctx.strokeStyle = 'rgba(251,191,36,0.7)';
      ctx.setLineDash([2, 2]);
      ctx.beginPath();
      ctx.moveTo(X(0), yA);
      ctx.lineTo(X(0), yB);
      ctx.stroke();
      ctx.setLineDash([]);
      ctx.beginPath();
      ctx.arc(X(0), yB, 3.5, 0, Math.PI * 2);
      ctx.fillStyle = INK.warn;
      ctx.fill();
      ctx.fillStyle = INK.warn;
      ctx.fillText(`${Number(polyA[0]) + Number(witnessAtZero)}`, X(0) + 6, yB + 3);
      ctx.fillStyle = INK.primary;
      ctx.fillText(`${Number(polyA[0])}`, X(0) + 6, yA + 3);
      ctx.fillStyle = 'rgba(251,191,36,0.9)';
      ctx.fillText('two counts', X(0) + 6, Math.max(yA, yB) + 14);
    } else {
      ctx.fillStyle = INK.primary;
      ctx.fillText(`${Number(polyA[0])}`, X(0) + 6, yA + 3);
      ctx.fillStyle = 'rgba(56,189,248,0.9)';
      ctx.fillText('the count', X(0) + 6, yA - 10);
    }

    ctx.fillStyle = INK.text;
    ctx.fillText('T', w - R + 6, h - B + 12);
  }

  onMount(() => {
    draw();
    observer = new ResizeObserver(() => draw());
    if (canvas) observer.observe(canvas);
    return () => observer?.disconnect();
  });

  $effect(() => {
    d;
    Q;
    J;
    draw();
  });
</script>

<div class="rounded-lg border border-surface-700 bg-surface-950/40">
  <div class="p-3 border-b border-surface-800 space-y-3">
    <div class="grid sm:grid-cols-3 gap-3">
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap">degree bound d</span>
        <input type="range" min="2" max="12" step="1" bind:value={d} class="flex-1" />
        <span class="font-mono w-5 text-right">{d}</span>
      </label>
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap">points Q</span>
        <input type="range" min="1" max="6" step="1" bind:value={Q} class="flex-1" />
        <span class="font-mono w-5 text-right">{Q}</span>
      </label>
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap">Taylor terms J</span>
        <input type="range" min="1" max="4" step="1" bind:value={J} class="flex-1" />
        <span class="font-mono w-5 text-right">{J}</span>
      </label>
    </div>
    <div class="flex flex-wrap items-center gap-2 text-xs">
      <button
        class="badge preset-tonal hover:preset-filled-primary-500 transition-colors"
        onclick={() => (J = paperJ)}>paper's choice: J = ⌊d/Q⌋+1 = {paperJ}</button
      >
      <span class="text-surface-500">
        data <span class="font-mono text-surface-300">{weights.dataCount}</span> numbers ·
        unknowns <span class="font-mono text-surface-300">{weights.unknowns}</span>
      </span>
      <span
        class="badge"
        class:preset-filled-success-500={determined}
        class:preset-filled-error-500={!determined}
      >
        {determined ? '✓ H(0) is determined' : '✗ H(0) is not determined'}
      </span>
    </div>
  </div>

  <div class="grid md:grid-cols-[1fr_1fr] divide-y md:divide-y-0 md:divide-x divide-surface-800">
    <div class="p-3 space-y-2">
      <canvas bind:this={canvas} class="block w-full h-[220px] rounded bg-[#0b1020]"></canvas>
      <div class="text-[10px] text-surface-500 text-center font-mono">
        white dots: the QJ numbers the algorithm has · dashed red: the point it wants
      </div>
    </div>

    <div class="p-3 space-y-3">
      {#if determined}
        <div class="text-[11px] uppercase tracking-wider text-surface-500">
          the weights: H(0) = Σ λ · datum
        </div>
        <div class="space-y-1 font-mono text-[11px]">
          {#each Array.from({ length: Q }, (_, i) => i + 1) as t}
            <div class="flex flex-wrap items-baseline gap-x-3 gap-y-0.5">
              <span class="text-surface-500 w-9 shrink-0">t={t}</span>
              {#each Array.from({ length: J }, (_, j) => j) as j}
                <span class="text-surface-600 text-[10px]">
                  {j === 0 ? 'value' : `${ordinal(j)} deriv`}
                </span>
                <span
                  class="text-surface-200"
                  title={fracToText((weights.lambda ?? [])[(t - 1) * J + j] ?? { n: 0n, d: 1n })}
                >
                  {(weights.lambda ?? [])[(t - 1) * J + j]
                    ? fracToText((weights.lambda ?? [])[(t - 1) * J + j])
                    : '—'}
                </span>
              {/each}
            </div>
          {/each}
        </div>
        <div class="rounded border border-surface-800 bg-surface-900/40 p-2 space-y-1 text-[11px]">
          <div class="text-surface-400">
            H<sub>A</sub>(0) = {trueA} → recovered
            <span class="font-mono text-success-300">{recoveredA ? fracToText(recoveredA) : '—'}</span>
          </div>
          <div class="text-surface-400">
            H<sub>B</sub>(0) = {trueB} → recovered
            <span class="font-mono text-success-300">{recoveredB ? fracToText(recoveredB) : '—'}</span>
          </div>
          <div class="text-surface-500 leading-snug pt-1">
            Two unrelated polynomials of degree ≤ {d}, the same weights, both counts exact. The weights
            depend on d, Q, J and nothing else — which is why the algorithm can compute #F without ever
            forming H.
          </div>
        </div>
      {:else}
        <div class="text-[11px] uppercase tracking-wider text-surface-500">
          the counterexample
        </div>
        <div class="text-[11px] text-surface-300 leading-snug">
          With only <span class="font-mono">{weights.dataCount}</span> numbers and
          <span class="font-mono">{weights.unknowns}</span> unknowns, the polynomial
        </div>
        <div class="rounded border border-surface-800 bg-surface-950 p-2 font-mono text-[11px] text-warning-300 overflow-x-auto">
          W(T) = Π<sub>t=1..{Q}</sub> (T − t)<sup>{J}</sup>
        </div>
        <div class="text-[11px] text-surface-400 leading-snug">
          has degree <span class="font-mono">{witness.length - 1}</span>
          {#if witnessFits}(≤ d ✓){:else}(&gt; d, outside the budget){/if} and vanishes at every sample
          point{#if J > 1}, along with its first {J - 1} derivative{J - 1 === 1 ? '' : 's'}{/if}. So H and
          H + W give identical data — checked here:
          <span class="font-mono {witnessClean ? 'text-success-300' : 'text-error-400'}">
            {witnessClean ? 'W contributes 0 to every datum ✓' : 'W does not vanish — bug'}
          </span>
          — while at zero they differ by
          <span class="font-mono text-warning-300">{witnessAtZero}</span>. That gap is small next to the
          polynomial's own values, which is why it is printed on the plot rather than left to the eye.
        </div>
        <div class="text-[11px] text-surface-500 leading-snug">
          No cleverness recovers the count: every additional sample point multiplies W's degree by J, so
          you need <span class="font-mono">QJ &gt; d</span> before the obstruction disappears. That is
          exactly the inequality the paper arranges with q = ⌊d<sub>H</sub>/Q⌋ and J = q+1.
        </div>
      {/if}
    </div>
  </div>

  <div class="p-3 border-t border-surface-800">
    <p class="text-xs text-surface-500 leading-relaxed">
      H is the count as a polynomial in the interpolation parameter: deg H ≤ 3nR in the paper's
      bookkeeping. The algorithm cannot evaluate it at zero — the replacement forms are built to be
      summable at t = 1, …, Q — so it computes QJ Taylor coefficients and takes a fixed rational
      combination of them. Here d, Q and J are toy values, the arithmetic is exact, and the weights are
      solved from the constraint matrix M<sub>(t,j),k</sub> = C(k,j)·t<sup>k−j</sup> by Gauss–Jordan over
      the rationals. Move a slider past the inequality and the recovery fails in the way the paper is
      careful to avoid.
    </p>
  </div>
</div>
