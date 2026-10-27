<script lang="ts">
  import { onMount } from 'svelte';
  import { fit, clear, mulberry32, QUARK_COLOURS, ANTI_COLOURS, clamp, fmt } from './util';

  // σ ≈ 1 GeV/fm  ⇒  1.602e-10 J / 1e-15 m ≈ 1.6e5 N
  const SIGMA_GEV_FM = 1.0;
  const SIGMA_NEWTON = 0.9 * 1.602e-10 / 1e-15; // ≈ 1.44e5 N for σ=0.9 GeV/fm

  let sep = $state(0.55); // separation in fm, 0.05 .. 1.6
  let canvas: HTMLCanvasElement;
  let raf = 0;
  let t = 0;
  let pairsCreated = $state(0);
  let flash = 0;
  let mesons: { x: number; y: number; vx: number; vy: number; c: number; a: number; life: number }[] = [];
  let dragging = false;

  const energy = $derived(SIGMA_GEV_FM * sep); // GeV of potential energy
  const broken = $derived(sep > 1.2);

  function snap() {
    if (sep <= 1.2) return;
    // string breaking: two new quark pairs appear, the string becomes two mesons
    pairsCreated += 1;
    flash = 1;
    mesons.push({
      x: -0.35,
      y: 0,
      vx: -1.1 - Math.random() * 0.5,
      vy: (Math.random() - 0.5) * 0.35,
      c: 0,
      a: 0,
      life: 1
    });
    mesons.push({
      x: 0.35,
      y: 0,
      vx: 1.1 + Math.random() * 0.5,
      vy: (Math.random() - 0.5) * 0.35,
      c: 1,
      a: 1,
      life: 1
    });
    sep = 0.3;
  }

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h);
    t += 0.016;
    flash *= 0.94;

    const cx = w / 2;
    const cy = h * 0.52;
    const scale = Math.min(w * 0.34, 320); // px per fm at sep=1

    // ── the flux tube ──
    const half = sep * scale;
    const tubeW = clamp(34 - sep * 12, 10, 34);
    const x0 = cx - half;
    const x1 = cx + half;

    if (!broken) {
      const grad = ctx.createLinearGradient(x0, cy, x1, cy);
      grad.addColorStop(0, 'rgba(248,113,113,0.55)');
      grad.addColorStop(0.5, 'rgba(250,204,21,0.7)');
      grad.addColorStop(1, 'rgba(147,197,253,0.55)');
      ctx.strokeStyle = grad;
      ctx.lineWidth = tubeW;
      ctx.lineCap = 'round';
      ctx.beginPath();
      ctx.moveTo(x0, cy);
      for (let i = 1; i <= 24; i++) {
        const u = i / 24;
        const x = x0 + (x1 - x0) * u;
        const y = cy + Math.sin(u * Math.PI * 6 + t * 3) * (1.5 + sep * 1.5);
        ctx.lineTo(x, y);
      }
      ctx.stroke();

      // energy density glow scaled by tension
      ctx.strokeStyle = `rgba(250,250,210,${0.25 + 0.2 * Math.sin(t * 4)})`;
      ctx.lineWidth = 2;
      ctx.stroke();
    }

    // ── quarks ──
    const drawQ = (x: number, y: number, col: string, label: string) => {
      ctx.beginPath();
      ctx.arc(x, y, 15, 0, Math.PI * 2);
      ctx.fillStyle = col;
      ctx.shadowColor = col;
      ctx.shadowBlur = 20;
      ctx.fill();
      ctx.shadowBlur = 0;
      ctx.strokeStyle = '#0b1020';
      ctx.lineWidth = 2;
      ctx.stroke();
      ctx.fillStyle = 'rgba(11,16,32,0.85)';
      ctx.font = '10px ui-monospace, monospace';
      ctx.textAlign = 'center';
      ctx.fillText(label, x, y + 3.5);
    };
    if (!broken) {
      drawQ(x0, cy, QUARK_COLOURS[0], 'q');
      drawQ(x1, cy, ANTI_COLOURS[0], 'q̄');
    }

    // ── snapping / hadronisation ──
    if (flash > 0.02) {
      ctx.beginPath();
      ctx.arc(cx, cy, 60 * (1 - flash) + 18, 0, Math.PI * 2);
      ctx.strokeStyle = `rgba(255,255,255,${flash})`;
      ctx.lineWidth = 3;
      ctx.stroke();
    }

    // decay the meson list
    mesons.forEach((m) => {
      m.x += m.vx * 0.02;
      m.y += m.vy * 0.02;
      m.life -= 0.01;
      const mx = cx + m.x * scale * 1.2;
      const my = cy + m.y * scale * 1.2 - 8;
      ctx.globalAlpha = clamp(m.life, 0, 1);
      drawQ(mx, my - 6, QUARK_COLOURS[m.c], 'q');
      drawQ(mx + 22, my + 6, ANTI_COLOURS[m.a], 'q̄');
      ctx.beginPath();
      ctx.moveTo(mx + 6, my - 2);
      ctx.lineTo(mx + 16, my + 2);
      ctx.strokeStyle = 'rgba(250,204,21,0.8)';
      ctx.lineWidth = 3;
      ctx.stroke();
      ctx.globalAlpha = 1;
    });
    mesons = mesons.filter((m) => m.life > 0);

    // ── energy readout bars ──
    const barW = Math.min(w - 60, 420);
    const bx = (w - barW) / 2;
    const by = h - 58;
    ctx.fillStyle = 'rgba(30,41,59,0.9)';
    ctx.fillRect(bx, by, barW, 8);
    const fracE = clamp(energy / 1.6, 0, 1);
    ctx.fillStyle = `hsl(${50 - 40 * fracE}, 90%, 60%)`;
    ctx.fillRect(bx, by, barW * fracE, 8);

    ctx.font = '11px ui-monospace, monospace';
    ctx.textAlign = 'center';
    ctx.fillStyle = 'rgba(226,232,240,0.9)';
    ctx.fillText(
      `r = ${sep.toFixed(2)} fm   ·   V = σr = ${energy.toFixed(2)} GeV   ·   force = ${fmt(SIGMA_NEWTON / 1000, 0)} kN, constant`,
      w / 2,
      by - 10
    );
    ctx.fillStyle = 'rgba(148,163,184,0.8)';
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText(
      broken
        ? 'the tube has enough energy to make a new qq̄ pair — pull again'
        : `enough energy for a new qq̄ pair at ≈ 1.2 fm`,
      w / 2,
      by + 26
    );

    raf = requestAnimationFrame(draw);
  }

  function pointer(e: PointerEvent) {
    if (!canvas) return;
    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const scale = Math.min(rect.width * 0.34, 320);
    const s = clamp(Math.abs(x - rect.width / 2) / scale, 0.05, 1.6);
    sep = s;
    if (sep > 1.2) snap();
  }

  onMount(() => {
    draw();
    return () => cancelAnimationFrame(raf);
  });
