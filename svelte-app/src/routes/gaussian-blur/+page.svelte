<script lang="ts">
  import { getContext } from 'svelte';
  import type { WasmApi } from '$lib/wasm/loader';
  import GaussianBlur from '$lib/GaussianBlur.svelte';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());

  const rustCode = `use ferray::Array1;
use ferray_ufunc::exp;

#[wasm_bindgen]
pub fn gaussian_blur(data: &[f64], sigma: f64, kernel_size: usize) -> Vec<f64> {
    let n = data.len();
    let half = (kernel_size as isize) / 2;

    // Build Gaussian kernel
    let ks: Vec<f64> = (-half..=half)
        .map(|i| {
            let x = i as f64;
            (-x * x / (2.0 * sigma * sigma)).exp()
                / (sigma * (2.0 * std::f64::consts::PI).sqrt())
        })
        .collect();

    let kernel = Array1::from_vec(ks);
    let kernel = &kernel / kernel.sum();

    // Convolve (O(n·k))
    let signal = Array1::from_vec(data.to_vec());
    let mut out = Array1::zeros(n);
    for i in 0..n {
        let mut s = 0.0;
        for (j, &k) in kernel.iter().enumerate() {
            let idx = (i + j) as isize - half;
            if idx >= 0 && idx < n as isize {
                s += signal[idx as usize] * k;
            }
        }
        out[i] = s;
    }
    out.into_raw_vec()
}`;
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl space-y-8">

  <h1 class="h1">🔵 Gaussian Blur (1D Convolution)</h1>
  <p class="text-lg text-surface-400">
    Adds Gaussian noise to a signal and recovers it via 1D convolution with a Gaussian kernel.
  </p>

  <GaussianBlur {wasm} />

  <section class="card card-demo p-6 space-y-4">
    <h2 class="h2">What is convolution?</h2>

    <p class="text-surface-300">
      Convolution slides a small <strong>kernel</strong> (a weighted window) over a signal,
      multiplying and summing at each position. Think of it like a moving weighted average:
      each output point is influenced by its neighbors. The wider the kernel, the more
      smoothing.
    </p>

    <p class="text-surface-300">
      The <strong>Gaussian kernel</strong> uses the bell-shaped normal distribution,
      so nearby points matter more than distant ones. The diagram below shows the formula
      and what happens when you adjust Σ (sigma):
    </p>

    <div class="bg-surface-800 p-4 rounded text-center font-mono text-primary-400">
      G(x) = (1 / σ√2π) · e<sup>−x²/(2σ²)</sup>
    </div>

    <p class="text-surface-300">
      Larger σ = wider bell curve = more aggressive blur. Try sigma=0.5 (barely any smoothing)
      vs sigma=10 (nearly flat line). The red line is the original noisy signal, green is the
      smoothed result.
    </p>

    <h3 class="h3 mt-4">Rust implementation</h3>
    <pre class="code-block p-4 overflow-x-auto text-xs"><code>{rustCode}</code></pre>

    <h3 class="h3 mt-4">Performance</h3>
    <p class="text-surface-400 text-sm">
      The naive convolution is O(n·k). For the demo parameters (n=200, k=5—21), this runs
      in 0.5—2 ms. For larger signals, consider FFT-based convolution (O(n log n)).
      <code class="code-block">ferray-opencv</code> provides optimized separable filters.
    </p>
  </section>

</div>
