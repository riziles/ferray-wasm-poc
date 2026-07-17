<script lang="ts">
  import { onMount, tick } from 'svelte';

  let { wasm } = $props<{ wasm: ReturnType<typeof import('./wasm/loader').getWasm> }>();

  let fnType = $state('Eggholder function');
  let freq = $state(4);
  let canvas: HTMLCanvasElement;
  let elapsed = $state(0);

  const fnOptions = ['sinc', 'ripple', 'gaussian', 'spiral waves', 'Eggholder function'];

  function mapFnName(d: string): string {
    switch (d) {
      case 'spiral waves': return 'spiral';
      case 'Eggholder function': return 'eggholder';
      default: return d;
    }
  }

  function rainbow(t: number): [number, number, number] {
    // t in [0, 1], returns [r, g, b]
    const r = Math.max(0, Math.min(1, Math.sin(t * Math.PI * 2 - Math.PI / 2) * 0.5 + 0.5));
    const g = Math.max(0, Math.min(1, Math.sin(t * Math.PI * 2 + Math.PI / 6) * 0.5 + 0.5));
    const b = Math.max(0, Math.min(1, Math.sin(t * Math.PI * 2 + 5 * Math.PI / 6) * 0.5 + 0.5));
    return [Math.round(r * 255), Math.round(g * 255), Math.round(b * 255)];
  }

  function draw() {
    if (!canvas) return;
    const size = 200;
    canvas.width = size;
    canvas.height = size;

    const t0 = performance.now();
    const data = wasm.radial_2d(size, mapFnName(fnType), freq);
    elapsed = performance.now() - t0;

    const arr: number[] = Array.from(data);
    let minV = Infinity, maxV = -Infinity;
    for (const v of arr) { if (v < minV) minV = v; if (v > maxV) maxV = v; }
    const range = (maxV - minV) || 1;

    const ctx = canvas.getContext('2d')!;
    const img = ctx.createImageData(size, size);
    for (let y = 0; y < size; y++) {
      for (let x = 0; x < size; x++) {
        const idx = y * size + x;
        const t = (arr[idx] - minV) / range;
        const [r, g, b] = rainbow(Math.max(0, Math.min(1, t)));
        const px = idx * 4;
        img.data[px] = r;
        img.data[px + 1] = g;
        img.data[px + 2] = b;
        img.data[px + 3] = 255;
      }
    }
    ctx.putImageData(img, 0, 0);
  }

  $effect(() => { fnType; freq; tick().then(draw); });
  onMount(draw);
</script>

<div class="card card-demo p-6 space-y-4">

  <div class="flex gap-3 items-end flex-wrap">
    <label class="label w-48">
      <span>Function</span>
      <select class="input" bind:value={fnType}>
        {#each fnOptions as opt}
          <option>{opt}</option>
        {/each}
      </select>
    </label>
    <label class="label w-48">
      <span>Frequency: {freq}</span>
      <input type="range" class="input" min="1" max="20" step="0.1" bind:value={freq} />
    </label>
    {#if elapsed}
      <span class="badge preset-tonal-primary">{elapsed.toFixed(2)} ms</span>
    {/if}
  </div>

  <canvas bind:this={canvas} width="200" height="200"
          class="rounded border border-surface-700 spectrum-canvas"
          style="width:400px;height:400px"></canvas>
</div>
