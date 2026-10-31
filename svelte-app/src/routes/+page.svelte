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
  import BarbaultBasket from '$lib/BarbaultBasket.svelte';
  import Kan from '$lib/Kan.svelte';
  import Wormhole from '$lib/Wormhole.svelte';
  import TechStack from '$lib/TechStack.svelte';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl space-y-12">
  <Hero />

  <section class="group/demo border border-primary-700/40 rounded-lg p-4 bg-primary-950/10">
    <a href="{base}/eth" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-2 cursor-pointer">🧨 The Exponential Time Hypothesis is False <span class="badge preset-filled-primary-500 text-xs">NEW</span></h2>
    </a>
    <p class="text-sm text-surface-400 mb-3">
      OpenAI's October 2026 #SAT algorithm refutes ETH. Three exponents on one axis — the abstract's,
      Theorem 1.1's, and the one Lean actually checks — a ruler that puts the crossovers past every
      physical quantity, a ledger of which conditional lower bounds just went vacuous, a live
      counting-preserving rewrite, and the Bluesky thread that opened with “3 SAT in sub exponential
      time just seems absurd to me”, quoted in full with links.
      <span class="text-surface-500">(Pure TypeScript — no WASM needed.)</span>
    </p>
    <div class="flex flex-wrap gap-2 text-xs">
      <span class="badge preset-tonal">4 panels</span>
      <span class="badge preset-tonal">the thread, verbatim</span>
      <span class="badge preset-tonal-warning">claims marked as claims</span>
      <a href="{base}/eth" data-sveltekit-prefetch class="badge preset-filled-primary-500">Read the ladder →</a>
    </div>
  </section>

  <section class="group/demo border border-primary-700/40 rounded-lg p-4 bg-primary-950/10">
    <a href="{base}/kakeya" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-2 cursor-pointer">🪡 The Kakeya Conjecture <span class="badge preset-filled-primary-500 text-xs">NEW</span></h2>
    </a>
    <p class="text-sm text-surface-400 mb-3">
      Five live experiments on the needle problem: turn a unit needle in four rooms, slide a Perron
      tree until its area vanishes, count boxes to measure the dimension yourself, compress a family
      of tubes, and focus a wave. Then the honest ledger on OpenAI's October 2026 claim that its
      models proved local smoothing — and with it Kakeya — in every dimension.
      <span class="text-surface-500">(Pure TypeScript — no WASM needed.)</span>
    </p>
    <div class="flex flex-wrap gap-2 text-xs">
      <span class="badge preset-tonal">5 interactive demos</span>
      <span class="badge preset-tonal">measured, not quoted</span>
      <span class="badge preset-tonal-warning">claims marked as claims</span>
      <a href="{base}/kakeya" data-sveltekit-prefetch class="badge preset-filled-primary-500">Turn the needle →</a>
    </div>
  </section>

  <section class="group/demo border border-primary-700/40 rounded-lg p-4 bg-primary-950/10">
    <a href="{base}/objects" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-2 cursor-pointer">🧊 Object Folio <span class="badge preset-filled-primary-500 text-xs">NEW</span></h2>
    </a>
    <p class="text-sm text-surface-400 mb-3">
      Eight interactive Three.js objects in one canvas — a torus knot, a Klein bottle, a Möbius
      strip, a superformula star and friends — plus a catoid wrapped around them with an animated
      tail and a Cheshire grin. Orbit them, spin them, strip them to wireframe, or read the maths
      behind each one. <span class="text-surface-500">(Three.js + Threlte — no WASM needed.)</span>
    </p>
    <div class="flex flex-wrap gap-2 text-xs">
      <span class="badge preset-tonal">8 objects + 1 catoid</span>
      <span class="badge preset-tonal">4 materials</span>
      <span class="badge preset-tonal">4 hand-built surfaces</span>
      <a href="{base}/objects" data-sveltekit-prefetch class="badge preset-filled-primary-500">Open the folio →</a>
    </div>
  </section>

  <section class="group/demo border border-primary-700/40 rounded-lg p-4 bg-primary-950/10">
    <a href="{base}/qcd" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-2 cursor-pointer">🌈 Quantum Chromodynamics <span class="badge preset-filled-primary-500 text-xs">NEW</span></h2>
    </a>
    <p class="text-sm text-surface-400 mb-3">
      An interactive descent into the strong force: colour charge, gluon self-coupling, confinement,
      asymptotic freedom, the not-empty vacuum, and a theory whose mathematical existence is still an
      open problem. Every picture is a labelled cartoon, and the labels get worse as the physics gets
      better. <span class="text-surface-500">(Pure TypeScript — no WASM needed.)</span>
    </p>
    <div class="flex flex-wrap gap-2 text-xs">
      <span class="badge preset-tonal">9 acts</span>
      <span class="badge preset-tonal">8 interactive demos</span>
      <span class="badge preset-tonal-warning">picture honesty: 78% → 4%</span>
      <a href="{base}/qcd" data-sveltekit-prefetch class="badge preset-filled-primary-500">Enter the descent →</a>
    </div>
  </section>

  <section class="group/demo border border-primary-700/40 rounded-lg p-4 bg-primary-950/10">
    <a href="{base}/wormhole" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🌀 Wormhole Embedding Diagram <span class="badge preset-filled-primary-500 text-xs">NEW</span></h2>
    </a>
    <Wormhole {wasm} />
  </section>

  <section class="group/demo border border-primary-700/40 rounded-lg p-4 bg-primary-950/10">
    <a href="{base}/kan" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🧠 Kolmogorov–Arnold Network <span class="badge preset-filled-primary-500 text-xs">NEW</span></h2>
    </a>
    <Kan {wasm} />
  </section>

  <section class="group/demo border border-primary-700/40 rounded-lg p-4 bg-primary-950/10">
    <a href="{base}/jacobian" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🧮 Jacobian Conjecture Counterexample <span class="badge preset-filled-error-500 text-xs">NEW</span></h2>
    </a>
    <Jacobian {wasm} />
  </section>

  <section class="group/demo border border-amber-700/30 rounded-lg p-4 bg-amber-950/5">
    <a href="{base}/counterexample-lab" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🔬 Counterexample Lab <span class="badge preset-filled-warning-500 text-xs">NEW</span></h2>
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

  <section class="group/demo">
    <a href="{base}/barbault-basket" data-sveltekit-prefetch class="block hover:text-primary-400 transition-colors">
      <h2 class="h2 mb-4 cursor-pointer">🪐 Barbault Basket <span class="badge preset-filled-primary-500 text-xs">NEW</span></h2>
    </a>
    <BarbaultBasket {wasm} />
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
