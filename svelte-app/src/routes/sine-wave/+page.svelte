<script lang="ts">
  import { getContext } from 'svelte';
  import type { WasmApi } from '$lib/wasm/loader';
  import WaveAndStats from '$lib/WaveAndStats.svelte';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());

  const rustCode = `use ferray::Array1;
use ferray_ufunc::sin;
use ferray_stats;

#[wasm_bindgen]
pub fn sine_wave(freq: f64, samples: usize) -> Vec<f64> {
    let x = Array1::linspace(0.0, freq * 2.0 * std::f64::consts::PI, samples);
    sin(&x).unwrap().into_raw_vec()
}

#[wasm_bindgen]
pub fn stats(arr: &[f64]) -> String {
    let a = Array1::from_vec(arr.to_vec());
    format!(
        "min={:.4}, max={:.4}, mean={:.4}, median={:.4}, std={:.4}",
        ferray_stats::min(&a),
        ferray_stats::max(&a),
        ferray_stats::mean(&a).unwrap_or(0.0),
        ferray_stats::median(&a).unwrap_or(0.0),
        ferray_stats::std(&a, 1).unwrap_or(0.0),
    )
}`;
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl space-y-8">

  <h1 class="h1">🌊 Sine Wave + Live Stats</h1>
  <p class="text-lg text-surface-400">
    Generate a sine wave with adjustable frequency and samples, then compute
    statistical reductions in the same WASM call.
  </p>

  <WaveAndStats {wasm} />

  <section class="card card-demo p-6 space-y-4">
    <h2 class="h2">How it works</h2>

    <p class="text-surface-300">
      Uses <code class="code-block">ferray::linspace</code> to generate evenly-spaced points,
      then <code class="code-block">ferray_ufunc::sin</code> to compute element-wise sine.
      On native targets, sine is computed via Intel's CORE-MATH library (&lt; 0.5 ULP accuracy).
      On WASM, the pure-Rust <code class="code-block">libm</code> crate provides ~1—2 ULP accuracy —
      the same as NumPy.
    </p>

    <h3 class="h3 mt-4">Rust implementation</h3>
    <pre class="code-block p-4 overflow-x-auto text-xs"><code>{rustCode}</code></pre>

    <h3 class="h3 mt-4">Live reactivity</h3>
    <p class="text-surface-400 text-sm">
      Svelte 5 <code class="code-block">$effect</code> runes watch the slider values. Any change
      triggers a full redraw: compute the wave in WASM → render to canvas → compute stats →
      update badge text. All in under 1 ms.
    </p>
  </section>

</div>
