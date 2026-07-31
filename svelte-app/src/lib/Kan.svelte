<script lang="ts">
  import { onMount } from 'svelte';
  import type { WasmApi } from './wasm/loader';

  let { wasm } = $props<{ wasm: WasmApi }>();

  // ── configuration (changing these resets the network) ──
  let targetIdx = $state(0);
  let hidden = $state(5);
  let gridSize = $state(6);
  let order = $state(3);
  let samplesPerAxis = $state(24);
  let lr = $state(0.02);
  let epochsPerFrame = $state(4);
  let evalSize = $state(96);
  let playing = $state(true);

  // ── runtime stats ──
  let epoch = $state(0);
  let rmse = $state(0);
  let bestRmse = $state(Infinity);
  let nParams = $state(0);
  let frameMs = $state(0);
  let losses: number[] = $state([]);

  // ── canvases ──
  let truthCanvas: HTMLCanvasElement;
  let learnedCanvas: HTMLCanvasElement;
  let lossCanvas: HTMLCanvasElement;
  let splineCanvas: HTMLCanvasElement;

  // last grid evaluation (interleaved truth/learned) — refreshed lazily
  let gridData: number[] = [];

  const targets = [
    { name: 'exp(sin(πx/2)) · exp(y²/8)', short: 'wavy exp' },
    { name: '(x² − y²) / 18', short: 'saddle' },
    { name: 'x·y / 9', short: 'product' },
    { name: 'sin(x/2) · cos(y/2)', short: 'ripples' },
    { name: 'exp(−(x²+y²)/6)', short: 'gaussian' },
    { name: 'sin((x²+y²)/3)', short: 'rings' },
  ];

  let rafId = 0;

  // ── colormap (shared by both heatmaps) ──
  // Precomputed 512-entry LUT: 3 Math.sin per pixel → 3 lookups.
  const LUT_SIZE = 512;
  const colorLut = new Uint8Array(LUT_SIZE * 3);
  {
    for (let i = 0; i < LUT_SIZE; i++) {
      const t = i / (LUT_SIZE - 1);
      const r = Math.max(0, Math.min(1, Math.sin(t * Math.PI * 2 - Math.PI / 2) * 0.5 + 0.5));
      const g = Math.max(0, Math.min(1, Math.sin(t * Math.PI * 2 + Math.PI / 6) * 0.5 + 0.5));
      const b = Math.max(0, Math.min(1, Math.sin(t * Math.PI * 2 + 5 * Math.PI / 6) * 0.5 + 0.5));
      colorLut[i * 3] = Math.round(r * 255);
      colorLut[i * 3 + 1] = Math.round(g * 255);
      colorLut[i * 3 + 2] = Math.round(b * 255);
    }
  }

  // ── rendering ──
  function drawHeatmap(canvas: HTMLCanvasElement, values: number[], size: number) {
    const ctx = canvas.getContext('2d')!;
    canvas.width = size;
    canvas.height = size;
    let minV = Infinity, maxV = -Infinity;
    for (const v of values) { if (v < minV) minV = v; if (v > maxV) maxV = v; }
    const range = (maxV - minV) || 1;
    const img = ctx.createImageData(size, size);
    const lut = colorLut;
    const lutScale = (LUT_SIZE - 1) / range;
    for (let i = 0; i < values.length; i++) {
      const t = (values[i] - minV) * lutScale;
      const li = (t < 0 ? 0 : t > LUT_SIZE - 1 ? LUT_SIZE - 1 : t) | 0;
      const px = i * 4;
      img.data[px] = lut[li * 3];
      img.data[px + 1] = lut[li * 3 + 1];
      img.data[px + 2] = lut[li * 3 + 2];
      img.data[px + 3] = 255;
    }
    ctx.putImageData(img, 0, 0);
  }

  function refreshGrid() {
    gridData = Array.from(wasm.kan_eval_grid(evalSize));
    drawTruth();
  }

  function drawTruth() {
    if (!truthCanvas || gridData.length === 0) return;
    const size = evalSize;
    const truth: number[] = new Array(size * size);
    for (let i = 0; i < size * size; i++) truth[i] = gridData[i * 2];
    drawHeatmap(truthCanvas, truth, size);
  }

  function drawLearned() {
    if (!learnedCanvas || gridData.length === 0) return;
    const size = evalSize;
    const learned: number[] = new Array(size * size);
    for (let i = 0; i < size * size; i++) learned[i] = gridData[i * 2 + 1];
    drawHeatmap(learnedCanvas, learned, size);
  }

  function drawLoss() {
    const canvas = lossCanvas;
    if (!canvas || losses.length === 0) return;
    const ctx = canvas.getContext('2d')!;
    const W = canvas.width, H = canvas.height;
    ctx.clearRect(0, 0, W, H);

    const minL = Math.max(1e-6, Math.min(...losses));
    const maxL = Math.max(1e-6, ...losses);
    const y0 = Math.log10(minL), y1 = Math.log10(maxL);
    const span = Math.max(1e-9, y1 - y0);

    // gridlines at decades
    ctx.strokeStyle = 'rgba(148,163,184,0.18)';
    ctx.lineWidth = 1;
    for (let d = Math.ceil(y0); d <= Math.floor(y1); d++) {
      const yy = H - ((d - y0) / span) * H;
      ctx.beginPath(); ctx.moveTo(0, yy); ctx.lineTo(W, yy); ctx.stroke();
      ctx.fillStyle = 'rgba(148,163,184,0.6)';
      ctx.font = '9px monospace';
      ctx.fillText(`1e${d}`, 4, yy - 3);
    }

    // polyline (downsample if long)
    const n = losses.length;
    const step = Math.max(1, Math.floor(n / 2000));
    ctx.strokeStyle = '#22d3ee';
    ctx.lineWidth = 1.5;
    ctx.beginPath();
    for (let i = 0; i < n; i += step) {
      const x = (i / Math.max(1, n - 1)) * W;
      const y = H - ((Math.log10(Math.max(1e-9, losses[i])) - y0) / span) * H;
      if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
    }
    ctx.stroke();

    // current value
    ctx.fillStyle = '#22d3ee';
    ctx.font = '10px monospace';
    ctx.fillText(`rmse ${rmse.toExponential(2)} · epoch ${epoch}`, W - 170, H - 6);
  }

  function drawSplines() {
    const canvas = splineCanvas;
    if (!canvas) return;
    const ctx = canvas.getContext('2d')!;
    const W = canvas.width, H = canvas.height;
    ctx.clearRect(0, 0, W, H);
    const PAD = 24;
    const X_LO = -4, X_HI = 4, Y_LO = -2.2, Y_HI = 2.2;

    const sx = (x: number) => PAD + ((x - X_LO) / (X_HI - X_LO)) * (W - 2 * PAD);
    const sy = (y: number) => H - PAD - ((y - Y_LO) / (Y_HI - Y_LO)) * (H - 2 * PAD);

    // axes + grid bounds
    ctx.strokeStyle = 'rgba(148,163,184,0.5)';
    ctx.lineWidth = 1;
    ctx.beginPath(); ctx.moveTo(PAD, sy(0)); ctx.lineTo(W - PAD, sy(0)); ctx.stroke();
    ctx.strokeStyle = 'rgba(148,163,184,0.2)';
    for (const gx of [-1, 0, 1]) {
      ctx.beginPath(); ctx.moveTo(sx(gx), PAD); ctx.lineTo(sx(gx), H - PAD); ctx.stroke();
    }
    ctx.fillStyle = 'rgba(148,163,184,0.6)';
    ctx.font = '9px monospace';
    ctx.fillText('−1', sx(-1) - 3, H - PAD + 12);
    ctx.fillText('1', sx(1) - 3, H - PAD + 12);

    const nPoints = 160;
    const drawEdge = (layer: number, from: number, to: number, color: string, width: number) => {
      const data = wasm.kan_edge_spline(layer, from, to, nPoints);
      if (data.length === 0) return;
      ctx.strokeStyle = color;
      ctx.lineWidth = width;
      ctx.beginPath();
      for (let i = 0; i < nPoints; i++) {
        const x = data[i * 2], y = data[i * 2 + 1];
        const px = sx(x), py = sy(y);
        if (i === 0) ctx.moveTo(px, py); else ctx.lineTo(px, py);
      }
      ctx.stroke();
    };

    // layer 0 (input → hidden): x-edge cyan, y-edge amber
    for (let j = 0; j < hidden; j++) {
      drawEdge(0, 0, j, 'rgba(34,211,238,0.75)', 1.3);
      drawEdge(0, 1, j, 'rgba(251,191,36,0.75)', 1.3);
    }
    // layer 1 (hidden → output): violet
    for (let j = 0; j < hidden; j++) {
      drawEdge(1, j, 0, 'rgba(167,139,250,0.8)', 1.6);
    }
  }

  // ── training loop ──
  let frameTick = 0;

  function step() {
    try {
      const t0 = performance.now();
      const ls = wasm.kan_train(epochsPerFrame, lr);
      const len = ls.length;
      if (len === 0) return;
      epoch += len;
      rmse = ls[len - 1];
      for (const l of ls) { losses.push(l); if (l < bestRmse) bestRmse = l; }
      frameMs = performance.now() - t0;
      // heatmaps barely change per epoch — refresh every 2nd frame
      frameTick++;
      if (frameTick % 2 === 0) {
        refreshGrid();
        drawLearned();
      }
      drawSplines();
      drawLoss();
    } catch (e) {
      console.error('KAN train step failed:', e);
      playing = false;
    }
  }

  function loop() {
    if (!playing) return;
    rafId = requestAnimationFrame(loop);
    step();
  }

  function reset() {
    try {
      wasm.kan_reset(hidden, gridSize, order, targetIdx, samplesPerAxis, (Date.now() & 0xffffffff) >>> 0);
      const s = JSON.parse(wasm.kan_stats());
      nParams = s.n_params;
      losses = [];
      bestRmse = Infinity;
      rmse = 0;
      epoch = 0;
      refreshGrid();
      drawSplines();
      drawLoss();
    } catch (e) {
      console.error('KAN reset failed:', e);
    }
  }

  // Config changes rebuild the network. Uses explicit input handlers instead
  // of an $effect so training state writes can't re-trigger the reset.
  function onConfigChange() {
    reset();
  }

  onMount(() => {
    reset();
    rafId = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(rafId);
  });
