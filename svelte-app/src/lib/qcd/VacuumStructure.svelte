<script lang="ts">
  import { onMount } from 'svelte';
  import { fit, clear, mulberry32, clamp } from './util';

  // ── the vacuum, as a boiling mess of temporary particles ─────────────────
  interface Pair {
    x: number;
    y: number;
    life: number;
    maxLife: number;
    colour: number;
  }
  interface Instanton {
    x: number;
    y: number;
    r: number;
    phase: number;
    q: number;
  }

  let canvas: HTMLCanvasElement;
  let raf = 0;
  let t = 0;
  let showPairs = $state(true);
  let showGluons = $state(true);
  let showInstantons = $state(true);

  const rnd = mulberry32(20260720);
  let pairs: Pair[] = [];
  let instantons: Instanton[] = [];
  let blobs: { x: number; y: number; v: number }[] = [];
  for (let i = 0; i < 40; i++) blobs.push({ x: rnd(), y: rnd(), v: rnd() });

  function spawn(w: number, h: number) {
    if (pairs.length < 90 && Math.random() < 0.55) {
      const maxLife = 20 + Math.random() * 40;
      pairs.push({
        x: 30 + Math.random() * (w - 60),
        y: 30 + Math.random() * (h - 60),
        life: maxLife,
        maxLife,
        colour: Math.floor(Math.random() * 3)
      });
    }
    if (instantons.length < 4 && Math.random() < 0.004) {
      instantons.push({
        x: 40 + Math.random() * (w - 80),
        y: 40 + Math.random() * (h - 80),
        r: 16 + Math.random() * 18,
        phase: 0,
        q: Math.random() < 0.5 ? 1 : -1
      });
    }
  }

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h);
    t += 0.016;

    // background: the condensate — a low-frequency shimmer that never goes away
    if (showGluons) {
      for (const b of blobs) {
        const x = b.x * w;
        const y = b.y * h;
        const r = 40 + 60 * (0.5 + 0.5 * Math.sin(t * 0.4 + b.v * 6.28));
        const g = ctx.createRadialGradient(x, y, 0, x, y, r);
        g.addColorStop(0, `rgba(56,189,248,${0.05 + 0.05 * Math.sin(t + b.v * 9)})`);
        g.addColorStop(1, 'rgba(56,189,248,0)');
        ctx.fillStyle = g;
        ctx.beginPath();
        ctx.arc(x, y, r, 0, Math.PI * 2);
        ctx.fill();
      }
      // wriggling field lines
      ctx.lineWidth = 1;
      for (let i = 0; i < 26; i++) {
        const seed = i * 97.13;
        const y0 = ((seed * 13.7) % 1) * h;
        ctx.beginPath();
        ctx.moveTo(0, y0);
        for (let x = 0; x <= w; x += 12) {
          const y =
            y0 +
            9 * Math.sin(x * 0.02 + t * 1.4 + seed) +
            5 * Math.sin(x * 0.051 - t * 0.9 + seed * 2);
          ctx.lineTo(x, y);
        }
        ctx.strokeStyle = `rgba(56,189,248,${0.05 + 0.04 * Math.sin(t + i)})`;
        ctx.stroke();
      }
    }

    // instantons: lumps of topological charge
    if (showInstantons) {
      for (const ins of instantons) {
        ins.phase += 0.02;
        const a = clamp(Math.sin(ins.phase), 0, 1);
        ctx.beginPath();
        ctx.arc(ins.x, ins.y, ins.r, 0, Math.PI * 2);
        ctx.fillStyle = ins.q > 0 ? `rgba(167,139,250,${0.25 * a})` : `rgba(251,191,36,${0.25 * a})`;
        ctx.fill();
        ctx.strokeStyle = ins.q > 0 ? `rgba(167,139,250,${0.8 * a})` : `rgba(251,191,36,${0.8 * a})`;
        ctx.lineWidth = 1.5;
        ctx.stroke();
        ctx.fillStyle = `rgba(226,232,240,${0.75 * a})`;
        ctx.font = '10px ui-monospace, monospace';
        ctx.textAlign = 'center';
        ctx.fillText(`Q=${ins.q > 0 ? '+1' : '−1'}`, ins.x, ins.y + 3);
      }
      instantons = instantons.filter((i) => Math.sin(i.phase) > -0.98);
    }

    // virtual quark pairs: born together, die together
    if (showPairs) {
      spawn(w, h);
      for (const p of pairs) {
        p.life -= 1;
        const u = clamp(p.life / p.maxLife, 0, 1);
        const alpha = Math.sin(u * Math.PI); // rise & fall
        const d = 9;
        const c1 = ['#f87171', '#4ade80', '#60a5fa'][p.colour];
        const c2 = ['#fca5a5', '#86efac', '#93c5fd'][p.colour];
        ctx.globalAlpha = alpha;
        ctx.beginPath();
        ctx.arc(p.x - d, p.y, 3.6, 0, Math.PI * 2);
        ctx.fillStyle = c1;
        ctx.fill();
        ctx.beginPath();
        ctx.arc(p.x + d, p.y, 3.6, 0, Math.PI * 2);
        ctx.fillStyle = c2;
        ctx.fill();
        ctx.beginPath();
        ctx.moveTo(p.x - d + 4, p.y);
        ctx.lineTo(p.x + d - 4, p.y);
        ctx.strokeStyle = 'rgba(250,204,21,0.5)';
        ctx.lineWidth = 1.2;
        ctx.stroke();
        ctx.globalAlpha = 1;
      }
      pairs = pairs.filter((p) => p.life > 0);
    }

    ctx.fillStyle = 'rgba(100,116,139,0.85)';
    ctx.font = '10px ui-monospace, monospace';
    ctx.textAlign = 'left';
    ctx.fillText('the QCD vacuum (artist’s impression, wildly not to scale)', 12, h - 12);

    raf = requestAnimationFrame(draw);
  }

  onMount(() => {
    draw();
    return () => cancelAnimationFrame(raf);
  });
