<script lang="ts">
  import { onMount } from 'svelte';
  import type { WasmApi } from './wasm/loader';

  let { wasm } = $props<{ wasm: WasmApi }>();

  const planetNames = ['Jupiter', 'Saturn', 'Uranus', 'Neptune', 'Pluto'];
  const planetSymbols = ['♃', '♄', '♅', '♆', '♇'];
  const planetColors = ['#fbbf24', '#d4a574', '#60a5fa', '#3b82f6', '#a78bfa'];

  let date = $state('2026-07-20');
  let positions = $state<number[]>([0, 5.2, 0, 9.5, 0, 19.2, 0, 30.1, 0, 39.5]);
  let barbaultIdx = $state(0);
  let canvas: HTMLCanvasElement | null = $state(null);
  let zoom = $state(1.0);
  let animating = $state(false);
  let animFrame: ReturnType<typeof requestAnimationFrame> | null = null;

  const ZODIAC = ['♈ Aries', '♉ Tau', '♊ Gem', '♋ Can', '♌ Leo', '♍ Vir',
    '♎ Lib', '♏ Sco', '♐ Sag', '♑ Cap', '♒ Aqu', '♓ Pis'];

  function compute() {
    if (!wasm) return;
    try {
      positions = wasm.planet_positions(date);
      // Extract just the longitudes (every other element)
      const lons = positions.filter((_, i) => i % 2 === 0);
      barbaultIdx = wasm.barbault_index(lons);
    } catch (e) {
      console.error(e);
    }
  }

  function draw() {
    if (!canvas) return;
    const ctx = canvas.getContext('2d')!;
    const w = canvas.width = canvas.clientWidth * devicePixelRatio;
    const h = canvas.height = canvas.clientHeight * devicePixelRatio;
    ctx.scale(devicePixelRatio, devicePixelRatio);
    const cw = canvas.clientWidth;
    const ch = canvas.clientHeight;
    const cx = cw / 2;
    const cy = ch / 2;
    const r = Math.min(cx, cy) * 0.75 * zoom;

    ctx.clearRect(0, 0, cw, ch);

    // Background circle
    ctx.fillStyle = '#1a1a2e';
    ctx.beginPath(); ctx.arc(cx, cy, r + 30, 0, Math.PI * 2); ctx.fill();

    // Zodiac ring
    const r_inner = r * 0.85;
    const r_outer = r * 0.97;
    for (let i = 0; i < 12; i++) {
      const a1 = ((i * 30 - 195) * Math.PI) / 180;
      const a2 = (((i + 1) * 30 - 195) * Math.PI) / 180;
      ctx.beginPath();
      ctx.moveTo(cx + r_inner * Math.cos(a1), cy + r_inner * Math.sin(a1));
      ctx.arc(cx, cy, r_inner, a1, a2);
      ctx.arc(cx, cy, r_outer, a2, a1, true);
      ctx.closePath();
      ctx.fillStyle = i % 2 === 0 ? '#1e293b' : '#1e293b';
      ctx.fill();
      ctx.strokeStyle = '#334155';
      ctx.lineWidth = 0.5;
      ctx.stroke();

      // Sign labels
      const ma = (a1 + a2) / 2;
      const lx = cx + (r * 0.79) * Math.cos(ma);
      const ly = cy + (r * 0.79) * Math.sin(ma);
      ctx.fillStyle = '#64748b';
      ctx.font = '9px sans-serif';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.save();
      ctx.translate(lx, ly);
      ctx.fillText(ZODIAC[i], 0, 0);
      ctx.restore();
    }

    // Inner grid lines
    ctx.strokeStyle = '#1e293b';
    ctx.lineWidth = 0.5;
    for (let i = 0; i < 36; i++) {
      const a = ((i * 10 - 195) * Math.PI) / 180;
      ctx.beginPath();
      ctx.moveTo(cx + r_inner * Math.cos(a), cy + r_inner * Math.sin(a));
      ctx.lineTo(cx + r_outer * Math.cos(a), cy + r_outer * Math.sin(a));
      ctx.stroke();
    }

    // Cardinal axes
    ctx.strokeStyle = '#475569';
    ctx.lineWidth = 1;
    for (const deg of [0, 90, 180, 270]) {
      const a = ((deg - 195) * Math.PI) / 180;
      ctx.beginPath();
      ctx.moveTo(cx + r_inner * Math.cos(a), cy + r_inner * Math.sin(a));
      ctx.lineTo(cx + r * 1.02 * Math.cos(a), cy + r * 1.02 * Math.sin(a));
      ctx.stroke();
    }

    // Aspect lines between planets
    const lons = positions.filter((_, i) => i % 2 === 0);
    const r_planet = r * 0.55;
    for (let i = 0; i < lons.length; i++) {
      for (let j = i + 1; j < lons.length; j++) {
        const diff = Math.abs(lons[i] - lons[j]) % 360;
        const ad = Math.min(diff, 360 - diff);
        // Show lines for conjunctions, sextiles, squares, trines, oppositions
        const aspects = [
          { deg: 0, tol: 8, color: 'rgba(250,204,21,0.4)', width: 1 },
          { deg: 60, tol: 6, color: 'rgba(52,211,153,0.3)', width: 1 },
          { deg: 90, tol: 5, color: 'rgba(248,113,113,0.3)', width: 1 },
          { deg: 120, tol: 6, color: 'rgba(96,165,250,0.3)', width: 1 },
          { deg: 180, tol: 8, color: 'rgba(192,132,252,0.4)', width: 1.5 },
        ];
        for (const { deg, tol, color, width } of aspects) {
          if (Math.abs(ad - deg) < tol) {
            const a1 = ((lons[i] - 195) * Math.PI) / 180;
            const a2 = ((lons[j] - 195) * Math.PI) / 180;
            ctx.beginPath();
            ctx.moveTo(cx + r_planet * Math.cos(a1), cy + r_planet * Math.sin(a1));
            ctx.lineTo(cx + r_planet * Math.cos(a2), cy + r_planet * Math.sin(a2));
            ctx.strokeStyle = color;
            ctx.lineWidth = width;
            ctx.stroke();
          }
        }
      }
    }

    // Planet markers
    const scaleDist = [1.8, 1.5, 1.3, 1.1, 0.8]; // closer = bigger dot
    for (let i = 0; i < lons.length; i++) {
      const a = ((lons[i] - 195) * Math.PI) / 180;
      const px = cx + r_planet * Math.cos(a);
      const py = cy + r_planet * Math.sin(a);

      // Orbit circle
      ctx.beginPath();
      ctx.arc(cx, cy, r * 0.2 * (i + 1), 0, Math.PI * 2);
      ctx.strokeStyle = '#1e293b';
      ctx.lineWidth = 0.5;
      ctx.stroke();

      // Glow
      const g = ctx.createRadialGradient(px, py, 0, px, py, 12 * scaleDist[i]);
      g.addColorStop(0, planetColors[i]);
      g.addColorStop(1, 'transparent');
      ctx.beginPath();
      ctx.arc(px, py, 12, 0, Math.PI * 2);
      ctx.fillStyle = g;
      ctx.fill();

      // Planet dot
      ctx.beginPath();
      ctx.arc(px, py, 4 * scaleDist[i], 0, Math.PI * 2);
      ctx.fillStyle = planetColors[i];
      ctx.fill();

      // Symbol
      ctx.fillStyle = '#fff';
      ctx.font = `${11 * scaleDist[i]}px sans-serif`;
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText(planetSymbols[i], px, py - 10 * scaleDist[i]);

      // Name + degrees
      ctx.fillStyle = '#94a3b8';
      ctx.font = '9px monospace';
      ctx.fillText(`${planetNames[i]} ${lons[i].toFixed(1)}°`, px, py + 12 * scaleDist[i]);
    }

    // Center dot (Sun)
    ctx.beginPath();
    ctx.arc(cx, cy, 5, 0, Math.PI * 2);
    ctx.fillStyle = '#fbbf24';
    ctx.fill();
    ctx.beginPath();
    ctx.arc(cx, cy, 9, 0, Math.PI * 2);
    ctx.fillStyle = 'rgba(251,191,36,0.3)';
    ctx.fill();
  }

  function animate() {
    compute();
    requestAnimationFrame(() => {
      draw();
      animFrame = null;
    });
  }

  let debounce: ReturnType<typeof setTimeout> | null = null;
  function onDateChange() {
    if (debounce) clearTimeout(debounce);
    debounce = setTimeout(() => animate(), 100);
  }

  function startAnimation() {
    animating = true;
    const d = new Date(date);
    function step() {
      if (!animating) return;
      d.setDate(d.getDate() + 10);
      date = d.toISOString().slice(0, 10);
      compute();
      requestAnimationFrame(() => {
        draw();
        animFrame = requestAnimationFrame(step);
      });
    }
    step();
  }

  function stopAnimation() {
    animating = false;
    if (animFrame) cancelAnimationFrame(animFrame);
  }

  onMount(() => {
    compute();
    requestAnimationFrame(draw);
  });

  // Redraw on zoom/resize via effect
  $effect(() => {
    if (!wasm) return;
    void zoom;
    requestAnimationFrame(draw);
  });
