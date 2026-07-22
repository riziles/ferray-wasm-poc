<script lang="ts">
  import { getContext } from 'svelte';
  import type { WasmApi } from '$lib/wasm/loader';
  import CounterexampleLab from '$lib/CounterexampleLab.svelte';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl space-y-8">

  <h1 class="h1">🔬 Counterexample Lab</h1>
  <p class="text-lg text-surface-400">
    Adjust the coefficients of a parameterized polynomial map and watch ferray-autodiff
    compute the Jacobian determinant at 100 random points in real time. When the determinant
    is constant everywhere, you've found a candidate — then search for collision points to
    prove non-invertibility.
  </p>

  <CounterexampleLab {wasm} />

  <section class="card card-demo p-6 space-y-4">
    <h2 class="h2">🎯 How to find a counterexample</h2>

    <div class="space-y-3 text-surface-300">
      <p>
        <strong>Step 1 — Understand the family.</strong>
        Click <strong>Load Alpöge</strong> to see the known counterexample. The 6 parameters
        (a,b,c,d,e,f) control the strength of each polynomial term. The Jacobian determinant
        being constant is a <em>delicate balance</em> — most random settings will produce a
        determinant that varies wildly from point to point.
      </p>
      <p>
        <strong>Step 2 — Search for constancy.</strong>
        Hit <strong>Randomize</strong> and watch the green bar. When σ (standard deviation)
        is tiny, the determinant is nearly the same everywhere. Fine-tune with the sliders
        to drive σ → 0.
      </p>
      <p>
        <strong>Step 3 — Verify non-invertibility.</strong>
        Once the bar is fully green, click <strong>Find Collision</strong>. This runs a
        gradient-descent search to find two distinct points (x₁,y₁,z₁) and (x₂,y₂,z₂)
        that map to the same output. If found, you've disproved invertibility!
      </p>
    </div>

    <h3 class="h3 mt-4">What's happening under the hood</h3>
    <ul class="list-disc list-inside text-sm text-surface-400 space-y-1">
      <li>
        <strong>ferray-autodiff</strong> computes the 3×3 Jacobian via forward-mode
        automatic differentiation — no hand-derived partial derivatives
      </li>
      <li>
        <strong>ferray-linalg::det</strong> computes the determinant of the Jacobian matrix
      </li>
      <li>
        <strong>ferray-random</strong> generates the sampling points for scoring
      </li>
      <li>
        Gradient descent tries to minimize ‖F(x₁) − F(x₂)‖² to find collision pairs
      </li>
    </ul>

    <h3 class="h3 mt-4">The real search (how the AI did it)</h3>
    <div class="bg-surface-800 p-4 rounded text-sm text-surface-400 space-y-2">
      <p>
        The actual counterexample was found using a multi-agent search coordinated by
        Claude Fable 5. The search explored dozens of mathematical approaches simultaneously
        — algebraic geometry, polynomial automorphism theory, differential forms, invariant
        theory — with adversarial agents checking each candidate for errors.
      </p>
      <p>
        The 6-parameter family in this demo captures the essential structure: terms of the
        form (1+xy)ᵏ appeared naturally in the search because they simplify the Jacobian
        computation (the chain rule causes factors to cancel).
      </p>
    </div>
  </section>

</div>
