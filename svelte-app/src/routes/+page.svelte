<script lang="ts">
  import { getContext } from 'svelte';
  import { base } from '$app/paths';
  import type { WasmApi } from '$lib/wasm/loader';
  import Hero from '$lib/Hero.svelte';
  import SumOfSquares from '$lib/SumOfSquares.svelte';
  import WaveAndStats from '$lib/WaveAndStats.svelte';
  import GaussianBlur from '$lib/GaussianBlur.svelte';
  import FftAnalyzer from '$lib/FftAnalyzer.svelte';
  import WindowGallery from '$lib/WindowGallery.svelte';
  import Heatmap from '$lib/Heatmap.svelte';
  import Jacobian from '$lib/Jacobian.svelte';
  import CounterexampleLab from '$lib/CounterexampleLab.svelte';
  import TechStack from '$lib/TechStack.svelte';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl space-y-12">
  <Hero />

  <section class="group/demo border border-primary-700/40 rounded-lg p-4 bg-primary-950/10">
    <a href="{base}/jacobian" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🧮 Jacobian Conjecture Counterexample <span class="badge preset-filled-error text-xs">NEW</span></h2>
    </a>
    <Jacobian {wasm} />
  </section>

  <section class="group/demo border border-amber-700/30 rounded-lg p-4 bg-amber-950/5">
    <a href="{base}/counterexample-lab" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🔬 Counterexample Lab <span class="badge preset-filled-warning text-xs">NEW</span></h2>
    </a>
    <CounterexampleLab {wasm} />
  </section>

  <section class="group/demo">
    <a href="{base}/sum-of-squares" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">📐 Sum of Squares</h2>
    </a>
    <SumOfSquares {wasm} />
  </section>

  <section class="group/demo">
    <a href="{base}/sine-wave" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🌊 Sine Wave + Live Stats</h2>
    </a>
    <WaveAndStats {wasm} />
  </section>

  <section class="group/demo">
    <a href="{base}/gaussian-blur" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🔵 Gaussian Blur (1D Convolution)</h2>
    </a>
    <GaussianBlur {wasm} />
  </section>

  <section class="group/demo">
    <a href="{base}/fft" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">📈 FFT Spectrum Analyzer</h2>
    </a>
    <FftAnalyzer {wasm} />
  </section>

  <section class="group/demo">
    <a href="{base}/window-functions" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🪟 Window Function Gallery</h2>
    </a>
    <WindowGallery {wasm} />
  </section>

  <section class="group/demo">
    <a href="{base}/heatmap" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🔥 2D Function Heatmap</h2>
    </a>
    <Heatmap {wasm} />
  </section>

  <TechStack />

  <footer class="text-center text-xs text-surface-500 pb-8 space-y-2">
    <div>
      Built with
      <a href="https://github.com/dollspace-gay/ferray" target="_blank" class="text-primary-400 hover:text-primary-300 underline">Ferray</a> +
      <a href="https://www.rust-lang.org" target="_blank" class="text-primary-400 hover:text-primary-300 underline">Rust</a> +
      <a href="https://webassembly.org" target="_blank" class="text-primary-400 hover:text-primary-300 underline">WASM</a> +
      <a href="https://svelte.dev/docs/kit" target="_blank" class="text-primary-400 hover:text-primary-300 underline">SvelteKit</a> +
      <a href="https://skeletonlabs.dev" target="_blank" class="text-primary-400 hover:text-primary-300 underline">Skeleton v4</a>
    </div>
    <div>
      <a href="https://github.com/riziles/ferray-wasm-poc" target="_blank" class="text-surface-600 hover:text-surface-400 underline">View source on GitHub</a>
    </div>
  </footer>
</div>
