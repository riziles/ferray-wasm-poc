<script lang="ts">
  import { getContext } from 'svelte';
  import type { WasmApi } from '$lib/wasm/loader';
  import Kan from '$lib/Kan.svelte';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());

  const formula = 'x_j^(l+1) = Σ_i φ_{i,j}^{(l)}(x_i^(l))';

  const rustCode = `// One edge: φ(x) = base·silu(x) + Σ cᵢ·Bᵢ(x)
// Bᵢ = B-spline basis (Cox–de Boor), G+k+1 coefficients per edge

fn edge_phi(params: &[f64], e: &EdgeInfo, grid: &[f64], k: usize, x: f64)
    -> (f64, f64, Vec<f64>, f64) {
    let base = params[e.off];
    let coeffs = &params[e.off + 1 .. e.off + 1 + e.ncoeff];
    let (vals, ders) = spline_basis(x, grid, k);       // Bᵢ(x) + dBᵢ/dx

    let mut s = 0.0; let mut ds = 0.0;
    for (i, &c) in coeffs.iter().enumerate() {
        s  += c * vals[i];
        ds += c * ders[i];
    }
    if x < grid[0]        { s += (x - grid[0]) * ds; } // linear extrapolation
    if x > *grid.last().unwrap() { s += (x - *grid.last().unwrap()) * ds; }

    let sig = 1.0 / (1.0 + (-x).exp());
    let silu = x * sig;                                // SiLU residual
    let dsilu = sig * (1.0 + x * (1.0 - sig));
    (base * silu + s, base * dsilu + ds, vals, silu)
}

// Backprop through one edge (accumulated over the batch):
//   ∂L/∂base   = err · silu(x)
//   ∂L/∂cᵢ     = err · Bᵢ(x)
//   err→input  = err · dφ/dx
// Full-batch Adam steps the flat parameter vector.`;
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl space-y-8">

  <h1 class="h1">🧠 Kolmogorov–Arnold Network</h1>
  <p class="text-lg text-surface-400">
    A KAN learns a 2D function live in your browser — every edge is a B-spline, not a weight.
    Watch the splines bend into shape in real time.
  </p>

  <Kan {wasm} />

  <section class="card card-demo p-6 space-y-4">

    <h2 class="h2">What is a KAN?</h2>

    <p class="text-surface-300">
      A <a href="https://arxiv.org/abs/2404.19756" target="_blank" class="text-primary-400 underline">Kolmogorov–Arnold Network</a>
      (Liu et al., 2024) is a neural network based on the
      <a href="https://en.wikipedia.org/wiki/Kolmogorov%E2%80%93Arnold_representation_theorem" target="_blank" class="text-primary-400 underline">Kolmogorov–Arnold representation theorem</a>:
      any multivariate continuous function can be written as a finite composition of univariate
      functions and addition. So instead of stacking linear layers <code class="code-block">W·x + b</code>
      followed by activations (like an MLP), a KAN puts a <strong>learnable univariate function
      φ(x)</strong> on every edge and sums them at each node:
    </p>

    <pre class="code-block p-4 overflow-x-auto text-xs"><code>{formula}</code></pre>

    <p class="text-surface-300">
      Here each φ is a <strong>B-spline</strong> — a piecewise polynomial controlled by a grid of
      <em>G</em> intervals and <em>k</em>th-order basis functions — plus a SiLU residual
      (φ = silu(x) + spline(x), as in the original paper). The non-linearity comes from the splines
      themselves, so no separate activation function is needed. The name comes from the
      Kolmogorov–Arnold theorem, which guarantees a 2-layer network with univariate inner functions
      can represent <em>any</em> continuous function — KANs make that constructive.
    </p>

    <h3 class="h3 mt-4">Why they're interesting</h3>
    <ul class="list-disc list-inside text-surface-300 space-y-1">
      <li>
        <strong>Interpretability:</strong> a trained KAN is literally a stack of 1D curves. The
        spline panel above <em>is</em> the model — you can read off what each input contributes
        (compare with the black-box weights of an MLP).
      </li>
      <li>
        <strong>Accuracy on smooth/low-dim functions:</strong> splines are excellent local
        approximators; small KANs fit functions like <code class="code-block">exp(sin(πx) + y²)</code>
        with far fewer parameters than MLPs.
      </li>
      <li>
        <strong>Grid refinement:</strong> after training you can <em>refine</em> the spline grid to
        increase resolution without retraining (not demoed here — hit Reset instead).
      </li>
    </ul>

    <h3 class="h3 mt-4">What you're watching</h3>
    <p class="text-surface-300">
      A 2→<em>hidden</em>→1 KAN (150 parameters by default: 15 edges × (1 base + 9 spline
      coefficients)) trains via <strong>full-batch Adam</strong> on a grid of samples from a target
      function. Targets are min–max normalized to [0,1]. Each animation frame runs a few epochs in
      WASM: the left heatmap is ground truth, the right is what the network currently thinks the
      function looks like, the log-scale curve is training RMSE, and the bottom panel shows the
      learned edge splines morphing into shape. Try the <strong>product x·y</strong> target —
      learning multiplication requires the splines to coordinate in a way MLPs famously struggle with.
    </p>

    <h3 class="h3 mt-4">Implementation notes</h3>
    <ul class="list-disc list-inside text-surface-300 space-y-1">
      <li>
        Backprop is hand-written (Cox–de Boor gives φ and dφ/dx in one pass). Forward-mode autodiff
        would cost O(parameters) per sample — 150× slower here.
      </li>
      <li>
        Splines use linear extrapolation beyond the grid (±1), so hidden-unit activations outside
        [−1,1] stay well-behaved instead of saturating.
      </li>
      <li>
        Pure scalar Rust — no ferray crates needed for this one; the network fits in a few hundred
        lines and runs per-frame in under ~10 ms.
      </li>
    </ul>

    <h3 class="h3 mt-4">Rust implementation (core)</h3>
    <pre class="code-block p-4 overflow-x-auto text-xs"><code>{rustCode}</code></pre>

    <h3 class="h3 mt-4">Further reading</h3>
    <ul class="list-disc list-inside text-surface-400 text-sm space-y-1">
      <li>KAN paper: <a href="https://arxiv.org/abs/2404.19756" target="_blank" class="text-primary-400 underline">arXiv:2404.19756</a></li>
      <li>Official PyTorch implementation: <a href="https://github.com/KindXiaoming/pykan" target="_blank" class="text-primary-400 underline">KindXiaoming/pykan</a></li>
      <li>Kolmogorov–Arnold theorem: <a href="https://en.wikipedia.org/wiki/Kolmogorov%E2%80%93Arnold_representation_theorem" target="_blank" class="text-primary-400 underline">Wikipedia</a></li>
    </ul>
  </section>

</div>
