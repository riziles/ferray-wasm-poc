<script lang="ts">
  import { onMount } from 'svelte';
  import { fit, clear, clamp } from './util';

  // ── two-loop running coupling, integrated numerically ───────────────────
  const MZ = 91.1876;
  const ALPHA_MZ = 0.1179;
  const Q_MIN = 0.4;
  const Q_MAX = 2000;

  const nfOf = (q: number) => (q > 172.7 ? 6 : q > 4.18 ? 5 : q > 1.27 ? 4 : 3);

  function beta(a: number, nf: number) {
    const b0 = 11 - (2 / 3) * nf;
    const b1 = 102 - (38 / 3) * nf;
    // dα/dlnQ
    return -(b0 * a * a) / (2 * Math.PI) - (b1 * a * a * a) / (8 * Math.PI * Math.PI);
  }

  /** integrate dα/dlnQ from a reference point */
  function run(fromQ: number, toQ: number, a0: number) {
    const N = 4000;
    const dl = Math.log(toQ / fromQ) / N;
    let a = a0;
    let q = fromQ;
    for (let i = 0; i < N; i++) {
      const nf = nfOf(q);
      const k1 = beta(a, nf);
      const k2 = beta(a + 0.5 * dl * k1, nf);
      a += dl * k2;
      q *= Math.exp(dl);
      if (a <= 0 || a > 40) break;
    }
    return a;
  }

  const table: { q: number; a: number }[] = [];
  onMount(() => {});
  {
    const N = 260;
    for (let i = 0; i <= N; i++) {
      const q = Q_MIN * Math.pow(Q_MAX / Q_MIN, i / N);
      const a = run(MZ, q, ALPHA_MZ);
      table.push({ q, a });
    }
  }

  function alphaAt(q: number) {
    if (q <= table[0].q) return table[0].a;
    if (q >= table[table.length - 1].q) return table[table.length - 1].a;
    // log-interpolate
    const lq = Math.log(q);
    let lo = 0;
    let hi = table.length - 1;
    while (hi - lo > 1) {
      const mid = (lo + hi) >> 1;
      if (Math.log(table[mid].q) < lq) lo = mid;
      else hi = mid;
    }
    const t = (lq - Math.log(table[lo].q)) / (Math.log(table[hi].q) - Math.log(table[lo].q));
    return table[lo].a * (1 - t) + table[hi].a * t;
  }

  let logQ = $state(Math.log10(10)); // 10 GeV
  const Q = $derived(Math.pow(10, logQ));
  const alpha = $derived(alphaAt(Q));

  let canvas: HTMLCanvasElement;

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h);

    const padL = 54;
    const padR = 16;
    const padT = 18;
    const padB = 34;
    const plotW = w - padL - padR;
    const plotH = h - padT - padB;

    const xOf = (q: number) =>
      padL + (plotW * (Math.log10(q) - Math.log10(Q_MIN))) / (Math.log10(Q_MAX) - Math.log10(Q_MIN));
    const yOf = (a: number) => padT + plotH * (1 - clamp(a, 0, 1.6) / 1.6);

    // grid
    ctx.strokeStyle = 'rgba(51,65,85,0.7)';
    ctx.fillStyle = 'rgba(100,116,139,0.9)';
    ctx.font = '10px ui-monospace, monospace';
    ctx.lineWidth = 1;
    for (const q of [0.5, 1, 10, 100, 1000]) {
      const x = xOf(q);
      ctx.beginPath();
      ctx.moveTo(x, padT);
      ctx.lineTo(x, padT + plotH);
      ctx.stroke();
      ctx.textAlign = 'center';
      ctx.fillText(q >= 1 ? `${q} GeV` : `${q}`, x, h - padB + 16);
    }
    for (const a of [0, 0.4, 0.8, 1.2, 1.6]) {
      const y = yOf(a);
      ctx.beginPath();
      ctx.moveTo(padL, y);
      ctx.lineTo(padL + plotW, y);
      ctx.stroke();
      ctx.textAlign = 'right';
      ctx.fillText(a.toFixed(1), padL - 8, y + 3);
    }

    // "perturbation theory is dead" region: alpha > 0.3
    const yDead = yOf(0.3);
    ctx.fillStyle = 'rgba(239,68,68,0.08)';
    ctx.fillRect(padL, yDead, plotW, padT + plotH - yDead);
    ctx.fillStyle = 'rgba(239,68,68,0.65)';
    ctx.textAlign = 'left';
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText('α_s > 0.3 — Feynman diagrams stop helping', padL + 8, padT + plotH - 10);

    // the curve
    ctx.beginPath();
    let started = false;
    for (const p of table) {
      if (p.a > 1.6) continue;
      const x = xOf(p.q);
      const y = yOf(p.a);
      if (!started) {
        ctx.moveTo(x, y);
        started = true;
      } else ctx.lineTo(x, y);
    }
    ctx.strokeStyle = '#38bdf8';
    ctx.lineWidth = 2.5;
    ctx.shadowColor = 'rgba(56,189,248,0.6)';
    ctx.shadowBlur = 10;
    ctx.stroke();
    ctx.shadowBlur = 0;

    // marker for the selected scale
    const mx = xOf(Q);
    const my = yOf(alpha);
    ctx.beginPath();
    ctx.arc(mx, my, 6, 0, Math.PI * 2);
    ctx.fillStyle = '#fbbf24';
    ctx.fill();
    ctx.strokeStyle = '#0b1020';
    ctx.lineWidth = 2;
    ctx.stroke();

    ctx.setLineDash([4, 4]);
    ctx.beginPath();
    ctx.moveTo(mx, my);
    ctx.lineTo(mx, padT + plotH);
    ctx.strokeStyle = 'rgba(251,191,36,0.4)';
    ctx.lineWidth = 1;
    ctx.stroke();
    ctx.setLineDash([]);

    // landmark: M_Z
    const zx = xOf(MZ);
    ctx.beginPath();
    ctx.arc(zx, yOf(ALPHA_MZ), 3.5, 0, Math.PI * 2);
    ctx.fillStyle = 'rgba(226,232,240,0.9)';
    ctx.fill();
    ctx.fillStyle = 'rgba(148,163,184,0.9)';
    ctx.textAlign = 'center';
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText('Z pole: 0.1179', zx, yOf(ALPHA_MZ) - 10);

    ctx.fillStyle = 'rgba(148,163,184,0.9)';
    ctx.textAlign = 'left';
    ctx.fillText('α_s(Q)', padL + 6, padT + 12);
    ctx.textAlign = 'right';
    ctx.fillText('momentum transfer Q →', padL + plotW, h - 6);
  }

  onMount(() => {
    const ro = new ResizeObserver(() => draw());
    ro.observe(canvas);
    draw();
    return () => ro.disconnect();
  });

  $effect(() => {
    void Q;
    void alpha;
    draw();
  });

  const verdict = $derived(
    alpha > 1.2
      ? 'the coupling is bigger than 1. QCD here is not weakly interacting; it is not even approximately weakly interacting. Perturbation theory has failed and confinement is in charge.'
      : alpha > 0.3
        ? 'strong. Feynman diagrams are a rumour. This is the regime where the whole machinery of particle physics stops working.'
        : alpha > 0.1
          ? 'weakish. Quarks behave almost like free particles — which is why hard scattering experiments see them as independent points.'
          : 'tiny. At this scale the strong force is embarrassed by the electroweak force. Quarks fall apart laughing.'
  );