</script>

<div class="card card-demo p-6 space-y-4">
  <div class="flex flex-wrap gap-3 items-center">
    <label class="label flex-1 min-w-48">
      <span class="text-xs">separation r = {sep.toFixed(2)} fm</span>
      <input
        type="range"
        class="input"
        min="0.05"
        max="1.6"
        step="0.01"
        bind:value={sep}
        onchange={() => sep > 1.2 && snap()}
      />
    </label>
    <button
      class="btn preset-tonal-warning btn-sm"
      onclick={() => {
        sep = 1.6;
        snap();
      }}
    >
      Snap the string
    </button>
    <span class="badge preset-tonal">pairs created: {pairsCreated}</span>
  </div>

  <canvas
    bind:this={canvas}
    class="w-full rounded border border-surface-700 cursor-ew-resize touch-none"
    style="height:300px"
    onpointerdown={(e) => {
      dragging = true;
      pointer(e);
    }}
    onpointermove={(e) => dragging && pointer(e)}
    onpointerup={() => (dragging = false)}
    onpointerleave={() => (dragging = false)}
  ></canvas>

  <div class="grid sm:grid-cols-3 gap-3 text-xs">
    <div class="bg-surface-800/60 rounded p-3">
      <div class="font-semibold text-surface-300 mb-1">Linear potential</div>
      <div class="text-surface-500">
        V(r) ≈ σr with σ ≈ 0.9–1 GeV/fm. Energy grows <em>linearly</em> forever. Pulling harder does
        not weaken the force; it adds energy.
      </div>
    </div>
    <div class="bg-surface-800/60 rounded p-3">
      <div class="font-semibold text-surface-300 mb-1">The force in human units</div>
      <div class="text-surface-500">
        ≈ {fmt(SIGMA_NEWTON / 1000, 0)} kN — roughly 15 tonnes of weight — at <em>any</em> distance.
        Nothing in your experience is remotely this stiff.
      </div>
    </div>
    <div class="bg-surface-800/60 rounded p-3">
      <div class="font-semibold text-surface-300 mb-1">Why it never ends badly</div>
      <div class="text-surface-500">
        At r ≈ 1.2 fm the stored energy exceeds 2m<sub>q</sub>c², so the tube snaps by
        <em>creating a new quark–antiquark pair</em>. You get two mesons, not two free quarks.
      </div>
    </div>
  </div>

  <p class="text-xs text-surface-500">
    Drag the canvas or the slider: the tube stretches. Past ~1.2 fm it never breaks into free quarks —
    it breaks into <em>more hadrons</em>. This is why no free quark has ever been seen, and why
    “confinement” is the polite word for “if you try to look at a quark, the universe manufactures new
    matter to hide it”.
  </p>
</div>
