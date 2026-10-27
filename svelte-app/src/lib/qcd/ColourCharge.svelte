<script lang="ts">
  import { onMount } from 'svelte';
  import { fit, clear, mixCss, QUARK_NAMES } from './util';
  import {
    canonicalQuarks,
    randomGenerator,
    su3At,
    applyU,
    singletWeight,
    det,
    type M3,
    type Vec3,
    type Cx
  } from './su3';

  let H = $state<M3>(randomGenerator());
  let theta = $state(0);
  let canvas: HTMLCanvasElement;
  let caught = $state<number[] | null>(null);
  let tween: number | null = null;

  const U = $derived(su3At(H, theta));
  const quarks = $derived(canonicalQuarks().map((v) => applyU(U, v)));
  const weights = $derived(quarks.map((q) => [0, 1, 2].map((i) => q.re[i] ** 2 + q.im[i] ** 2)));
  const invariant = $derived(singletWeight(quarks[0], quarks[1], quarks[2]));

  /** Joint measurement amplitudes: A(i,j,k) = eps_abc U_ia U_jb U_kc = det(rows i,j,k). */
  function measurement(): number[] {
    const p: number[] = new Array(27).fill(0);
    for (let i = 0; i < 3; i++)
      for (let j = 0; j < 3; j++)
        for (let k = 0; k < 3; k++) {
          const sub: M3 = {
            re: [U.re[i], U.re[j], U.re[k]],
            im: [U.im[i], U.im[j], U.im[k]]
          };
          const d: Cx = det(sub);
          p[(i * 3 + j) * 3 + k] = d.re * d.re + d.im * d.im;
        }
    return p;
  }

  function doMeasure() {
    const p = measurement();
    const total = p.reduce((a, b) => a + b, 0);
    let x = Math.random() * total;
    for (let n = 0; n < p.length; n++) {
      x -= p[n];
      if (x <= 0) {
        caught = [Math.floor(n / 9), Math.floor(n / 3) % 3, n % 3];
        return;
      }
    }
    caught = [0, 1, 2];
  }

  function randomise() {
    H = randomGenerator();
    caught = null;
    // sweep the parameter so it is obvious the colours *move*
    if (tween !== null) cancelAnimationFrame(tween);
    const t0 = performance.now();
    const step = (now: number) => {
      const k = Math.min(1, (now - t0) / 900);
      theta = k * (Math.PI * 2);
      if (k < 1) tween = requestAnimationFrame(step);
      else tween = null;
    };
    tween = requestAnimationFrame(step);
  }

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h);

    const cx = w / 2;
    const cy = h / 2 - 14;
    const R = Math.min(w * 0.3, h * 0.33);
    const pos = [0, 1, 2].map((i) => {
      const a = -Math.PI / 2 + (i * 2 * Math.PI) / 3;
      return [cx + R * Math.cos(a), cy + R * Math.sin(a)];
    });

    // "proton" body
    const body = ctx.createRadialGradient(cx, cy, 4, cx, cy, R * 1.5);
    body.addColorStop(0, 'rgba(56,189,248,0.10)');
    body.addColorStop(1, 'rgba(56,189,248,0)');
    ctx.fillStyle = body;
    ctx.beginPath();
    ctx.arc(cx, cy, R * 1.5, 0, Math.PI * 2);
    ctx.fill();

    // colour triads: three little bars around each quark
    for (let q = 0; q < 3; q++) {
      const [x, y] = pos[q];
      const wts = weights[q];
      const hit = caught;
      const shown = hit ? [0, 1, 2].map((c) => (c === hit[q] ? 1 : 0)) : wts;

      const col = mixCss(shown);
      const r = 26;
      ctx.beginPath();
      ctx.arc(x, y, r, 0, Math.PI * 2);
      ctx.fillStyle = col;
      ctx.shadowColor = col;
      ctx.shadowBlur = 22;
      ctx.fill();
      ctx.shadowBlur = 0;
      ctx.strokeStyle = 'rgba(15,23,42,0.85)';
      ctx.lineWidth = 2;
      ctx.stroke();

      // tiny r/g/b weight pips below each quark
      const bw = 10;
      for (let c = 0; c < 3; c++) {
        const bx = x - 1.5 * bw + c * bw;
        const by = y + r + 9;
        ctx.fillStyle = 'rgba(30,41,59,0.9)';
        ctx.fillRect(bx, by, bw - 2, 5);
        ctx.fillStyle = mixCss([c === 0 ? 1 : 0, c === 1 ? 1 : 0, c === 2 ? 1 : 0]);
        ctx.fillRect(bx, by, (bw - 2) * Math.min(1, shown[c]), 5);
      }

      ctx.fillStyle = 'rgba(226,232,240,0.75)';
      ctx.font = '11px ui-monospace, monospace';
      ctx.textAlign = 'center';
      ctx.fillText(`q${q + 1}`, x, y + 4);
    }

    // the singlet "glue" triangle
    ctx.beginPath();
    pos.forEach(([x, y], i) => (i ? ctx.lineTo(x, y) : ctx.moveTo(x, y)));
    ctx.closePath();
    ctx.strokeStyle = `rgba(148,163,184,${0.12 + 0.35 * Math.min(1, invariant)})`;
    ctx.setLineDash([5, 6]);
    ctx.lineWidth = 1.5;
    ctx.stroke();
    ctx.setLineDash([]);

    // the invariant, kept well clear of the diagram
    ctx.textAlign = 'center';
    ctx.fillStyle = 'rgba(226,232,240,0.9)';
    ctx.font = '13px ui-monospace, monospace';
    ctx.fillText(`|ε qqq|² = ${invariant.toFixed(6)}`, cx, h - 34);
    ctx.fillStyle = 'rgba(100,116,139,0.9)';
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText('the invariant — change the basis all you like, this is the physics', cx, h - 16);
  }

  onMount(() => {
    const ro = new ResizeObserver(() => draw());
    ro.observe(canvas);
    draw();
    return () => ro.disconnect();
  });

  $effect(() => {
    // re-read reactive deps
    void weights;
    void caught;
    void invariant;
    draw();
  });
