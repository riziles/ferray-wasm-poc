<script lang="ts">
  import { onMount, tick } from 'svelte';

  let { wasm } = $props<{ wasm: ReturnType<typeof import('./wasm/loader').getWasm> }>();

  let winType = $state('hamming');
  let peakFreq = $state('');
  let canvasTime: HTMLCanvasElement;
  let canvasFreq: HTMLCanvasElement;
  let infoText = $state('');

  function run() {
    if (!canvasTime || !canvasFreq) return;

    const sampleRate = 1000;
    const n = 512;
    const freqs = new Float64Array([50, 120, 300]);
    const amps = new Float64Array([1.0, 0.5, 0.25]);

    // Generate composite signal
    const sigArr = wasm.composite_signal(freqs, amps, n);

    // Window the signal
    const winArr = wasm.window_fn(winType, n);
    const windowed = new Float64Array(n);
    for (let i = 0; i < n; i++) {
      windowed[i] = Number(sigArr[i]) * Number(winArr[i]);
    }

    // FFT
    const t0 = performance.now();
    const magArr = wasm.fft_magnitude(windowed);
    const elapsed = performance.now() - t0;

    const mag: number[] = Array.from(magArr);
    const half = Math.floor(n / 2);

    // Find peak
    let peakIdx = 1;
    let peakVal = 0;
    for (let i = 1; i < half; i++) {
      if (mag[i] > peakVal) { peakVal = mag[i]; peakIdx = i; }
    }
    const peakHz = Math.round(peakIdx * sampleRate / n);
    peakFreq = `${peakHz} Hz`;
    infoText = `FFT in ${elapsed.toFixed(2)} ms · ${n} points · window: ${winType}`;

    // --- Draw time domain ---
    const ctxT = canvasTime.getContext('2d')!;
    const cw = canvasTime.width, ch = canvasTime.height;
    ctxT.clearRect(0, 0, cw, ch);
    ctxT.fillStyle = '#1a202c';
    ctxT.fillRect(0, 0, cw, ch);

    ctxT.strokeStyle = '#3b82f6';
    ctxT.lineWidth = 1.5;
    ctxT.beginPath();
    for (let i = 0; i < windowed.length; i++) {
      const x = 10 + (i / (windowed.length - 1)) * (cw - 20);
      const y = ch / 2 - windowed[i] * (ch / 2 - 10);
      if (i === 0) ctxT.moveTo(x, y); else ctxT.lineTo(x, y);
    }
    ctxT.stroke();

    ctxT.font = '11px monospace';
    ctxT.fillStyle = '#94a3b8';
    ctxT.fillText('Time domain (windowed)', 12, 18);

    // --- Draw frequency domain ---
    const ctxF = canvasFreq.getContext('2d')!;
    ctxF.clearRect(0, 0, cw, ch);
    ctxF.fillStyle = '#1a202c';
    ctxF.fillRect(0, 0, cw, ch);

    // Bar chart
    const barW = (cw - 20) / half - 1;
    const maxMag = Math.max(...mag.slice(1, half), 1);
    for (let i = 1; i < half; i++) {
      const barH = (mag[i] / maxMag) * (ch - 30);
      const x = 10 + i * (cw - 20) / half;
      const isPeak = i === peakIdx;
      ctxF.fillStyle = isPeak ? '#f59e0b' : '#6366f1';
      ctxF.fillRect(x, ch - 10 - barH, Math.max(barW, 1), barH);
    }

    ctxF.font = '11px monospace';
    ctxF.fillStyle = '#94a3b8';
    ctxF.fillText('Frequency domain (magnitude)', 12, 18);
  }

  $effect(() => { winType; tick().then(run); });
  onMount(run);
</script>

<div class="card card-demo p-6 space-y-4">

  <div class="flex gap-3 items-end flex-wrap">
    <label class="label w-40">
      <span>Window</span>
      <select class="input" bind:value={winType}>
        <option>hanning</option>
        <option>hamming</option>
        <option>blackman</option>
        <option>bartlett</option>
        <option>cosine</option>
        <option>nuttall</option>
        <option>parzen</option>
      </select>
    </label>
    {#if peakFreq}
      <span class="badge preset-tonal-warning">Peak: {peakFreq}</span>
    {/if}
    {#if infoText}
      <span class="badge preset-tonal">{infoText}</span>
    {/if}
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <canvas bind:this={canvasTime} width="400" height="200"
            class="w-full rounded border border-surface-700 spectrum-canvas"></canvas>
    <canvas bind:this={canvasFreq} width="400" height="200"
            class="w-full rounded border border-surface-700 spectrum-canvas"></canvas>
  </div>
</div>
