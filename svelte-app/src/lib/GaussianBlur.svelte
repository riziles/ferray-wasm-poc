<script lang="ts">
  import { onMount, tick } from 'svelte';

  let { wasm } = $props<{ wasm: ReturnType<typeof import('./wasm/loader').getWasm> }>();

  let sigma = $state(3);
  let kernelSize = $state(5);
  let canvas: HTMLCanvasElement;
  let resultText = $state('');
  let elapsed = $state(0);

  function run() {
    if (!canvas) return;
    const n = 200;
    const data = wasm.sine_wave(3, n);

    // Add noise
    const noisy = new Float64Array(n);
    for (let i = 0; i < n; i++) {
      noisy[i] = Number(data[i]) + (Math.random() - 0.5) * 0.5;
    }

    const t0 = performance.now();
    const smoothedArr = wasm.gaussian_blur(noisy, sigma, kernelSize);
    elapsed = performance.now() - t0;

    const rounded = (x: number) => Math.round(x * 1000) / 1000;
    const diff = Array.from(smoothedArr).reduce((s, v, i) => s + Math.abs(v - Number(data[i])), 0);
    resultText = `RMSE: ${rounded(diff / n)} · ${elapsed.toFixed(2)} ms`;

    const ctx = canvas.getContext('2d')!;
    const cw = canvas.width;
    const ch = canvas.height;
    ctx.clearRect(0, 0, cw, ch);

    // Background grid
    ctx.strokeStyle = '#2d3748';
    ctx.lineWidth = 0.5;
    for (let y = 20; y < ch - 10; y += 30) {
      ctx.beginPath(); ctx.moveTo(30, y); ctx.lineTo(cw - 20, y); ctx.stroke();
    }

    // Noisy signal
    ctx.strokeStyle = '#ef4444';
    ctx.lineWidth = 1;
    ctx.beginPath();
    for (let i = 0; i < n; i++) {
      const x = 30 + (i / (n - 1)) * (cw - 50);
      const y = ch / 2 - noisy[i] * (ch / 2 - 15);
      if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
    }
    ctx.stroke();

    // Smoothed signal
    ctx.strokeStyle = '#22c55e';
    ctx.lineWidth = 2;
    ctx.beginPath();
    for (let i = 0; i < smoothedArr.length; i++) {
      const x = 30 + (i / (smoothedArr.length - 1)) * (cw - 50);
      const y = ch / 2 - Number(smoothedArr[i]) * (ch / 2 - 15);
      if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
    }
    ctx.stroke();

    // Legend
    ctx.font = '11px monospace';
    ctx.fillStyle = '#ef4444'; ctx.fillText('Noisy', cw - 100, 20);
    ctx.fillStyle = '#22c55e'; ctx.fillText('Smoothed', cw - 100, 36);
  }

  $effect(() => { sigma; kernelSize; tick().then(run); });
  onMount(run);
</script>

<div class="card card-demo p-6 space-y-4">

  <div class="flex gap-3 items-end flex-wrap">
    <label class="label w-32">
      <span>Sigma</span>
      <input type="number" class="input" min="0.5" max="10" step="0.5" bind:value={sigma} />
    </label>
    <label class="label w-32">
      <span>Kernel Size</span>
      <input type="number" class="input" min="3" max="21" step="2" bind:value={kernelSize} />
    </label>
  </div>

  <canvas bind:this={canvas} width="600" height="200"
          class="w-full rounded bg-surface-800 border border-surface-700"></canvas>

  {#if resultText}
    <span class="badge preset-tonal-success">{resultText}</span>
  {/if}
</div>
