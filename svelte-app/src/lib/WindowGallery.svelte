<script lang="ts">
  import { onMount, tick } from 'svelte';

  let { wasm } = $props<{ wasm: ReturnType<typeof import('./wasm/loader').getWasm> }>();

  let points = $state(256);
  let canvas: HTMLCanvasElement;

  const windows = [
    { name: 'hanning', color: '#3b82f6', label: 'Hanning' },
    { name: 'hamming', color: '#f59e0b', label: 'Hamming' },
    { name: 'blackman', color: '#22c55e', label: 'Blackman' },
    { name: 'bartlett', color: '#a855f7', label: 'Bartlett' },
    { name: 'cosine', color: '#ef4444', label: 'Cosine' },
    { name: 'nuttall', color: '#f97316', label: 'Nuttall' },
    { name: 'parzen', color: '#06b6d4', label: 'Parzen' },
  ];

  function draw() {
    if (!canvas) return;
    const ctx = canvas.getContext('2d')!;
    const cw = canvas.width, ch = canvas.height;
    ctx.clearRect(0, 0, cw, ch);
    ctx.fillStyle = '#1a202c';
    ctx.fillRect(0, 0, cw, ch);

    // Grid
    ctx.strokeStyle = '#2d3748';
    ctx.lineWidth = 0.5;
    for (let y = 0.2; y <= 1.0; y += 0.2) {
      const py = 20 + y * (ch - 40);
      ctx.beginPath(); ctx.moveTo(20, ch - py + 20); ctx.lineTo(cw - 20, ch - py + 20); ctx.stroke();
    }

    // Each window
    for (const w of windows) {
      try {
        const arr = wasm.window_fn(w.name, points);
        ctx.strokeStyle = w.color;
        ctx.lineWidth = 2;
        ctx.beginPath();
        for (let i = 0; i < arr.length; i++) {
          const x = 20 + (i / (arr.length - 1)) * (cw - 40);
          const y = ch - 20 - Number(arr[i]) * (ch - 40);
          if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
        }
        ctx.stroke();
      } catch (e) {
        console.warn(`Window ${w.name} failed:`, e);
      }
    }
  }

  $effect(() => { points; tick().then(draw); });
  onMount(draw);
</script>

<div class="card card-demo p-6 space-y-4">

  <label class="label w-48">
    <span>Points: {points}</span>
    <input type="range" class="input" min="32" max="512" bind:value={points} />
  </label>

  <canvas bind:this={canvas} width="600" height="300"
          class="w-full rounded border border-surface-700 spectrum-canvas"></canvas>

  <div class="flex flex-wrap gap-3">
    {#each windows as w}
      <span class="badge" style="background:{w.color}22;color:{w.color};border:1px solid {w.color}44">
        ■ {w.label}
      </span>
    {/each}
  </div>
</div>
