<script lang="ts">
  import { getContext } from 'svelte';
  import type { WasmApi } from '$lib/wasm/loader';
  import SumOfSquares from '$lib/SumOfSquares.svelte';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());

  const rustCode = `use ferray::Array1;

#[wasm_bindgen]
pub fn sum_of_squares(input: &[f64]) -> f64 {
    let arr = Array1::from_vec(input.to_vec());
    arr.mapv(|x| x.powi(2)).sum()
}`;
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl space-y-8">

  <h1 class="h1">📐 Sum of Squares</h1>
  <p class="text-lg text-surface-400">
    The simplest possible WASM call: send an array to Rust, compute Σ x², return a scalar.
    Zero-copy via shared linear memory between JS and WASM.
  </p>

  <SumOfSquares {wasm} />

  <section class="card card-demo p-6 space-y-4">
    <h2 class="h2">How it works</h2>

    <p class="text-surface-300">
      The browser allocates a <code class="code-block">Float64Array</code> backed by the WASM linear
      memory heap. When passed to the Rust function via <code class="code-block">wasm-bindgen</code>,
      no copy occurs — Rust receives a slice view into the same memory.
    </p>

    <p class="text-surface-300">
      Internally, the Rust function creates a <code class="code-block">ferray::Array1</code>, calls
      <code class="code-block">.mapv(|x| x.powi(2)).sum()</code>, and returns the result. The
      <code class="code-block">mapv</code> operation benefits from auto-vectorization (SIMD) when
      the LLVM optimizer detects contiguous data.
    </p>

    <h3 class="h3 mt-4">Rust implementation</h3>
    <pre class="code-block p-4 overflow-x-auto text-xs"><code>{rustCode}</code></pre>

    <h3 class="h3 mt-4">Performance note</h3>
    <p class="text-surface-400 text-sm">
      For small arrays (&lt; 1000 elements), the WASM call overhead dominates — this is about
      0.1—0.5 ms. For large arrays, SIMD auto-vectorization in the Rust compiler provides
      a ~2—4× speedup over equivalent JS loops.
    </p>
  </section>

</div>