</script>

<div class="card card-demo p-6 space-y-4">
  <div class="flex flex-wrap gap-4 items-center">
    <label class="flex items-center gap-2 cursor-pointer text-xs">
      <input type="checkbox" class="checkbox" bind:checked={showPairs} /> virtual quark pairs
    </label>
    <label class="flex items-center gap-2 cursor-pointer text-xs">
      <input type="checkbox" class="checkbox" bind:checked={showGluons} /> gluon condensate
    </label>
    <label class="flex items-center gap-2 cursor-pointer text-xs">
      <input type="checkbox" class="checkbox" bind:checked={showInstantons} /> instantons (topology)
    </label>
  </div>

  <canvas bind:this={canvas} class="w-full rounded border border-surface-700" style="height:300px"
  ></canvas>

  <div class="grid sm:grid-cols-3 gap-3 text-xs">
    <div class="bg-surface-800/60 rounded p-3">
      <div class="font-semibold text-surface-300 mb-1">⟨q̄q⟩ ≈ −(250 MeV)³</div>
      <div class="text-surface-500">
        The chiral condensate: the vacuum contains a non-zero density of quark–antiquark pairs. Turn
        them off and the pion would weigh exactly zero (its mass is borrowed from this condensate) and
        the proton would lose a chunk of its mass too. The vacuum is a material.
      </div>
    </div>
    <div class="bg-surface-800/60 rounded p-3">
      <div class="font-semibold text-surface-300 mb-1">⟨(α_s/π)G²⟩ ≈ 0.012 GeV⁴</div>
      <div class="text-surface-500">
        The gluon condensate. Filled with a field strength that is not zero even with no particles in
        it. “Empty” is a classical prejudice.
      </div>
    </div>
    <div class="bg-surface-800/60 rounded p-3">
      <div class="font-semibold text-surface-300 mb-1">~1 instanton per fm⁴</div>
      <div class="text-surface-500">
        Localised blobs of topological charge: configurations the field <em>tunnels</em> between. They
        are not particles. They are shapes of the vacuum.
      </div>
    </div>
  </div>

  <p class="text-xs text-surface-500">
    None of these blobs is a particle in the ordinary sense. They are terms in a superposition — the
    vacuum state is not “nothing”, it is the lowest-energy state of a field, and for a non-abelian
    field the lowest-energy state is a churning, symmetry-broken medium. If the naive estimates are
    taken at face value, the energy density sitting in these condensates should be enormous — about
    10<sup>44</sup> times the density of the dark energy we actually observe. Gravity does not seem to
    notice. Nobody knows why. That is the cosmological constant problem, and QCD is one of the two
    places the number comes from.
  </p>
</div>