</script>

<div class="card card-demo p-6 space-y-4">
  <div class="flex flex-wrap gap-3 items-center">
    <label class="label flex-1 min-w-50">
      <span class="text-xs">probe scale Q = {Q < 10 ? Q.toFixed(2) : Q.toFixed(0)} GeV</span>
      <input type="range" class="input" min={Math.log10(Q_MIN)} max={Math.log10(Q_MAX)} step="0.01" bind:value={logQ} />
    </label>
    <span class="badge preset-tonal-primary font-mono">α_s = {alpha.toFixed(3)}</span>
    <span class="badge preset-tonal">{alpha > 0.3 ? 'non-perturbative' : 'perturbative'}</span>
  </div>

  <canvas bind:this={canvas} class="w-full rounded border border-surface-700" style="height:320px"
  ></canvas>

  <p class="text-xs text-surface-400">
    At Q = {Q.toFixed(1)} GeV, α<sub>s</sub> = {alpha.toFixed(3)} — {verdict}
  </p>

  <div class="grid sm:grid-cols-2 gap-3 text-xs">
    <div class="bg-surface-800/60 rounded p-3 space-y-1">
      <div class="font-semibold text-surface-300">The coupling runs — the wrong way</div>
      <div class="text-surface-500">
        Zoom in (large Q, short distance) and the strong force gets <em>weaker</em>. Pull back and it
        gets <em>stronger</em>, without limit. This is <strong>asymptotic freedom</strong>: quarks
        inside a proton rattle around as if they were free — while being absolutely unremovable.
      </div>
    </div>
    <div class="bg-surface-800/60 rounded p-3 space-y-1">
      <div class="font-semibold text-surface-300">Where the running comes from</div>
      <div class="text-surface-500">
        β<sub>0</sub> = 11 − ⅔n<sub>f</sub>. The “11” is gluon loops (anti-screening); the
        “−⅔n<sub>f</sub>” is quark loops (screening). With 17 quark flavours the 11 would lose, β
        would flip sign, and confinement would evaporate. We have 6. The universe is on the edge of a
        cliff it does not know about.
      </div>
    </div>
  </div>
</div>
