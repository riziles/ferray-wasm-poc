<script lang="ts">
  import { getContext } from 'svelte';
  import type { WasmApi } from '$lib/wasm/loader';
  import Heatmap from '$lib/Heatmap.svelte';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());

  const rustCode = `use ferray::Array2;
use ferray_ufunc::{sin, exp};

#[wasm_bindgen]
pub fn radial_2d(size: usize, fn_type: &str, freq: f64) -> Vec<f64> {
    let half = size as f64 / 2.0;
    let mut data = Array2::zeros((size, size));

    for y in 0..size {
        for x in 0..size {
            let dx = x as f64 - half;
            let dy = y as f64 - half;
            let r = (dx * dx + dy * dy).sqrt() / half * freq;
            let theta = dy.atan2(dx);

            data[(y, x)] = match fn_type {
                "sinc" => if r.abs() < 1e-6 { 1.0 }
                          else { r.sin() / r },
                "gaussian" => (-r * r * 0.5).exp(),
                "ripple" => (freq * r).sin() / (1.0 + r / 10.0),
                "spiral" => (freq * r - theta).sin(),
                "eggholder" => {
                    let xi = dx / 20.0;
                    let yi = dy / 20.0 + 47.0;
                    -(yi + 47.0) * ((xi / 2.0 + yi).abs().sqrt()).sin()
                        - xi * ((xi - yi).abs().sqrt()).sin()
                },
                _ => 0.0,
            };
        }
    }
    data.into_raw_vec()
}`;
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl space-y-8">

  <h1 class="h1">🔥 2D Function Heatmap</h1>
  <p class="text-lg text-surface-400">
    40,000 evaluations in a single WASM call, rendered to canvas <code class="code-block">ImageData</code>.
  </p>

  <Heatmap {wasm} />

  <section class="card card-demo p-6 space-y-4">
    <h2 class="h2">How it works</h2>

    <p class="text-surface-300">
      For each pixel (x, y) in a 200×200 grid, a radial distance r is computed from the center,
      then fed into a chosen 2D function. The result is normalized and mapped through a rainbow
      colormap to produce the final image.
    </p>

    <h3 class="h3 mt-4">Available functions</h3>

    <div class="overflow-x-auto">
      <table class="table table-hover text-xs">
        <thead><tr><th>Function</th><th>Formula</th><th>Appearance</th></tr></thead>
        <tbody>
          <tr>
            <td class="font-mono">sinc</td>
            <td class="font-mono">sin(r) / r</td>
            <td class="text-surface-400">Concentric rings, decaying amplitude</td>
          </tr>
          <tr>
            <td class="font-mono">ripple</td>
            <td class="font-mono">sin(f·r) / (1 + r/10)</td>
            <td class="text-surface-400">Water-drop ripples</td>
          </tr>
          <tr>
            <td class="font-mono">gaussian</td>
            <td class="font-mono">exp(−r² / 2σ²)</td>
            <td class="text-surface-400">Radial gradient</td>
          </tr>
          <tr>
            <td class="font-mono">spiral waves</td>
            <td class="font-mono">sin(f·r − θ)</td>
            <td class="text-surface-400">Archimedean spiral interference</td>
          </tr>
          <tr>
            <td class="font-mono">Eggholder</td>
            <td class="font-mono">−(y+47)·sin√|x/2 + (y+47)| − x·sin√|x − (y+47)|</td>
            <td class="text-surface-400">Complex optimization landscape</td>
          </tr>
        </tbody>
      </table>
    </div>

    <h3 class="h3 mt-4">Rust implementation</h3>
    <pre class="code-block p-4 overflow-x-auto text-xs"><code>{rustCode}</code></pre>

    <h3 class="h3 mt-4">Performance</h3>
    <p class="text-surface-400 text-sm">
      40,000 function evaluations (200×200 grid) complete in 1—3 ms depending on function
      complexity. The bottleneck is not WASM computation but the JavaScript colormap mapping
      and <code class="code-block">putImageData</code> call — both are O(pixels) and run in
      0.5—1 ms.
    </p>
  </section>

</div>
