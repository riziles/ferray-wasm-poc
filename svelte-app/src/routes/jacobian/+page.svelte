<script lang="ts">
  import { getContext } from 'svelte';
  import type { WasmApi } from '$lib/wasm/loader';
  import Jacobian from '$lib/Jacobian.svelte';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());

  const rustCode = `use ferray_autodiff::{jacobian, DualNumber};
use ferray_linalg::det;

// Compute Jacobian via forward-mode automatic differentiation.
// No hand-derived partials — the engine traces gradients
// through the Alpöge map automatically.
#[wasm_bindgen]
pub fn jacobian_autodiff(x: f64, y: f64, z: f64) -> Vec<f64> {
    let (jac, _m) = jacobian(
        |v: &[DualNumber<f64>]| {
            let xy = v[0] * v[1];
            let t1 = DualNumber::constant(1.0) + xy;
            let t2 = t1 * t1;
            let t3 = t1 * t2;
            let t4 = DualNumber::constant(4.0)
                     + DualNumber::constant(3.0) * xy;
            let f1 = v[2] * t3 + v[1] * v[1] * t1 * t4;
            // ... f2, f3 similarly ...
            vec![f1, f2, f3]
        },
        &[x, y, z],
    );
    jac
}

// Determinant via ferray-linalg
pub fn jacobian_det(x: f64, y: f64, z: f64) -> f64 {
    let j = jacobian_autodiff(x, y, z);
    let mat = Array::from_vec(Ix2([3, 3]), j).unwrap();
    ferray_linalg::det(&mat).unwrap()
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
      The Jacobian matrix is computed via <strong>ferray-autodiff</strong> — forward-mode
      automatic differentiation. The same Alpöge map code runs over
      <code class="code-block">DualNumber</code> values, which track derivatives alongside
      regular values. No hand-derived partial derivatives, no finite differences.
      The determinant uses <strong>ferray-linalg::det</strong> on a 3×3 ferray matrix.
    </p>

    <p class="text-surface-300">
      The wireframe grid transformation uses <strong>ferray-core</strong> Array1 with
      <code class="code-block">mapv()</code> and elementwise multiplication — all 625 grid points
      processed in a single WASM call via vectorized ops.
    </p>

    <h3 class="h3 mt-4">Jacobian via ferray-autodiff</h3>
    <pre class="code-block p-4 overflow-x-auto text-xs"><code>{rustCode}</code></pre>

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

  <section class="card card-demo p-6 space-y-4 border-2 border-amber-700/30">
    <h2 class="h2">📖 How does this all work?</h2>

    <h3 class="h3">🔍 Step 1 — What's a Jacobian?</h3>
    <p class="text-surface-300">
      You already know that the <strong>derivative</strong> of f(x) tells you how much the
      output changes when you nudge the input: if f′(3) = 5, then near x=3 the function
      stretches the x-axis by a factor of 5.
    </p>
    <p class="text-surface-300">
      A Jacobian is the same idea, but for functions with <em>multiple inputs and outputs</em>.
      If F(x,y,z) produces three numbers, you need a 3×3
      <span class="term" data-tip="A grid of numbers showing every partial derivative. Row i, column j = how much output i changes when you wiggle input j.">matrix</span>
      of all the
      <span class="term" data-tip="A derivative with respect to one variable while holding the others fixed. Write it as ∂f/∂x.">partial derivatives</span>.
      That matrix <em>is</em> the Jacobian. Its
      <span class="term" data-tip="For a 3×3 matrix, a single number that tells you the volume scaling factor of the transformation. Zero means the transformation squishes space flat.">determinant</span>
      tells you how much the function stretches <em>volume</em> at each point.
    </p>

    <h3 class="h3">🎯 Step 2 — What did the conjecture claim?</h3>
    <p class="text-surface-300">
      The Jacobian <strong>conjecture</strong> said: if a
      <span class="term" data-tip="A function like F(x,y,z) = (x²+y, z³−x) where every output is a polynomial — just multiplication, addition, and powers. No division, no roots, no trig.">polynomial map</span>
      has a Jacobian determinant that's the <em>same nonzero constant everywhere</em>,
      then you can always
      <span class="term" data-tip="If F turns A into B, the inverse F⁻¹ turns B back into A. A function that's not one-to-one (injective) can't have an inverse, because you wouldn't know which original to go back to.">invert</span>
      it — there exists another polynomial map that undoes it perfectly.
    </p>
    <p class="text-surface-300">
      For one variable this is obvious: if f′(x) = 5 everywhere, then f(x) = 5x + c,
      and its inverse is just f<sup>−1</sup>(y) = (y−c)/5.
    </p>
    <p class="text-surface-300">
      For two variables, mathematicians have been stuck since <strong>1884</strong>.
      It's one of Stephen Smale's
      <span class="term" data-tip="In 1998, the Fields Medalist Stephen Smale published a list of 18 unsolved problems he thought would be solved in the 21st century. The Jacobian conjecture was #16.">"Mathematical Problems for the Next Century."</span>
    </p>

    <h3 class="h3">💥 Step 3 — The counterexample (July 19, 2026)</h3>
    <p class="text-surface-300">
      In three variables, the conjecture is now <strong>false</strong>. The Alpöge map has
      Jacobian determinant
      <span class="term" data-tip="det(J_F) = −2 means the transformation flips space (the minus sign) and doubles its volume at every single point. The same number everywhere — just what the conjecture's premise requires!">−2 at every point</span>
      — constant and nonzero! But look:
    </p>

    <div class="bg-green-900/20 border border-green-800/40 p-4 rounded space-y-1">
      <div class="font-mono text-sm">
        <span class="text-blue-400">F(0, 0, −¼)</span> =
        <span class="text-green-400">(−¼, 0, 0)</span>
      </div>
      <div class="font-mono text-sm">
        <span class="text-amber-400">F(1, −1.5, 6.5)</span> =
        <span class="text-green-400">(−¼, 0, 0)</span>
      </div>
      <div class="text-surface-400 text-xs mt-2">
        Two <strong>different</strong> inputs land on the <strong>same</strong> output.
        If someone hands you the output (−¼, 0, 0), you can't know whether the original
        was (0, 0, −¼) or (1, −1.5, 6.5). The function
        <span class="term" data-tip="Injective = one-to-one. Every output comes from exactly one input. If two different inputs give the same output, the function is not injective and cannot have an inverse.">isn't injective</span>,
        so no inverse exists — the conjecture fails for 3+ variables.
      </div>
    </div>

    <p class="text-surface-300 mt-4">
      <strong>Why doesn't the constant Jacobian prevent this?</strong> The constant −2
      only guarantees that the map <em>locally</em> behaves like a flipped, doubled copy
      of space everywhere. But it doesn't prevent global "folds" — like wrapping a sheet
      around itself. The visualizer above shows the fold: drag to rotate and you'll see
      the output grid overlaps itself.
    </p>

    <details class="mt-4">
      <summary class="cursor-pointer text-sm text-surface-500 hover:text-surface-300">
        📐 Show the full polynomial map and Jacobian matrix
      </summary>
      <div class="mt-3 space-y-3">
        <p class="text-sm text-surface-400">The Alpöge map (2026):</p>
        <div class="bg-surface-800 p-3 rounded font-mono text-xs space-y-1">
          <div>F<sub>1</sub>(x, y, z) = z(1 + xy)³ + y²(1 + xy)(4 + 3xy)</div>
          <div>F<sub>2</sub>(x, y, z) = y + 3x(1 + xy)²z + 3xy²(4 + 3xy)</div>
          <div>F<sub>3</sub>(x, y, z) = 2x − 3x²y − x³z</div>
        </div>
        <p class="text-sm text-surface-400">Jacobian matrix J =</p>
        <div class="bg-surface-800 p-3 rounded font-mono text-xs">
          | ∂F₁/∂x &nbsp;∂F₁/∂y &nbsp;∂F₁/∂z |<br/>
          | ∂F₂/∂x &nbsp;∂F₂/∂y &nbsp;∂F₂/∂z |<br/>
          | ∂F₃/∂x &nbsp;∂F₃/∂y &nbsp;∂F₃/∂z |
        </div>
        <p class="text-sm text-surface-400">det(J) = −2 (verified at every sample point in the demo)</p>
      </div>
    </details>
  </section>

</div>
