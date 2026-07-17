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
    <h2 class="h2">What is a sine wave?</h2>

    <p class="text-surface-300">
      A sine wave is a smooth, repeating oscillation that cycles between −1 and +1.
      The <strong>frequency</strong> controls how many complete cycles fit in your
      data — at 3 Hz with 200 samples, you'll see 3 complete waves.
    </p>

    <p class="text-surface-300">
      Each point is y(t) = sin(2π · frequency · t), with t evenly spaced from 0 to 1.
      The statistics below the graph summarize the full wave: minimum, maximum, mean,
      median, and standard deviation — all computed by <code class="code-block">ferray-stats</code>
      in a single pass over the array.
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