</script>

<div class="card card-demo p-6 space-y-6">

  <!-- controls -->
  <div class="flex gap-3 items-end flex-wrap">
    <label class="label w-64">
      <span>Target function</span>
      <select class="input" bind:value={targetIdx} oninput={onConfigChange}>
        {#each targets as t, i}
          <option value={i}>{t.name}</option>
        {/each}
      </select>
    </label>
    <label class="label w-36">
      <span>Hidden width: {hidden}</span>
      <input type="range" class="input" min="2" max="10" step="1" bind:value={hidden} oninput={onConfigChange} />
    </label>
    <label class="label w-36">
      <span>Spline grid G: {gridSize}</span>
      <input type="range" class="input" min="3" max="16" step="1" bind:value={gridSize} oninput={onConfigChange} />
    </label>
    <label class="label w-36">
      <span>Spline order k: {order}</span>
      <input type="range" class="input" min="2" max="5" step="1" bind:value={order} oninput={onConfigChange} />
    </label>
    <label class="label w-36">
      <span>Epochs/frame: {epochsPerFrame}</span>
      <input type="range" class="input" min="1" max="20" step="1" bind:value={epochsPerFrame} />
    </label>
    <label class="label w-40">
      <span>Learning rate</span>
      <select class="input" bind:value={lr}>
        <option value={0.002}>0.002</option>
        <option value={0.005}>0.005</option>
        <option value={0.01}>0.01</option>
        <option value={0.02}>0.02 (default)</option>
        <option value={0.05}>0.05</option>
        <option value={0.1}>0.1</option>
      </select>
    </label>
    <label class="label w-44">
      <span>Training samples: {samplesPerAxis}²</span>
      <input type="range" class="input" min="8" max="40" step="1" bind:value={samplesPerAxis} oninput={onConfigChange} />
    </label>
    <label class="label w-40">
      <span>Render grid: {evalSize}²</span>
      <input type="range" class="input" min="64" max="192" step="8" bind:value={evalSize} oninput={() => refreshGrid()} />
    </label>
  </div>

  <div class="flex gap-2 items-center flex-wrap">
    <button class="btn preset-filled-primary" onclick={() => { playing = !playing; if (playing) rafId = requestAnimationFrame(loop); }}>
      {playing ? '⏸ Pause' : '▶ Train'}
    </button>
    <button class="btn preset-tonal-surface" onclick={() => { reset(); playing = true; rafId = requestAnimationFrame(loop); }}>↺ Reset weights</button>
    <span class="badge preset-tonal-primary">epoch {epoch}</span>
    <span class="badge preset-tonal-warning">rmse {rmse.toExponential(2)}</span>
    {#if bestRmse < Infinity}
      <span class="badge preset-tonal-success">best {bestRmse.toExponential(2)}</span>
    {/if}
    <span class="badge preset-tonal-surface">params {nParams}</span>
    <span class="badge preset-tonal-surface">{frameMs.toFixed(1)} ms/train</span>
  </div>

  <!-- heatmaps -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <div class="space-y-2">
      <p class="text-sm text-surface-300 font-medium">Target — <code class="code-block text-xs">{targets[targetIdx].name}</code></p>
      <canvas bind:this={truthCanvas} class="rounded border border-surface-700 spectrum-canvas w-full"
              style="aspect-ratio:1/1; image-rendering:pixelated;"></canvas>
    </div>
    <div class="space-y-2">
      <p class="text-sm text-surface-300 font-medium">KAN learned — <code class="code-block text-xs">{targets[targetIdx].short}</code></p>
      <canvas bind:this={learnedCanvas} class="rounded border border-surface-700 spectrum-canvas w-full"
              style="aspect-ratio:1/1; image-rendering:pixelated;"></canvas>
    </div>
  </div>

  <!-- loss curve -->
  <div class="space-y-2">
    <p class="text-sm text-surface-300 font-medium">Training RMSE (log scale) — full-batch Adam, all {samplesPerAxis * samplesPerAxis} samples per epoch</p>
    <canvas bind:this={lossCanvas} width="880" height="200"
            class="rounded border border-surface-700 bg-surface-950/40 w-full"></canvas>
  </div>

  <!-- learned edge splines -->
  <div class="space-y-2">
    <p class="text-sm text-surface-300 font-medium">
      Learned edge functions φ(x) — each edge of the network is a B-spline (plus a SiLU residual),
      not a scalar weight. <span class="text-cyan-400">x→hidden</span> ·
      <span class="text-amber-400">y→hidden</span> ·
      <span class="text-violet-400">hidden→output</span>. Dashed lines: spline grid bounds at ±1.
    </p>
    <canvas bind:this={splineCanvas} width="880" height="240"
            class="rounded border border-surface-700 bg-surface-950/40 w-full"></canvas>
  </div>

</div>