</script>

<div class="card card-demo p-4 space-y-3">
  <div class="flex gap-2 items-center flex-wrap">
    <h3 class="h3 flex-1">🪐 Barbault Basket — Ecliptic Chart</h3>
    <div class="flex gap-1 items-center text-xs">
      <button class="btn preset-tonal text-xs" onclick={() => { zoom = Math.max(0.5, zoom - 0.1); }}>−</button>
      <span class="text-surface-400">{(zoom * 100).toFixed(0)}%</span>
      <button class="btn preset-tonal text-xs" onclick={() => { zoom = Math.min(2.0, zoom + 0.1); }}>+</button>
    </div>
  </div>

  <div class="flex gap-2 items-center flex-wrap">
    <input type="date" class="input text-xs w-36" bind:value={date} oninput={onDateChange} />
    <button class="btn preset-tonal text-xs" onclick={animate}>🔄 Update</button>
    {#if animating}
      <button class="btn preset-filled-warning text-xs" onclick={stopAnimation}>⏹ Stop</button>
    {:else}
      <button class="btn preset-tonal text-xs" onclick={startAnimation}>▶ Play (10d steps)</button>
    {/if}
  </div>

  <canvas bind:this={canvas} class="w-full rounded" style="height:500px;background:#0f172a"></canvas>

  <div class="flex gap-4 items-center flex-wrap">
    <div class="flex-1 bg-surface-800 p-3 rounded space-y-1">
      <div class="flex gap-2 items-baseline">
        <span class="text-xs text-surface-500">Barbault Cyclic Index</span>
        <span class="text-sm font-mono {barbaultIdx < 200 ? 'text-green-400' : barbaultIdx < 500 ? 'text-amber-400' : 'text-red-400'}">
          {barbaultIdx.toFixed(1)}°
        </span>
        <span class="text-xs text-surface-600">(max 900°)</span>
      </div>
      <div class="mt-1 h-2 bg-surface-700 rounded overflow-hidden">
        <div class="h-full rounded transition-all duration-300"
          style="width:{Math.min(100, (barbaultIdx / 900) * 100)}%;
            background:{barbaultIdx < 200 ? '#22c55e' : barbaultIdx < 500 ? '#f59e0b' : '#ef4444'}">
        </div>
      </div>
      <div class="text-xs text-surface-500">
        {#if barbaultIdx < 200}
          <span class="text-green-400">🟢 Tight cluster — outer planets bunched in one hemisphere</span>
        {:else if barbaultIdx < 500}
          <span class="text-amber-400">🟡 Moderate spread</span>
        {:else}
          <span class="text-red-400">🔴 Widely dispersed — planets span the full zodiac</span>
        {/if}
      </div>
    </div>

    <div class="bg-surface-800 p-3 rounded space-y-1 text-xs">
      <div class="text-surface-400 font-mono mb-1">Positions (ecliptic lon)</div>
      {#each planetNames as name, i}
        {@const lon = positions[i * 2]}
        <div class="flex gap-1 items-center">
          <span class="w-2 h-2 rounded-full inline-block" style="background:{planetColors[i]}"></span>
          <span class="text-surface-300">{planetSymbols[i]}</span>
          <span class="font-mono text-surface-400">{name}: <span class="text-primary-300">{lon.toFixed(1)}°</span></span>
        </div>
      {/each}
    </div>
  </div>

  <details class="text-xs text-surface-500 bg-surface-800 p-3 rounded">
    <summary class="cursor-pointer text-surface-400">🛠 How ferray powers this</summary>
    <div class="mt-2 space-y-1">
      <p><strong>Kepler's equation</strong> (<code>M = E − e·sinE</code>) is solved via Newton's method using <code class="code-block">ferray-ufunc::sin</code> and <code class="code-block">ferray-ufunc::cos</code> — though for scalar values std trig is used.</p>
      <p><strong>Ecliptic positions</strong> are computed for 5 planets using Meeus mean orbital elements (J2000.0) — all arithmetic on <code class="code-block">f64</code> scalars, the same data type <code class="code-block">ferray-core::Array1</code> uses under the hood.</p>
      <p><strong>Barbault Cyclic Index</strong> collects the 10 pair-wise angular distances into a <code class="code-block">ferray_core::Array1&lt;f64&gt;</code>, iterates with <code class="code-block">.iter()</code>, and sums — showing ferray-core array creation and element access in action.</p>
      <p class="mt-1"><strong>Future:</strong> A batch position sweep (e.g. 100 dates) could use <code class="code-block">ferray_ufunc::sin</code>/<code class="code-block">cos</code> on full <code class="code-block">Array1</code> to compute all planets at once — the classic NumPy pattern that ferray was designed to replicate.</p>
    </div>
  </details>
</div>
