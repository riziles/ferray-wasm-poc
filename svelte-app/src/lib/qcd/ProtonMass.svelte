<script lang="ts">
  import { onMount } from 'svelte';
  import { fit, clear, clamp } from './util';

  const M_PROTON = 938.272; // MeV
  const M_U = 2.16;
  const M_D = 4.67;
  const VALENCE = 2 * M_U + M_D; // ≈ 8.99 MeV — the "Higgs" part

  let qcdOn = $state(true);
  let higgsOn = $state(true);

  // The QCD binding/kinetic contribution is what's left over.
  let shown = $state(M_PROTON);
  let target = $derived(
    higgsOn ? (qcdOn ? M_PROTON : VALENCE) : (qcdOn ? M_PROTON - VALENCE : 0)
  );

  let canvas: HTMLCanvasElement;
  let raf = 0;
  let lastT = 0;

  const displayed = $derived(shown);
  const quarkPart = $derived(higgsOn ? VALENCE : 0);
  const gluePart = $derived(Math.max(0, displayed - quarkPart));

  function draw(now = 0) {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h);

    // time-based easing toward target so it doesn't depend on frame rate
    const dt = lastT ? Math.min(0.05, (now - lastT) / 1000) : 0.016;
    lastT = now;
    shown += (target - shown) * (1 - Math.exp(-7 * dt));
    if (Math.abs(target - shown) < 0.05) shown = target;

    const padL = 110;
    const padR = 24;
    const barW = w - padL - padR;
    const cx = padL;
    const cy = h * 0.42;
    const barH = 34;
    const maxM = M_PROTON;
    const fracQ = clamp(quarkPart / maxM, 0, 1);
    const fracG = clamp(gluePart / maxM, 0, 1);

    // axis ticks
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillStyle = 'rgba(100,116,139,0.9)';
    ctx.textAlign = 'center';
    for (let m = 0; m <= maxM; m += 200) {
      const x = cx + (barW * m) / maxM;
      ctx.strokeStyle = 'rgba(51,65,85,0.6)';
      ctx.beginPath();
      ctx.moveTo(x, cy - 12);
      ctx.lineTo(x, cy + barH + 26);
      ctx.stroke();
      ctx.fillText(`${m}`, x, cy + barH + 40);
    }
    ctx.textAlign = 'left';
    ctx.fillStyle = 'rgba(148,163,184,0.9)';
    ctx.fillText('MeV of mass-energy', cx, cy + barH + 58);

    // the two stacked contributions
    const wq = barW * fracQ;
    const wg = barW * fracG;

    ctx.fillStyle = '#34d399';
    ctx.fillRect(cx, cy, wq, barH);
    ctx.fillStyle = '#38bdf8';
    ctx.fillRect(cx + wq, cy, wg, barH);

    ctx.strokeStyle = 'rgba(15,23,42,0.9)';
    ctx.lineWidth = 1;
    ctx.strokeRect(cx + 0.5, cy + 0.5, wq + wg, barH - 1);

    // labels
    ctx.textAlign = 'left';
    ctx.font = '11px ui-monospace, monospace';
    ctx.fillStyle = 'rgba(226,232,240,0.95)';
    ctx.fillText(`total m_p = ${shown.toFixed(1)} MeV`, cx, cy - 20);

    if (wq > 70) {
      ctx.fillStyle = 'rgba(6,78,59,0.95)';
      ctx.fillText(`Higgs: ${quarkPart.toFixed(1)} MeV`, cx + 8, cy + barH / 2 + 4);
    }
    if (wg > 130) {
      ctx.fillStyle = 'rgba(8,47,73,0.95)';
      ctx.fillText(`QCD field energy: ${gluePart.toFixed(0)} MeV`, cx + wq + 10, cy + barH / 2 + 4);
    }

    const geom = { padL, barW, cx, cy, barH, maxM, wq, wg };
    // annotation for the invisible sliver (kept between the bar and the tick row)
    if (wq < 40) {
      ctx.strokeStyle = 'rgba(52,211,153,0.9)';
      ctx.lineWidth = 1;
      ctx.beginPath();
      ctx.moveTo(cx + Math.max(wq, 2), cy + barH);
      ctx.lineTo(cx + Math.max(wq, 2), cy + barH + 12);
      ctx.stroke();
      ctx.fillStyle = 'rgba(52,211,153,0.95)';
      ctx.textAlign = 'left';
      ctx.font = '10px ui-monospace, monospace';
      ctx.fillText(`← Higgs quarks: ${quarkPart.toFixed(1)} MeV (${((100 * quarkPart) / M_PROTON).toFixed(1)}%)`, cx + Math.max(wq, 2) + 6, cy + barH + 14);
      void geom;
    }

    // body weight consequence
    const frac = shown / M_PROTON;
    const humanKg = 80 * frac;
    ctx.textAlign = 'center';
    ctx.font = '12px ui-monospace, monospace';
    ctx.fillStyle = 'rgba(226,232,240,0.9)';
    ctx.fillText(
      `if you were made of this proton: ${humanKg < 10 ? humanKg.toFixed(1) : humanKg.toFixed(0)} kg`,
      w / 2,
      h - 16
    );

    raf = requestAnimationFrame(draw);
  }

  onMount(() => {
    raf = requestAnimationFrame(draw);
    return () => cancelAnimationFrame(raf);
  });
</script>

<div class="card card-demo p-6 space-y-4">
  <div class="flex flex-wrap gap-4 items-center">
    <label class="flex items-center gap-2 cursor-pointer text-xs">
      <input type="checkbox" class="checkbox" bind:checked={higgsOn} /> Higgs coupling active
    </label>
    <label class="flex items-center gap-2 cursor-pointer text-xs">
      <input type="checkbox" class="checkbox" bind:checked={qcdOn} /> strong force active
    </label>
    <span class="badge preset-tonal font-mono">{displayed.toFixed(1)} MeV</span>
  </div>

  <canvas bind:this={canvas} class="w-full rounded border border-surface-700" style="height:220px"
  ></canvas>

  <div class="grid sm:grid-cols-2 gap-3 text-xs">
    <div class="bg-surface-800/60 rounded p-3 space-y-1">
      <div class="font-semibold text-surface-300">9 MeV vs 938 MeV</div>
      <div class="text-surface-500">
        A proton is two up quarks and a down quark. Their Higgs-given masses sum to {VALENCE.toFixed(1)}
        MeV. The proton weighs {M_PROTON.toFixed(0)} MeV. The missing
        <strong>{(M_PROTON - VALENCE).toFixed(0)} MeV — 99.04%</strong> is the energy of the gluon
        field and of the quarks' motion, converted to mass by E = mc².
      </div>
    </div>
    <div class="bg-surface-800/60 rounded p-3 space-y-1">
      <div class="font-semibold text-surface-300">Uncheck the strong force</div>
      <div class="text-surface-500">
        Turn QCD off and the proton is a 9 MeV bag of three escaping quarks. No nuclei, no atoms, no
        chemistry, no you. The Higgs gives quarks a rounding error of mass; the strong force gives
        them a <em>world</em>.
      </div>
    </div>
  </div>

  <p class="text-xs text-surface-500">
    The literature is more careful than this bar: in the modern trace-anomaly decomposition, the
    938 MeV splits into quark-mass, quark-energy, gluon-field and quantum-anomaly pieces whose sizes
    depend on the renormalisation scheme. What is not scheme-dependent: essentially none of your mass
    comes from the Higgs.
  </p>
</div>
