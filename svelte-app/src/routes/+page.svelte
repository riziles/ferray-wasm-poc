<script lang="ts">
  import { getContext } from 'svelte';
  import type { WasmApi } from '$lib/wasm/loader';
  import Hero from '$lib/Hero.svelte';
  import SumOfSquares from '$lib/SumOfSquares.svelte';
  import WaveAndStats from '$lib/WaveAndStats.svelte';
  import GaussianBlur from '$lib/GaussianBlur.svelte';
  import FftAnalyzer from '$lib/FftAnalyzer.svelte';
  import WindowGallery from '$lib/WindowGallery.svelte';
  import Heatmap from '$lib/Heatmap.svelte';
  import TechStack from '$lib/TechStack.svelte';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl space-y-12">
  <Hero />

  <section class="group/demo">
    <a href="/ferray-wasm-poc/sum-of-squares" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">📐 Sum of Squares</h2>
    </a>
    <SumOfSquares {wasm} />
  </section>

  <section class="group/demo">
    <a href="/ferray-wasm-poc/sine-wave" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🌊 Sine Wave + Live Stats</h2>
    </a>
    <WaveAndStats {wasm} />
  </section>

  <section class="group/demo">
    <a href="/ferray-wasm-poc/gaussian-blur" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🔵 Gaussian Blur (1D Convolution)</h2>
    </a>
    <GaussianBlur {wasm} />
  </section>

  <section class="group/demo">
    <a href="/ferray-wasm-poc/fft" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">📈 FFT Spectrum Analyzer</h2>
    </a>
    <FftAnalyzer {wasm} />
  </section>

  <section class="group/demo">
    <a href="/ferray-wasm-poc/window-functions" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🪟 Window Function Gallery</h2>
    </a>
    <WindowGallery {wasm} />
  </section>

  <section class="group/demo">
    <a href="/ferray-wasm-poc/heatmap" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🔥 2D Function Heatmap</h2>
    </a>
    <Heatmap {wasm} />
  </section>

  <TechStack />

  <footer class="text-center text-xs text-surface-500 pb-8">
    Built with <span class="text-surface-400">Ferray</span> + <span class="text-surface-400">Rust</span> + <span class="text-surface-400">WASM</span> + <span class="text-surface-400">SvelteKit</span> + <span class="text-surface-400">Skeleton v4</span>
  </footer>
</div>