</script>

<div class="card card-demo p-6 space-y-4">
  <div class="flex flex-wrap gap-3 items-center">
    <button class="btn preset-filled-primary-500 btn-sm" onclick={randomise}>
      Change colour basis
    </button>
    <button class="btn preset-tonal btn-sm" onclick={doMeasure}>Measure the colours</button>
    <label class="label flex-1 min-w-40">
      <span class="text-xs">Gauge rotation θ = {theta.toFixed(2)}</span>
      <input type="range" class="input" min="0" max={Math.PI * 2} step="0.01" bind:value={theta} />
    </label>
  </div>

  <canvas bind:this={canvas} class="w-full rounded border border-surface-700" style="height:340px"
  ></canvas>

  <div class="grid sm:grid-cols-2 gap-3 text-xs">
    <div class="bg-surface-800/60 rounded p-3 space-y-1">
      <div class="font-semibold text-surface-300">What each quark “is” right now</div>
      {#each weights as wts, q}
        <div class="font-mono text-[11px] text-surface-400">
          q{q + 1}: {wts
            .map((v, c) => `${(100 * v).toFixed(0)}% ${QUARK_NAMES[c]}`)
            .join(' · ')}
        </div>
      {/each}
    </div>
    <div class="bg-surface-800/60 rounded p-3 space-y-1">
      <div class="font-semibold text-surface-300">Measurement</div>
      {#if caught}
        <div class="font-mono text-[11px] text-surface-400">
          you caught: {caught.map((c) => QUARK_NAMES[c]).join(', ')}
        </div>
        <div class="text-[11px] text-surface-500">
          Always one of each. Always. In <em>every</em> basis. The three outcomes are perfectly
          correlated no matter which axis you call “red”.
        </div>
      {:else}
        <div class="text-[11px] text-surface-500">
          Click <em>Measure</em>. Then rotate the basis 90° and measure again.
        </div>
      {/if}
    </div>
  </div>

  <p class="text-xs text-surface-500">
    The colours above are a lie you will keep needing. “Red”, “green” and “blue” are the three
    components of a complex 3-vector; the rotation above is a random SU(3) matrix. Individual quark
    colours changed wildly. The physics did not: |ε<sub>ijk</sub>q<sup>i</sup>q<sup>j</sup>q<sup>k</sup>|²
    never left 1. Colour is not a property of a quark — it is a choice of coordinates, and a hadron
    is the statement that the choice doesn't matter.
  </p>
</div>
