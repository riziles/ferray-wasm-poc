<script lang="ts">
  import { onMount } from 'svelte';
  import { initWasm, type WasmApi } from './lib/wasm/loader';
  import Hero from './lib/Hero.svelte';
  import SumOfSquares from './lib/SumOfSquares.svelte';
  import WaveAndStats from './lib/WaveAndStats.svelte';
  import GaussianBlur from './lib/GaussianBlur.svelte';
  import FftAnalyzer from './lib/FftAnalyzer.svelte';
  import WindowGallery from './lib/WindowGallery.svelte';
  import Heatmap from './lib/Heatmap.svelte';
  import TechStack from './lib/TechStack.svelte';

  let wasm = $state<WasmApi | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      wasm = await initWasm();
    } catch (e) {
      error = String(e);
    }
  });
</script>

{#if error}
  <main class="min-h-screen flex items-center justify-center bg-surface-900 p-8">
    <div class="card preset-filled-error p-8 max-w-md text-center">
      <h1 class="h1 mb-2">⚠️ WASM Init Failed</h1>
      <p class="text-xs font-mono opacity-80 break-all">{error}</p>
    </div>
  </main>
{:else if !wasm}
  <main class="min-h-screen flex items-center justify-center bg-surface-900 p-8">
    <div class="flex flex-col items-center gap-4">
      <div class="spinner"></div>
      <p class="text-lg text-surface-400">Loading Ferray WASM…</p>
    </div>
  </main>
{:else}
  <main class="min-h-screen bg-surface-900">
    <div class="container mx-auto px-4 py-8 max-w-5xl space-y-12">
      <Hero />
      <SumOfSquares {wasm} />
      <WaveAndStats {wasm} />
      <GaussianBlur {wasm} />
      <FftAnalyzer {wasm} />
      <WindowGallery {wasm} />
      <Heatmap {wasm} />
      <TechStack />
      <footer class="text-center text-xs text-surface-500 pb-8">
        Built with <span class="text-surface-400">Ferray</span> + <span class="text-surface-400">Rust</span> + <span class="text-surface-400">WASM</span> + <span class="text-surface-400">Svelte 5</span> + <span class="text-surface-400">Skeleton v4</span>
      </footer>
    </div>
  </main>
{/if}
