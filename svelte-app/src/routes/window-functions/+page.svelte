<script lang="ts">
  import { getContext } from 'svelte';
  import type { WasmApi } from '$lib/wasm/loader';
  import WindowGallery from '$lib/WindowGallery.svelte';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());

  const rustCode = `use ferray::Array1;
use ferray_window;

#[wasm_bindgen]
pub fn window_fn(name: &str, size: usize) -> Vec<f64> {
    let arr: Array1<f64> = match name {
        "hanning" => ferray_window::hanning(size),
        "hamming" => ferray_window::hamming(size),
        "blackman" => ferray_window::blackman(size),
        "bartlett" => ferray_window::bartlett(size),
        "cosine"  => ferray_window::cosine(size),
        "nuttall" => ferray_window::nuttall(size),
        "parzen"  => ferray_window::parzen(size),
        _ => panic!("Unknown window: {name}"),
    };
    arr.into_raw_vec()
}`;
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl space-y-8">

  <h1 class="h1">🪟 Window Function Gallery</h1>
  <p class="text-lg text-surface-400">
    Overlays 7 standard window functions to visualize the tradeoff between main-lobe width and
    sidelobe suppression.
  </p>

  <WindowGallery {wasm} />

  <section class="card card-demo p-6 space-y-4">
    <h2 class="h2">Window function formulas</h2>

    <div class="overflow-x-auto">
      <table class="table table-hover text-xs">
        <thead><tr><th>Window</th><th>Formula (n = 0…N−1)</th></tr></thead>
        <tbody>
          <tr>
            <td class="font-mono">Hanning</td>
            <td class="font-mono">0.5 − 0.5·cos(2πn / (N−1))</td>
          </tr>
          <tr>
            <td class="font-mono">Hamming</td>
            <td class="font-mono">0.54 − 0.46·cos(2πn / (N−1))</td>
          </tr>
          <tr>
            <td class="font-mono">Blackman</td>
            <td class="font-mono">0.42 − 0.5·cos(2πn/(N−1)) + 0.08·cos(4πn/(N−1))</td>
          </tr>
          <tr>
            <td class="font-mono">Bartlett</td>
            <td class="font-mono">1 − |2n/(N−1) − 1|</td>
          </tr>
          <tr>
            <td class="font-mono">Cosine</td>
            <td class="font-mono">sin(πn / (N−1))</td>
          </tr>
          <tr>
            <td class="font-mono">Nuttall</td>
            <td class="font-mono">a₀ + a₁·cos(2πn/(N−1)) + a₂·cos(4πn/(N−1)) + a₃·cos(6πn/(N−1))<br/>
              a = [0.355768, −0.487396, 0.144232, −0.012604]</td>
          </tr>
          <tr>
            <td class="font-mono">Parzen</td>
            <td class="font-mono">Piecewise cubic: 1−6(n/M)²+6(n/M)³ for n≤M/2, 2(1−n/M)³ for n>M/2</td>
          </tr>
        </tbody>
      </table>
    </div>

    <h3 class="h3 mt-4">Rust implementation</h3>
    <pre class="code-block p-4 overflow-x-auto text-xs"><code>{rustCode}</code></pre>

    <h3 class="h3 mt-4">When to use which window</h3>
    <ul class="list-disc list-inside text-sm text-surface-400 space-y-1">
      <li><strong>Hanning</strong> — Good general-purpose, smooth edges</li>
      <li><strong>Hamming</strong> — Lower first sidelobe than Hanning, good for narrowband</li>
      <li><strong>Blackman</strong> — Very low sidelobes, good for high dynamic range</li>
      <li><strong>Bartlett</strong> — Triangular, simplest window</li>
      <li><strong>Nuttall</strong> — Extreme sidelobe suppression (−98 dB)</li>
      <li><strong>Parzen</strong> — Smooth piecewise cubic, no negative values</li>
    </ul>
  </section>

</div>
