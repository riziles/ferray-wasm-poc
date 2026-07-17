<script lang="ts">
  import { onMount, tick } from 'svelte';

  let { wasm } = $props<{ wasm: ReturnType<typeof import('./wasm/loader').getWasm> }>();

  let freq = $state(3);
  let samples = $state(200);
  let canvas: HTMLCanvasElement;
  let statsText = $state('');

  function draw() {
    if (!canvas) return;
    const w = freq;
    const n = samples;
    const arr = wasm.sine_wave(w, n);
    const ctx = canvas.getContext('2d')!;
    const cw = canvas.width;
    const ch = canvas.height;

    ctx.clearRect(0, 0, cw, ch);

    // Axes
    ctx.strokeStyle = '#4a5568';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(30, ch / 2);
    ctx.lineTo(cw - 20, ch / 2);
    ctx.moveTo(30, 10);
    ctx.lineTo(30, ch - 10);
    ctx.stroke();

    // Wave
    ctx.strokeStyle = '#3b82f6';
    ctx.lineWidth = 2;
    ctx.beginPath();
    for (let i = 0; i < n; i++) {
      const x = 30 + (i / (n - 1)) * (cw - 50);
      const y = ch / 2 - (Number(arr[i])) * (ch / 2 - 15);
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.stroke();

    // Update stats
    updateStats();
  }

  function updateStats() {
    statsText = wasm.stats(wasm.sine_wave(freq, samples));
  }

  $effect(() => {
    // redraw when freq or samples change
    freq; samples;
    tick().then(draw);
  });

  onMount(() => {
    draw();
  });
</script>

<div class="card card-demo p-6 space-y-4">

  <div class="flex gap-4 flex-wrap items-end">
    <label class="label flex-1 min-w-[200px]">
      <span>Frequency: {freq} Hz</span>
      <input type="range" class="input" min="0.5" max="20" step="0.1" bind:value={freq} />
    </label>
    <label class="label w-40">
      <span>Samples: {samples}</span>
      <input type="range" class="input" min="10" max="2000" step="10" bind:value={samples} />
    </label>
    <label class="label w-24">
      <span>Exact</span>
      <input type="number" class="input" min="10" max="2000" bind:value={samples} />
    </label>
  </div>

  <canvas bind:this={canvas} width="600" height="200"
          class="w-full rounded bg-surface-800 border border-surface-700"></canvas>

  {#if statsText}
    <div class="flex flex-wrap gap-2 text-xs font-mono text-surface-300" style="white-space:pre-wrap">
      {#each statsText.split(', ') as part}
        <span class="badge preset-tonal">{part}</span>
      {/each}
    </div>
  {/if}
</div>
