<script lang="ts">
  import { getContext } from 'svelte';
  import type { WasmApi } from '$lib/wasm/loader';
  import Jacobian from '$lib/Jacobian.svelte';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());

  const rustCode = `fn alpoge_map(x: f64, y: f64, z: f64) -> [f64; 3] {
    let xy = x * y;
    let t1 = 1.0 + xy;
    let t2 = t1 * t1;
    let t3 = t1 * t2;
    let t4 = 4.0 + 3.0 * xy;

    let f1 = z * t3 + y * y * t1 * t4;
    let f2 = y + 3.0 * x * t2 * z + 3.0 * x * y * y * t4;
    let f3 = 2.0 * x - 3.0 * x * x * y - x * x * x * z;
    [f1, f2, f3]
}`;

  const batchCode = `// Batch evaluation using ferray vectorized ops
fn jacobian_eval_batch(flat: Vec<f64>) -> Result<Vec<f64>> {
    let n = flat.len() / 3;
    let x = Array::from_vec(Ix1(n), x_col)?;
    let y = Array::from_vec(Ix1(n), y_col)?;
    let z = Array::from_vec(Ix1(n), z_col)?;

    let xy  = (&x * &y).unwrap();         // x_i · y_i
    let t1  = xy.mapv(|v| 1.0 + v);      // 1 + xy
    let t2  = t1.mapv(|v| v * v);        // (1+xy)^2
    let t3  = (&t1 * &t2).unwrap();      // (1+xy)^3
    let t4  = xy.mapv(|v| 4.0 + 3.0*v);  // 4 + 3xy

    let f1 = (&(&z * &t3).unwrap()
        + &(&(&y*&y).unwrap() * &t1).unwrap()
            * &t4).unwrap();
    // ... f2, f3 similarly ...
    Ok(interleave(f1, f2, f3))
}`;
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl space-y-8">

  <h1 class="h1">🔥 Jacobian Conjecture Counterexample</h1>
  <p class="text-lg text-surface-400">
    An AI-discovered counterexample to an 87-year-old conjecture in algebraic geometry,
    verified at 40,000 evaluations/second via WASM.
  </p>

  <Jacobian {wasm} />

  <section class="card card-demo p-6 space-y-4">
    <h2 class="h2">🧠 What is the Jacobian conjecture?</h2>

    <p class="text-surface-300">
      Consider a polynomial map F that takes N variables and outputs N values. If the
      <strong>Jacobian determinant</strong> of F — the determinant of the N×N matrix of
      partial derivatives — is a <em>nonzero constant</em> (the same number at every point),
      then the conjecture claimed F must have a polynomial inverse.
    </p>

    <p class="text-surface-300">
      For N=1: this is trivial (if f'(x)=c≠0, then f is linear and invertible).<br/>
      For N=2: still an <strong>unsolved problem</strong> as of 2026.<br/>
      For N≥3: <strong>disproved</strong> on July 19, 2026 by Levent Alpöge using Claude Fable 5.
    </p>
  </section>

  <section class="card card-demo p-6 space-y-4">
    <h2 class="h2">📐 The counterexample</h2>

    <p class="text-surface-300">
      The map F: ℂ³ → ℂ³ defined as:
    </p>

    <div class="bg-surface-800 p-4 rounded font-mono text-xs space-y-1 overflow-x-auto">
      <div>F<sub>1</sub>(x, y, z) = z(1 + xy)³ + y²(1 + xy)(4 + 3xy)</div>
      <div>F<sub>2</sub>(x, y, z) = y + 3x(1 + xy)²z + 3xy²(4 + 3xy)</div>
      <div>F<sub>3</sub>(x, y, z) = 2x − 3x²y − x³z</div>
    </div>

    <p class="text-surface-300 mt-4">
      has Jacobian determinant <strong>det(J<sub>F</sub>) = −2</strong> at every point —
      constant and non-zero, satisfying the conjecture's premise. But:
    </p>

    <div class="bg-red-900/20 border border-red-800/50 p-4 rounded space-y-1">
      <div class="font-mono text-sm">
        <span class="text-blue-400">F(0, 0, −¼)</span> =
        <span class="text-green-400">(−¼, 0, 0)</span>
      </div>
      <div class="font-mono text-sm">
        <span class="text-amber-400">F(1, −1.5, 6.5)</span> =
        <span class="text-green-400">(−¼, 0, 0)</span>
      </div>
      <div class="text-surface-400 text-xs mt-2">
        Two different inputs → same output. F is not injective, so no inverse exists.
      </div>
    </div>
  </section>

  <section class="card card-demo p-6 space-y-4">
    <h2 class="h2">⚙️ How it works</h2>

    <p class="text-surface-300">
      The 3D visualizer above renders a wireframe slice of space on the left, and its
      image under F on the right. Drag to rotate, scroll to zoom. The "fold" in the
      output is the geometric reason the inverse fails — F squashes two different points
      onto the same place, like folding a sheet of paper.
    </p>

    <p class="text-surface-300">
      The Jacobian matrix is computed via central finite differences (h = 10<sup>−6</sup>),
      then the 3×3 determinant formula is applied. The result rounds to −2.0000 at every
      sampled point — verified by batch-evaluating 1000 random points through WASM.
    </p>

    <h3 class="h3 mt-4">The map (Rust)</h3>
    <pre class="code-block p-4 overflow-x-auto text-xs"><code>{rustCode}</code></pre>

    <h3 class="h3 mt-4">Batch evaluation with ferray</h3>
    <pre class="code-block p-4 overflow-x-auto text-xs"><code>{batchCode}</code></pre>

    <h3 class="h3 mt-4">Historical context</h3>
    <div class="text-sm text-surface-400 space-y-2">
      <p>
        <strong>1884:</strong> Ludwig Kraus first states the conjecture for N=2 variables.
        <strong>1939:</strong> Ott-Heinrich Keller restates it for integer-coefficient
        polynomials. <strong>1998:</strong> Stephen Smale lists it as problem #16 in his
        "Mathematical Problems for the Next Century." <strong>2026:</strong> Levent Alpöge
        presents the counterexample, discovered using Anthropic's Claude Fable 5 AI model.
        The result extends to all N≥3 (and trivially to N≥4).
      </p>
    </div>
  </section>

</div>
