<script lang="ts">
  import { getContext } from 'svelte';
  import type { WasmApi } from '$lib/wasm/loader';
  import FftAnalyzer from '$lib/FftAnalyzer.svelte';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());

  const rustCode = `use ferray::Array1;
use ferray_fft::{rfft, FftNorm};
use ferray_window::hanning;
use num_complex::Complex;

#[wasm_bindgen]
pub fn fft_magnitude(data: &[f64]) -> Vec<f64> {
    let arr = Array1::from_vec(data.to_vec());
    let window = hanning(data.len());
    let windowed = &arr * &window;
    let spectrum = rfft(&windowed, None, None, FftNorm::Backward)
        .unwrap();
    spectrum.iter()
        .map(|c: &Complex<f64>| c.norm())
        .collect()
}

#[wasm_bindgen]
pub fn composite_signal(
    freqs: &[f64], amps: &[f64], n: usize
) -> Vec<f64> {
    let t = Array1::linspace(0.0, 1.0, n);
    freqs.iter().zip(amps.iter()).fold(
        Array1::zeros(n), |acc, (&f, &a)|
            acc + a * t.mapv(|x| (2.0 * PI * f * x).sin())
    ).into_raw_vec()
}`;
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl space-y-8">

  <h1 class="h1">📈 FFT Spectrum Analyzer</h1>
  <p class="text-lg text-surface-400">
    Generates a composite signal, windows it, runs a real FFT, and plots the magnitude spectrum.
  </p>

  <FftAnalyzer {wasm} />

  <section class="card card-demo p-6 space-y-4">
    <h2 class="h2">How it works</h2>

    <p class="text-surface-300">
      A composite signal is generated from three sinusoids (50 Hz, 120 Hz, 300 Hz) with
      amplitudes (1.0, 0.5, 0.25). A window function is applied to reduce spectral leakage,
      then a real FFT computes the frequency-domain representation.
    </p>

    <div class="bg-surface-800 p-4 rounded text-center font-mono text-sm text-surface-300 space-y-1">
      <div>signal(t) = 1.0·sin(2π·50t) + 0.5·sin(2π·120t) + 0.25·sin(2π·300t)</div>
      <div>X[k] = Σ<sub>n=0</sub><sup>N-1</sup> w[n]·x[n]·e<sup>−2πi·kn/N</sup></div>
    </div>

    <h3 class="h3 mt-4">Rust implementation</h3>
    <pre class="code-block p-4 overflow-x-auto text-xs"><code>{rustCode}</code></pre>

    <h3 class="h3 mt-4">Windowing and spectral leakage</h3>
    <p class="text-surface-400 text-sm">
      Without windowing, the abrupt cut-off at signal boundaries introduces sidebands —
      called spectral leakage. A window function tapers the signal to zero at edges,
      trading some frequency resolution for much cleaner spectra.
    </p>

    <div class="overflow-x-auto mt-4">
      <table class="table table-hover text-xs">
        <thead><tr><th>Window</th><th>Main lobe</th><th>Sidelobe</th><th>Best use</th></tr></thead>
        <tbody>
          <tr><td class="font-mono">Hanning</td><td>4 bins</td><td>−32 dB</td><td>General purpose</td></tr>
          <tr><td class="font-mono">Hamming</td><td>4 bins</td><td>−43 dB</td><td>Narrowband signals</td></tr>
          <tr><td class="font-mono">Blackman</td><td>6 bins</td><td>−58 dB</td><td>High dynamic range</td></tr>
          <tr><td class="font-mono">Nuttall</td><td>8 bins</td><td>−98 dB</td><td>Very low leakage</td></tr>
        </tbody>
      </table>
    </div>
  </section>

</div>
