<script lang="ts">
  import { onMount } from 'svelte';
  import ExponentLedger from '$lib/kakeya/ExponentLedger.svelte';
  import ShellBudget from '$lib/kakeya/ShellBudget.svelte';
  import DirectionFilter from '$lib/kakeya/DirectionFilter.svelte';
  import DeterminantScaling from '$lib/kakeya/DeterminantScaling.svelte';
  import { criticalP } from '$lib/kakeya/exponents';
  import { base } from '$app/paths';

  let scroll = $state(0);
  onMount(() => {
    const onScroll = () => {
      const doc = document.documentElement;
      const max = doc.scrollHeight - window.innerHeight;
      scroll = max > 0 ? doc.scrollTop / max : 0;
    };
    window.addEventListener('scroll', onScroll, { passive: true });
    onScroll();
    return () => window.removeEventListener('scroll', onScroll);
  });

  const contents = [
    { id: 'statement', n: 1, title: 'One estimate at the critical exponent' },
    { id: 'filter', n: 2, title: 'A direction filter made of imaginary time' },
    { id: 'current', n: 3, title: 'A conserved current from two flows' },
    { id: 'energy', n: 4, title: 'The energy is a determinant equation' },
    { id: 'circles', n: 5, title: 'Two circles, and the logarithm' },
    { id: 'tubes', n: 6, title: 'From caps to tubes: Kakeya falls out' },
    { id: 'scope', n: 7, title: 'What is not settled' }
  ];

  // The loss bookkeeping: A = 2/p_c from Theorem 1.2, the extension inherits A,
  // and Lemma 9.4 doubles it, so the Kakeya maximal estimate carries log^(4/p_c).
  const bookkeeping = $derived(
    [2, 3, 4, 5, 6].map((n) => ({
      n,
      pc: criticalP(n),
      A: 2 / criticalP(n),
      kakeya: 4 / criticalP(n),
      sharp: 1 / criticalP(n)
    }))
  );
</script>

<div class="fixed top-0 left-0 right-0 h-0.5 z-20 bg-surface-800">
  <div
    class="h-full bg-gradient-to-r from-secondary-500 via-primary-500 to-warning-500"
    style="width: {scroll * 100}%"
  ></div>
</div>

<div class="container mx-auto px-4 py-8 max-w-4xl space-y-16 pb-24">
  <!-- ───────────────────────── hero ───────────────────────── -->
  <header class="space-y-6">
    <div class="flex flex-wrap items-center gap-2">
      <span class="badge preset-filled-secondary-500">reading notes</span>
      <span class="badge preset-tonal-warning">no WASM required</span>
      <span class="badge preset-tonal">4 live models</span>
      <a href="{base}/kakeya" class="badge preset-tonal">← the Kakeya explainer</a>
    </div>
    <h1 class="h1 leading-tight">
      🔬 Inside the proof
      <span class="block text-surface-400 text-2xl md:text-3xl mt-2">
        how local smoothing was attacked in every dimension at once
      </span>
    </h1>
    <p class="text-lg text-surface-300 max-w-3xl">
      The Kakeya page asks what the problem is. This one is about the paper: OpenAI's
      <em>Local smoothing for the Euclidean wave equation</em> (25 October 2026). It is a strange
      proof, and not only because of who wrote it — it never decomposes the wave into tubes and it
      never runs an induction on scales. Instead it builds a <em>conserved current</em> for a
      superposition of waves and feeds it into a <strong>Monge–Ampère-type equation</strong> on an
      auxiliary space, and the exponent 2n/(n−1) falls out of how that equation scales.
    </p>

    <div class="rounded-lg border border-warning-700/50 bg-warning-950/20 p-5 space-y-3">
      <h2 class="font-bold text-warning-300 flex items-center gap-2"><span>⚠️</span> How to read this page</h2>
      <p class="text-sm text-surface-300">
        Every claim in quotation marks is the preprint's, with the section it comes from. Everything
        else is my summary, marked as such. The four interactive panels are <em>models of
        mechanisms</em> — a direction filter, an exponent ledger, a determinant scaling, a shell
        budget — not re-derivations of the theorem, and the page says at each one what it does and
        does not show.
      </p>
      <p class="text-sm text-surface-400">
        The paper is unrefereed. The Lean formalisation that accompanied it is a real check that the
        argument composes, and it is not a check that the statement is the theorem anyone wanted; the
        formalised version of the sharpest corollary is also visibly weaker than the paper's (see
        section 6).
      </p>
    </div>

    <nav class="rounded-lg border border-surface-700 bg-surface-800/40 p-4">
      <div class="text-xs uppercase tracking-wider text-surface-500 mb-2">Contents</div>
      <ol class="grid sm:grid-cols-2 gap-x-6 gap-y-1 text-sm">
        {#each contents as c}
          <li class="flex items-baseline gap-2 text-surface-400">
            <span class="font-mono text-surface-600 w-4 shrink-0">{c.n}.</span>
            <a href="#{c.id}" class="hover:text-primary-400 transition-colors flex-1">{c.title}</a>
          </li>
        {/each}
      </ol>
    </nav>
  </header>

  <!-- ───────────────────────── 1 ───────────────────────── -->
  <section id="statement" class="space-y-4 scroll-mt-16">
    <h2 class="h2">1. One estimate at the critical exponent</h2>
    <p class="text-surface-300">
      The target is Sogge's inequality: for n ≥ 2, 2 ≤ p &lt; ∞ and every ε &gt; 0,
    </p>
    <div class="rounded-lg border border-surface-700 bg-surface-950/60 p-4 font-mono text-sm overflow-x-auto">
      ‖U(T)f‖<sub>L<sup>p</sup>(ℝ<sup>n</sup>×[1,2])</sub> ≤ C<sub>n,p,ε</sub> ‖J<sup>s₊(p)+ε</sup>f‖<sub>L<sup>p</sup>(ℝ<sup>n</sup>)</sub>,
      &nbsp;&nbsp; s₊(p) = max(0, (n−1)/2 − n/p).
    </div>
    <p class="text-surface-300">
      At a fixed time the sharp Sobolev order is (n−1)(1/2 − 1/p). So once p ≥ p<sub>c</sub> =
      2n/(n−1), averaging over the time interval buys exactly 1/p of a derivative; below p<sub>c</sub>
      it makes the whole order unnecessary. Everything interesting happens at p<sub>c</sub>, where the
      theorem is a statement about a <em>logarithm</em>.
    </p>
    <ExponentLedger />
    <p class="text-surface-300">
      So the paper proves one sharp statement at the critical exponent instead: for
      φ ∈ C<sub>c</sub><sup>∞</sup>(ℝ<sup>n</sup>\{0}) and P<sub>N</sub> = φ(D/N),
    </p>
    <div class="rounded-lg border border-surface-700 bg-surface-950/60 p-4 font-mono text-sm overflow-x-auto">
      ‖U(T)P<sub>N</sub>f‖<sub>L<sup>p<sub>c</sub></sup>(ℝ<sup>n</sup>×[1,2])</sub> ≤ C<sub>n,φ</sub>
      [log(2 + N)]<sup>2/p<sub>c</sub></sup> ‖f‖<sub>L<sup>p<sub>c</sub></sup>(ℝ<sup>n</sup>)</sub>.
      <span class="text-surface-500">(Theorem 1.2)</span>
    </div>
    <p class="text-surface-300">
      Interpolation in p and a sum over frequency annuli give Theorem 1.1 from this. The factor
      log²(2 + N) is the entire quantitative loss, and Section 8 shows it cannot be removed
      entirely: the paper proves a matching lower bound of [log N]<sup>1/p<sub>c</sub></sup> from a
      radial focusing example, and a second weak-type obstruction showing that Sobolev order zero
      fails for every 2 &lt; p ≤ p<sub>c</sub>. The unexplored gap — 1/p<sub>c</sub> versus
      2/p<sub>c</sub> — is stated plainly in the paper and matters downstream, as section 6 shows.
    </p>
    <ShellBudget />
    <p class="text-sm text-surface-400 border-l-2 border-surface-600 pl-4">
      <strong>Why a logarithm and not a power.</strong> The focusing example puts comparable mass at
      every dyadic distance between one wavelength and unit distance from the focus, so the p<sub>c</sub>-th
      power of the spacetime norm receives the same contribution from every scale. Summing a constant
      over log N scales is a logarithm. The panel above is the exact bookkeeping: exponent
      d − dp/2, which is −1 precisely at p<sub>c</sub>.
    </p>
  </section>

  <!-- ───────────────────────── 2 ───────────────────────── -->
  <section id="filter" class="space-y-4 scroll-mt-16">
    <h2 class="h2">2. A direction filter made of imaginary time</h2>
    <p class="text-surface-300">
      Here is the idea the proof is built on, and the paper states it in one sentence: “An imaginary
      translation along a light ray filters the sum by direction.” Translating the wave by
      −iρ(1, ω) multiplies each Fourier mode ξ by
    </p>
    <div class="rounded-lg border border-surface-700 bg-surface-950/60 p-4 font-mono text-sm overflow-x-auto">
      e<sup>ρ(ξ·ω − |ξ|)</sup> = e<sup>−ρ|ξ| |ξ̂ − ω|²/2</sup>,
      <span class="text-surface-500">so for |ξ| ≍ N its angular width is (ρN)<sup>−1/2</sup>.</span>
    </div>
    <p class="text-surface-300">
      This is the standard wave-packet intuition — pick a direction and you pick a tube — but obtained
      by <em>multiplying</em> the sum instead of decomposing it. The paper is explicit about why that
      matters: “The modes are summed before taking the square, so their interference remains in the
      time component of the tensor.” Everything downstream — a focus surviving the estimate — depends
      on that ordering.
    </p>
    <DirectionFilter />
    <p class="text-surface-300">
      In the paper this Gaussian is not a picture but a weight: Section 2.1 derives it from the symbol
      of the wave operator, written as a symmetric first-order system in null coordinates
      t = (T + b)/√2, s₀ = (b − T)/√2, where the wave operator becomes 2∂<sub>t</sub>∂<sub>s₀</sub> +
      Δ<sub>x</sub> and each frequency shell is a free Schrödinger operator in the transverse
      variables. The imaginary translation is what makes a single cap selectable — and the cap is what
      the next section integrates against.
    </p>
  </section>

  <!-- ───────────────────────── 3 ───────────────────────── -->
  <section id="current" class="space-y-4 scroll-mt-16">
    <h2 class="h2">3. A conserved current from two flows</h2>
    <p class="text-surface-300">
      A positive quantity is needed that is conserved along the flow and still ≥ |wave|². The
      construction takes <em>two</em> real states w₁, w₂ of the symmetric system and pairs them into
      one complex state z = w₁ + k w₂, where k² = −1 — a second imaginary unit, so the pair is a
      quaternion-valued field. The flux of the pair,
      q = ½(w₁<sup>t</sup>C<sup>μ</sup>w₁ + w₂<sup>t</sup>C<sup>μ</sup>w₂), is then the imaginary
      displacement of the translation, and the two real squares add up to the Gaussian of the previous
      section:
    </p>
    <div class="rounded-lg border border-surface-700 bg-surface-950/60 p-4 font-mono text-sm overflow-x-auto">
      e<sup>p·q</sup> = exp(−κ/2 |ζ − vR|²)
      <span class="text-surface-500">(2.4) — the symbol determines the Gaussian</span>
    </div>
    <p class="text-surface-300">
      Continue the wave along that imaginary direction and differentiate in the two real state
      variables: the result is a tensor
      B = Σ<sub>ℓ=1,2</sub> Re Z<sub>ℓ</sub> ⊗ Z<sub>ℓ</sub> (2.5) which is symmetric and positive
      semidefinite, and — this is the theorem of Section 2 — <strong>each of its columns has zero
      divergence</strong> in the combined physical and auxiliary variables. A positive quantity with a
      conservation law, whose time component detects the wave on the chosen cap: exactly what a
      pseudo-differential proof never gets.
    </p>
    <p class="text-surface-300">
      Two books are being kept here. The divergence-free property is what lets the energy be
      integrated by parts, and it is inherited by a <em>cofactor</em> of the Hessian — the paper proves
      a polynomial version of a result of Alesker's for the Moore cofactor of a quaternionic Hermitian
      Hessian (Lemma 3.3), and the boundary term of that cofactor vanishes because its normal–normal
      coefficient is zero. The other book is geometric: the current is tangent to the level sets of a
      complex scalar Φ, so the background supplies no contribution to the conservation law while still
      contributing curvature to the determinant. That asymmetry is where the exponent comes from.
    </p>
  </section>

  <!-- ───────────────────────── 4 ───────────────────────── -->
  <section id="energy" class="space-y-4 scroll-mt-16">
    <h2 class="h2">4. The energy is a determinant equation</h2>
    <p class="text-surface-300">
      This is the step with no analogue in the standard toolkit. On an auxiliary “fibre” the regularised
      time component of the current gives a density μ, and the potential u is defined as the solution of
    </p>
    <div class="rounded-lg border border-surface-700 bg-surface-950/60 p-4 font-mono text-sm overflow-x-auto">
      det H<sub>fib</sub>(λ|Φ|² + u) = λμ, &nbsp; u|<sub>∂</sub> = 0,
      <span class="text-surface-500">a quaternionic Monge–Ampère equation with Dirichlet data.</span>
    </div>
    <p class="text-surface-300">
      The paper's own summary of what this buys: “density μ ≥ m on a small ball forces the depth of u at
      its centre to be at least a constant times the squared radius times m<sup>1/(n−1)</sup>.” Then:
      “The integral (−u)μ detects the power m<sup>n/(n−1)</sup>. Since μ recovers a square of the wave,
      the resulting exponent is 2n/(n−1).”
    </p>
    <DeterminantScaling />
    <p class="text-surface-300">
      The reason the depth scales as m<sup>1/(n−1)</sup> rather than the m<sup>1/n</sup> of ordinary
      Monge–Ampère is the background λ|Φ|²: it contributes exactly one direction of curvature, so the
      determinant of the Hessian of (λ|Φ|² + u) is (β + α)α<sup>n−1</sup> with β ≍ λ, and in the range
      where α is small the single β does the work of n. That is the whole content of the interactive
      above, and it is not a small detail: 1/n doubled would give 2n, not 2n/(n−1).
    </p>
    <p class="text-sm text-surface-400 border-l-2 border-surface-600 pl-4">
      <strong>What the panel does not show.</strong> The real equation is quaternionic, on a fibre with
      three real auxiliary variables, with a rank-one background that is not a constant. The panel
      solves the scalar model (β + α)α<sup>n−1</sup> = μ for α and checks the two scaling regimes. That
      is the mechanism; the existence theory for the actual Dirichlet problem (Appendix A) is not
      modelled.
    </p>
  </section>

  <!-- ───────────────────────── 5 ───────────────────────── -->
  <section id="circles" class="space-y-4 scroll-mt-16">
    <h2 class="h2">5. Two circles, and the logarithm</h2>
    <p class="text-surface-300">
      With the energy F(T, τ) constructed on the product of physical time T and two auxiliary time
      coordinates τ, the rest is a scalar argument. Section 5 ends with
    </p>
    <div class="rounded-lg border border-surface-700 bg-surface-950/60 p-4 font-mono text-sm overflow-x-auto">
      Δ<sub>(T,τ)</sub>F ≤ C s<sup>−2</sup>e<sup>−cs/h</sup>F,
      &nbsp;&nbsp; 2a ≤ s = |τ| ≤ 3, &nbsp; h = N<sup>−1</sup>,
    </div>
    <p class="text-surface-300">
      together with two comparisons: at |τ| = 2a the energy dominates the wave's p<sub>c</sub>-th power,
      and at |τ| = 4a it is dominated by the initial data. Then Section 6 walks between them: from
      |τ| = 2a out to |τ| = 1, then a fixed-radius mean comparison back to initial time, then out again
      to |τ| = 4a. Each passage shrinks the radius by a factor, and the paper explains the cost in one
      line: “A radial harmonic function in the plane is affine in the logarithm of its radius. Each
      passage thus loses an inverse logarithm.”
    </p>
    <p class="text-surface-300">
      Two passages, two logarithms — the [log(2 + N)]<sup>2/p<sub>c</sub></sup> of Theorem 1.2. And the
      paper says this is not laziness: Remark 6.2 notes that “both losses can occur for a scalar
      supersolution even with zero potential”, so a scalar comparison argument of this shape cannot do
      better without new structure.
    </p>
    <div class="overflow-x-auto rounded-lg border border-surface-700">
      <table class="w-full text-sm">
        <thead class="bg-surface-800/60 text-left text-xs uppercase tracking-wider text-surface-400">
          <tr>
            <th class="p-2">n</th>
            <th class="p-2">p<sub>c</sub></th>
            <th class="p-2">A = 2/p<sub>c</sub> (Thm 1.2)</th>
            <th class="p-2">Kakeya maximal 4/p<sub>c</sub> (Cor 9.1)</th>
            <th class="p-2">lower bound 1/p<sub>c</sub> (§8)</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-surface-800 font-mono text-xs">
          {#each bookkeeping as b}
            <tr>
              <td class="p-2 text-surface-300">{b.n}</td>
              <td class="p-2 text-surface-200">{b.pc.toFixed(3)}</td>
              <td class="p-2 text-primary-300">{b.A.toFixed(3)}</td>
              <td class="p-2 text-warning-300">{b.kakeya.toFixed(3)}</td>
              <td class="p-2 text-success-300">{b.sharp.toFixed(3)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    <p class="text-xs text-surface-500">
      The logarithmic power travels: Theorem 1.2's A is inherited by the spherical extension estimate
      (Lemma 9.2) and doubled by the tube argument (Lemma 9.4), so the Kakeya maximal estimate carries
      log<sup>4/p<sub>c</sub></sup>. The last column is what sharpness demands.
    </p>
  </section>

  <!-- ───────────────────────── 6 ───────────────────────── -->
  <section id="tubes" class="space-y-4 scroll-mt-16">
    <h2 class="h2">6. From caps to tubes: Kakeya falls out</h2>
    <p class="text-surface-300">
      Sections 9–12 spend the estimate. The route to Kakeya is short and is the part most worth
      knowing. First, modulation in time turns the wave estimate into a statement about spherical
      extension: an incoming spherical wave with direction-dependent amplitude focuses, and near the
      focus the radial oscillation cancels, leaving E g — the extension of g from the unit sphere
      (Lemma 9.2). Then a cap of radius δ produces a wave of “roughly constant modulus on a tube of
      length δ<sup>−2</sup> and radius δ<sup>−1</sup>”, so extension estimates become tube estimates.
    </p>
    <p class="text-surface-300">
      The conversion uses randomisation, and the paper is candid about why: “The restriction argument …
      puts independent signs on disjoint caps. The expected squared modulus of the sum is then the sum
      of the squared moduli. This accounts for the factor of two in the logarithmic exponent.” In
      symbols, Jensen's inequality at each point gives
    </p>
    <div class="rounded-lg border border-surface-700 bg-surface-950/60 p-4 font-mono text-sm overflow-x-auto">
      𝔼|E g<sub>ε</sub>|^p ≥ (Σ<sub>j</sub> a<sub>j</sub> |E h<sub>j</sub>|²)<sup>q</sup>,
      &nbsp; q = n/(n−1),
      <span class="text-surface-500">the right side is an overlap count of tubes.</span>
    </div>
    <p class="text-surface-300">
      The left side is a wave estimate; the right side is exactly the Kakeya maximal function. The
      output (Corollary 9.1) is
    </p>
    <div class="rounded-lg border border-surface-700 bg-surface-950/60 p-4 font-mono text-sm overflow-x-auto">
      ‖K<sub>δ</sub>f‖<sub>L<sup>n</sup>(S<sup>n−1</sup>)</sub> ≤ C<sub>n</sub>
      [log(2/δ)]<sup>4/p<sub>c</sub></sup> ‖f‖<sub>L<sup>n</sup>(ℝ<sup>n</sup>)</sub>,
      &nbsp; hence O<sub>ε</sub>(δ<sup>−ε</sup>), &nbsp; hence dim = n.
    </div>
    <p class="text-surface-300">
      A logarithm is weaker than δ<sup>−ε</sup> in form, so the corollary is stated with the
      logarithmic loss and then weakened to the classical form. Every Euclidean Kakeya set having full
      Hausdorff dimension is stated as an immediate consequence — the step that took Wang and Zahl a long
      paper in three dimensions is now the last line of an argument about waves.
    </p>
    <p class="text-sm text-surface-400 border-l-2 border-surface-600 pl-4">
      <strong>And the Lean version says what it can prove.</strong> The accompanying formalisation
      records the same chain, but its sharpest concrete tube statement is logarithmic rather than
      δ<sup>−ε</sup>: in dimension three, at least δ<sup>−2</sup> unit tubes of radius δ with
      δ-separated directions have union volume at least c/(1 + log(1/δ))<sup>66</sup>. That gap between
      a paper's “hence O<sub>ε</sub>(δ<sup>−ε</sup>)” and a machine-checked constant is the sort of
      thing to look at before believing a press release, in either direction: it is a real
      formalisation, and it is not the theorem as advertised.
    </p>
  </section>

  <!-- ───────────────────────── 7 ───────────────────────── -->
  <section id="scope" class="space-y-4 scroll-mt-16">
    <h2 class="h2">7. What is not settled</h2>
    <ul class="text-surface-300 space-y-3 text-sm list-disc pl-5">
      <li>
        <strong>The logarithm.</strong> The paper gets 2/p<sub>c</sub> where the examples allow
        1/p<sub>c</sub>, and it says so. A 1/p<sub>c</sub> second-moment estimate would give the sharp
        logarithmic power in the Kakeya maximal estimate, and its absence is the one quantitative gap
        the authors themselves flag.
      </li>
      <li>
        <strong>Refereeing.</strong> None of this has been reviewed. The Lean development checks the
        composition of the argument against stated lemmas; it cannot check that the lemmas say what
        everyone thinks they say, nor that the definitions encode Euclidean waves rather than something
        adjacent.
      </li>
      <li>
        <strong>Two dimensions is different.</strong> Appendix D replaces the determinant equation by a
        linear Dirichlet problem on a disk, and the paper notes that the linear problem suffices there.
        If the 2D case can be done without quaternionic Monge–Ampère, a sceptic has a fair question
        about how much of the machinery is load-bearing.
      </li>
      <li>
        <strong>The neighbours.</strong> Two further preprints in the same repository do related work —
        <em>Critical local smoothing for the three-dimensional wave equation</em> (24 September 2026)
        and <em>Every four-dimensional Kakeya set has full Hausdorff dimension</em> (24 September 2026).
        The 4D statement is a corollary here; it is also its own paper. Independent groups reading
        them together is the check that matters.
      </li>
      <li>
        <strong>What an independent check would look at.</strong> The two places where a mistake would
        be hardest to see: the claim that the corrected energy stays comparable to ∫(−u)μ while the
        full determinant cancels (Section 4.3, using an Aubin–Mabuchi-type energy), and the claim that
        the fibre boundary can only contribute where the Gaussian is already smaller than the cost of
        recovering the wave (Section 5, the separation in Figure 1).
      </li>
    </ul>
    <p class="text-surface-400 text-sm">
      None of that is a reason to dismiss the paper. It is a reason to read it as what OpenAI says it
      is: raw research data, unusually well presented, with its own gaps marked.
    </p>
  </section>

  <!-- ───────────────────────── sources ───────────────────────── -->
  <section class="space-y-3">
    <h2 class="h2">Sources</h2>
    <ul class="text-sm space-y-2 text-surface-400">
      <li>
        OpenAI, <a class="text-primary-400 hover:underline" href="https://github.com/openai/math/blob/main/preprints/Local-smoothing-for-the-Euclidean-wave-equation-October-25-2026/paper.pdf" target="_blank" rel="noopener">“Local smoothing for the Euclidean wave equation”</a>, 25 October 2026 — all quotations, section numbers and theorem numbers on this page.
      </li>
      <li>
        OpenAI, <a class="text-primary-400 hover:underline" href="https://github.com/openai/math/blob/main/lean/docs/079.md" target="_blank" rel="noopener">Lean formalisation notes for the local smoothing paper</a> — scope, comparator statements, and the log<sup>66</sup> tube-volume bound quoted in section 6.
      </li>
      <li>
        H. Wang and J. Zahl, <a class="text-primary-400 hover:underline" href="https://arxiv.org/abs/2502.17655" target="_blank" rel="noopener">“Volume estimates for unions of convex sets, and the Kakeya set conjecture in three dimensions”</a>, 2025 — the result the corollary now subsumes.
      </li>
      <li>
        L. Guth, H. Wang and R. Zhang, “A sharp square function estimate for the cone in ℝ³”, <em>Annals of Mathematics</em> 192 (2020) — the two-dimensional case, which this paper generalises.
      </li>
      <li>
        C. Sogge, “Local smoothing of Fourier integral operators and Carleson–Sjölin estimates”, <em>J. Amer. Math. Soc.</em> 6 (1993) — the conjecture's origin and its fixed-time comparison.
      </li>
    </ul>
    <p class="text-xs text-surface-500 pt-2">
      The four panels are plain TypeScript: an exponent ledger, a shell budget, a 300×168 complex plane-wave
      sum with a Gaussian direction filter, and a bisection solve of the scalar determinant equation.
      Nothing here is precomputed from the paper.
    </p>
  </section>
</div>
