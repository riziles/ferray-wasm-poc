<script lang="ts">
  import { onMount } from 'svelte';
  import NeedlePuzzle from '$lib/kakeya/NeedlePuzzle.svelte';
  import PerronDemo from '$lib/kakeya/PerronDemo.svelte';
  import DimensionDemo from '$lib/kakeya/DimensionDemo.svelte';
  import TubeDemo from '$lib/kakeya/TubeDemo.svelte';
  import FocusingDemo from '$lib/kakeya/FocusingDemo.svelte';

  let scroll = $state(0);

  function onScroll() {
    const doc = document.documentElement;
    const max = doc.scrollHeight - window.innerHeight;
    scroll = max > 0 ? doc.scrollTop / max : 0;
  }

  onMount(() => {
    window.addEventListener('scroll', onScroll, { passive: true });
    onScroll();
    return () => window.removeEventListener('scroll', onScroll);
  });

  const contents = [
    { id: 'room', n: 1, title: 'The room problem (1917)', tag: 'live' },
    { id: 'no-minimum', n: 2, title: 'The room can have no area', tag: 'live' },
    { id: 'dimension', n: 3, title: 'Stop asking about area', tag: 'live' },
    { id: 'tubes', n: 4, title: 'Tubes, and harmonic analysis', tag: 'live' },
    { id: 'smoothing', n: 5, title: 'Local smoothing (1991 → 2026)', tag: 'live' },
    { id: 'announcement', n: 6, title: 'What OpenAI announced', tag: 'paper' },
    { id: 'ledger', n: 7, title: 'The status ledger', tag: 'paper' }
  ];

  const strategies = [
    {
      field: 'Mathematics',
      what: 'local smoothing (⇒ Kakeya) in every dimension',
      how: 'release the AI output; researchers organise their own explanation',
      detail: 'Natural-language writeup and Lean formalisation published in the openai/math repository.'
    },
    {
      field: 'Computer science',
      what: 'the Exponential Time Hypothesis is false',
      how: 'an external group explains it, with the output public immediately',
      detail: 'Shepherded by Russell Impagliazzo, Ramamohan Paturi and Ryan Williams.'
    },
    {
      field: 'Physics',
      what: 'existence of QCD and a Standard-Model completion in 4D',
      how: 'an external group is formed and decides for itself when to release',
      detail: 'Led by David Gross, with Michael Douglas, Arthur Jaffe, Igor Klebanov and Nikita Nekrasov.'
    }
  ];

  const ledger = [
    ['Besicovitch sets of measure zero exist in the plane', 'theorem', 'Besicovitch, 1919'],
    ['A needle can be turned in arbitrarily small area', 'theorem', 'Besicovitch, 1928'],
    ['No needle-turning region has measure zero', 'theorem', 'classical'],
    ['Kakeya set conjecture, n = 2', 'theorem', 'Davies, 1971'],
    ['Kakeya set conjecture, n = 3', 'theorem', 'Wang & Zahl, Feb 2025'],
    ['Kakeya maximal (tube) bounds, n = 2', 'theorem', 'classical, 1970s'],
    ['Local smoothing, 2 spatial dimensions', 'theorem', 'Guth, Wang & Zhang, 2019'],
    ['Local smoothing, all dimensions', 'claimed', 'OpenAI, Oct 2026 — unreviewed'],
    ['Kakeya set & maximal, n = 4 and all n', 'claimed corollary', 'OpenAI, Oct 2026'],
    ["Stein's restriction conjecture, all n", 'claimed corollary', 'OpenAI, Oct 2026']
  ] as [string, string, string][];
</script>

<svelte:window on:scroll={onScroll} />

<div class="fixed top-0 left-0 right-0 h-0.5 z-20 bg-surface-800">
  <div
    class="h-full bg-gradient-to-r from-primary-500 via-secondary-500 to-warning-500"
    style="width: {scroll * 100}%"
  ></div>
</div>

<div class="container mx-auto px-4 py-8 max-w-4xl space-y-16 pb-24">
  <!-- ───────────────────────── hero ───────────────────────── -->
  <header class="space-y-6">
    <div class="flex flex-wrap items-center gap-2">
      <span class="badge preset-filled-primary-500">interactive explainer</span>
      <span class="badge preset-tonal-warning">no WASM required</span>
      <span class="badge preset-tonal">pure TypeScript</span>
      <span class="badge preset-tonal">5 live experiments</span>
    </div>
    <h1 class="h1 leading-tight">
      🪡 The Kakeya Conjecture
      <span class="block text-surface-400 text-2xl md:text-3xl mt-2">
        a needle, a room of zero area, and a proof OpenAI says its models found
      </span>
    </h1>
    <p class="text-lg text-surface-300 max-w-3xl">
      In 1917 Sōichi Kakeya asked how small a room could be and still let a unit needle turn all the
      way around. The answer turned out to be “no minimum at all” — and the question that replaced
      it, about the <em>dimension</em> of such sets, became one of the deepest problems in analysis.
      In late October 2026 OpenAI announced that its models had proved Sogge's local smoothing
      conjecture in every dimension, which would settle Kakeya everywhere. This page is an attempt to
      make all of that tangible: five experiments you can turn, slide and count, and then an honest
      account of what has actually been claimed.
    </p>

    <div class="rounded-lg border border-warning-700/50 bg-warning-950/20 p-5 space-y-3">
      <h2 class="font-bold text-warning-300 flex items-center gap-2">
        <span>⚠️</span> Two things this page insists on
      </h2>
      <p class="text-sm text-surface-300">
        <strong>First:</strong> every number below is <em>measured</em>. When you count boxes or area
        on this page, the count comes from a raster of the actual figure at a stated resolution, not
        from a formula quoted at you. Where the measurement falls short of the theorem — and for the
        Perron tree it does, visibly — you will see the shortfall rather than a clean curve that
        pretends otherwise.
      </p>
      <p class="text-sm text-surface-400">
        <strong>Second:</strong> the 2026 results are <em>claims</em>. OpenAI describes its
        manuscripts as “a form of raw research data”, the proofs are machine-checked in Lean but not
        refereed, and the announcement itself is a description of a process the field is still
        arguing about. That is the interesting part, so it gets its own section rather than a
        footnote.
      </p>
    </div>

    <nav class="rounded-lg border border-surface-700 bg-surface-800/40 p-4">
      <div class="text-xs uppercase tracking-wider text-surface-500 mb-2">Contents</div>
      <ol class="grid sm:grid-cols-2 gap-x-6 gap-y-1 text-sm">
        {#each contents as c}
          <li class="flex items-baseline gap-2 text-surface-400">
            <span class="font-mono text-surface-600 w-4 shrink-0">{c.n}.</span>
            <a href="#{c.id}" class="hover:text-primary-400 transition-colors flex-1">{c.title}</a>
            <span class="font-mono text-[10px] text-surface-600">{c.tag === 'live' ? 'demo' : 'text'}</span>
          </li>
        {/each}
      </ol>
    </nav>
  </header>

  <!-- ───────────────────────── 1 ───────────────────────── -->
  <section id="room" class="space-y-4 scroll-mt-16">
    <h2 class="h2">1. The room problem</h2>
    <p class="text-surface-300">
      A <strong>unit needle</strong> is a line segment of length 1. Kakeya's question: what is the
      smallest area of a room in which it can be turned through a full 360°, staying inside the
      room the whole time? Drag the needle, turn the dial, and watch the ring fill up as you find a
      legal placement for each direction. The <em>magnet</em> does the searching for you: it places
      the needle as deep inside the shape as it can for whatever angle you ask for.
    </p>
    <NeedlePuzzle />
    <div class="text-surface-300 space-y-3 text-sm">
      <p>
        Two shapes work and two fail, and the failure is the more useful lesson. The
        1 × ½ rectangle has <em>less</em> area than the equilateral triangle of height 1 — and a unit
        needle can never point across its short axis, because that width is only ½. So the question
        was never really about area: it is about whether a needle-length chord exists in every
        direction. In 1921 Pál showed that among <em>convex</em> rooms the equilateral triangle of
        height 1 is optimal, with area 1/√3 ≈ 0.577. Every width of that triangle is at least 1,
        which is exactly the condition that makes every direction available.
      </p>
      <p class="text-surface-400">
        One caveat, stated plainly: the magnet finds <em>a</em> placement for each direction, not a
        continuous motion. Turning the needle inside the triangle is Pál's separate theorem; the
        placements found here are the endpoint of that story, not the path.
      </p>
    </div>
  </section>

  <!-- ───────────────────────── 2 ───────────────────────── -->
  <section id="no-minimum" class="space-y-4 scroll-mt-16">
    <h2 class="h2">2. The room can have no area at all</h2>
    <p class="text-surface-300">
      In 1919 Abram Besicovitch was working on a problem about integrals and needed a set containing
      a unit segment in every direction but of arbitrarily small area. It should be impossible; it
      is not. Perron's version of the construction is a tree: cut a triangle of height 1 into pieces
      whose apexes all stay on one line, then slide neighbouring pieces over each other. Each piece
      still holds a unit needle for its own direction, and the pieces overlap more and more.
    </p>
    <PerronDemo />
    <div class="text-surface-300 space-y-3 text-sm">
      <p>
        The eight coloured copies are eight rotations by 45°: one copy of the tree holds needles for a
        45° fan of directions, and eight copies tile the whole circle. The white needle in the picture
        is one leaf's own needle, drawn from that leaf's apex — it is inside the leaf because the leaf
        is a translate of a slice of the original triangle, and a unit segment straight down from an
        apex reaches exactly the base.
      </p>
      <p class="text-surface-400">
        Now the honest part, which the chart on the right shows: the area does not fall fast. The
        tree keeps <span class="font-mono">(1 − α) + α<sup>2n</sup></span> of the original triangle,
        and optimising α for a given depth n leaves roughly
        <span class="font-mono">log n / n</span>. Getting down to 1% takes about 381 levels —
        2<sup>381</sup> pieces per copy, roughly 10<sup>115</sup>. Besicovitch's theorem is a limit,
        not a picture, and the price of small area is exponentially many pieces. That is also why the
        object is pathological: it is not a shape anyone can draw.
      </p>
    </div>
  </section>

  <!-- ───────────────────────── 3 ───────────────────────── -->
  <section id="dimension" class="space-y-4 scroll-mt-16">
    <h2 class="h2">3. So stop asking about area</h2>
    <p class="text-surface-300">
      Besicovitch answered Kakeya's question and made it useless: if the area can be anything, then
      area is not the measurement to make. The <strong>Kakeya conjecture</strong> replaces it with a
      dimension. Cover the set with boxes of side δ, count how many are needed, and the growth rate
      N(δ) ≈ δ<sup>−d</sup> defines d. A set of zero area can still consume boxes as eagerly as a
      solid region; the conjecture says that in n dimensions a Kakeya set always does, so d = n.
    </p>
    <DimensionDemo />
    <div class="text-surface-300 space-y-3 text-sm">
      <p>
        The fan measures about 1.7 at this resolution and the disk about 1.9; both are heading for 2,
        and the fan gets there more slowly because its structure lives at every scale. A single
        needle comes out at 1. That gap — between a set that fills space and a set that only points
        in every direction — is the entire subject.
      </p>
      <p class="text-surface-400">
        In two dimensions the conjecture is a theorem (Davies, 1971). In three it was settled by Hong
        Wang and Joshua Zahl in February 2025, after a century of partial results — the best previous
        lower bound in three dimensions was 5/2 + ε. In four and beyond it was open until October
        2026, which is where this page's story turns.
      </p>
    </div>
  </section>

  <!-- ───────────────────────── 4 ───────────────────────── -->
  <section id="tubes" class="space-y-4 scroll-mt-16">
    <h2 class="h2">4. Tubes, and why harmonic analysis cares</h2>
    <p class="text-surface-300">
      Here is the form the problem takes in analysis. Instead of one set, take a family of thin
      tubes — length 1, width δ, one per direction, directions spaced δ apart. Their total area is
      fixed at N·δ ≈ 2π. Push them all through a single point and you get a bush whose union is the
      unit disk: π/4, exactly eight times smaller. Rearrange them however you like and the compression
      stays bounded by a constant — the bush, at 8×, is the worst case anyone has found. That
      boundedness, for every family of tubes and every dimension, is the
      <strong>Kakeya maximal conjecture</strong>, and it implies the set problem above. In the plane
      it has been a theorem since the 1970s; in three and more dimensions it was open until the
      claims of 2025–26.
    </p>
    <TubeDemo />
    <p class="text-surface-300 text-sm">
      The maximal function is the object that shows up when you try to add waves together. If a
      function is large along many tubes, the maximal function measures how large, and the Kakeya
      bound is exactly what stops that largeness from piling up in one place. Which brings us to the
      conjecture at the top of the chain.
    </p>
  </section>

  <!-- ───────────────────────── 5 ───────────────────────── -->
  <section id="smoothing" class="space-y-4 scroll-mt-16">
    <h2 class="h2">5. Local smoothing</h2>
    <p class="text-surface-300">
      In 1991 Christopher Sogge asked how much of a wave's initial data you need in order to control
      its size <em>averaged over a short interval of time</em>. At any single instant a wave can
      focus: a ring converging on a point has a huge L<sup>p</sup> norm at exactly that moment. But
      averaging over time should cost nothing — the time-averaged norm should be bounded by the data
      measured in a Sobolev space with only ε more derivatives. That is the
      <strong>local smoothing conjecture</strong>.
    </p>
    <FocusingDemo />
    <div class="text-surface-300 space-y-3 text-sm">
      <p>
        Sogge's conjecture is the strongest member of a family, and the implications run downhill
        from it:
      </p>
      <div class="rounded-lg border border-surface-700 bg-surface-950/40 p-4 font-mono text-xs">
        <div class="flex flex-wrap items-center gap-2">
          <span class="badge preset-filled-primary-500">local smoothing</span>
          <span class="text-surface-500">⇒</span>
          <span class="badge preset-tonal">Bochner–Riesz</span>
          <span class="text-surface-500">⇒</span>
          <span class="badge preset-tonal">restriction (Stein)</span>
          <span class="text-surface-500">⇒</span>
          <span class="badge preset-tonal">Kakeya</span>
        </div>
        <p class="text-surface-500 mt-2 font-sans text-[11px]">
          Each arrow is a proof that has been known for decades. Proving the strongest statement
          settles every name to its right — which is why one harmonic-analysis theorem would resolve
          the needle problem in every dimension at once.
        </p>
      </div>
      <p>
        In two spatial dimensions the conjecture was proved by Larry Guth, Hong Wang and Ruixiang
        Zhang in 2019 (published in the <em>Annals</em> in 2020), by way of a sharp square-function
        estimate for the light cone. In every higher dimension it stayed open — until the claim
        below.
      </p>
    </div>
  </section>

  <!-- ───────────────────────── 6 ───────────────────────── -->
  <section id="announcement" class="space-y-4 scroll-mt-16">
    <h2 class="h2">6. What OpenAI announced</h2>
    <p class="text-surface-300">
      On 18 October 2026, OpenAI says, its models produced a natural-language proof of Sogge's local
      smoothing conjecture in every spatial dimension n ≥ 2, based on “a novel auxiliary partial
      differential equation”. The proof was formalised in Lean the next day, and on 25 October a
      writeup and the Lean proof were published in the <code class="code-inline">openai/math</code>
      repository. The paper's own abstract claims the consequences: the Bochner–Riesz conjecture,
      sharp bounds for Stein's square function, Fourier restriction for the sphere, the paraboloid
      and the cone throughout the conjectured range, plus the Kakeya and Nikodym maximal conjectures
      — “in particular, every Euclidean Kakeya set has full Hausdorff dimension”.
    </p>

    <div class="rounded-lg border border-surface-700 bg-surface-800/30 p-4">
      <h3 class="font-bold text-surface-200 mb-2 text-sm">Timeline, as stated by OpenAI</h3>
      <ul class="text-sm space-y-1 text-surface-400">
        <li><span class="font-mono text-surface-300">6 Oct 2026</span> — prior release of AI-produced mathematics, on which the process here builds.</li>
        <li><span class="font-mono text-surface-300">18 Oct 2026</span> — model output: local smoothing in every dimension.</li>
        <li><span class="font-mono text-surface-300">19 Oct 2026</span> — formalised in Lean.</li>
        <li><span class="font-mono text-surface-300">25 Oct 2026</span> — writeup and Lean proof published; announcement of the three collaborations.</li>
      </ul>
    </div>

    <p class="text-surface-300">
      The announcement is as much about <em>process</em> as about mathematics. OpenAI frames the
      manuscripts as “a form of raw research data”, and says the release strategy was decided with
      external experts acting as shepherds — differently for each of the three results:
    </p>

    <div class="overflow-x-auto rounded-lg border border-surface-700">
      <table class="w-full text-sm">
        <thead class="bg-surface-800/60 text-left text-xs uppercase tracking-wider text-surface-400">
          <tr>
            <th class="p-3">Field</th>
            <th class="p-3">Result</th>
            <th class="p-3">Release strategy</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-surface-800">
          {#each strategies as s}
            <tr class="align-top">
              <td class="p-3 font-semibold text-surface-200 whitespace-nowrap">{s.field}</td>
              <td class="p-3 text-surface-300">
                {s.what}
                <div class="text-xs text-surface-500 mt-1">{s.detail}</div>
              </td>
              <td class="p-3 text-surface-400">{s.how}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>

    <p class="text-surface-300 text-sm">
      The four stated principles are transparency (no NDAs, participants may discuss and publish),
      no academic credit (OpenAI does not seek credit for the discoveries, and being a shepherd does
      not imply authorship), recognition (for the people who build the understanding), and
      independence (unpaid, voluntary, speaking for themselves). OpenAI also acknowledges the cost:
      choosing initial shepherds “necessarily shapes who gets involved first and creates asymmetry of
      information”.
    </p>
    <p class="text-surface-300 text-sm">
      For the mathematics result the strategy was the loosest of the three: publish the AI output and
      leave the community to explain it. That is why this page can exist at all — and also why the
      claim lands in an awkward place. A Lean formalisation is a strong check that the proof follows
      from the stated lemmas; it is not a check that the statement is the theorem people meant, nor
      that the proof is the kind of thing a human would trust at a glance. Kakeya has been open in
      three dimensions for a century; the two-dimensional case alone took until 1971 to settle.
      Skepticism here is not hostility — it is the normal response to a ten-page wave equation
      claiming four conjectures at once.
    </p>
  </section>

  <!-- ───────────────────────── 7 ───────────────────────── -->
  <section id="ledger" class="space-y-4 scroll-mt-16">
    <h2 class="h2">7. The status ledger</h2>
    <p class="text-surface-300">
      Everything this page touches, with its actual status. The last three rows are the ones the
      announcement added.
    </p>
    <div class="overflow-x-auto rounded-lg border border-surface-700">
      <table class="w-full text-sm">
        <thead class="bg-surface-800/60 text-left text-xs uppercase tracking-wider text-surface-400">
          <tr>
            <th class="p-3">Statement</th>
            <th class="p-3 whitespace-nowrap">Status</th>
            <th class="p-3">Who / when</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-surface-800">
          {#each ledger as [claim, status, who]}
            <tr class="align-top">
              <td class="p-3 text-surface-200">{claim}</td>
              <td class="p-3">
                <span
                  class="badge {status === 'theorem'
                    ? 'preset-filled-success-500'
                    : status === 'claimed'
                      ? 'preset-filled-error-500'
                      : 'preset-filled-warning-500'} text-[10px]"
                >
                  {status}
                </span>
              </td>
              <td class="p-3 text-surface-400">{who}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    <p class="text-surface-400 text-sm">
      What to watch for: a refereed version of the preprint; an independent group reproducing the
      argument; and, more interestingly, whether the <em>auxiliary PDE</em> turns out to be the real
      contribution. If it is, the Kakeya conjecture will have been solved by a method that a century
      of harmonic analysts did not try — and the question of who gets credit for it, which OpenAI has
      explicitly declined to claim for itself, will be the least of the complications.
    </p>
  </section>

  <!-- ───────────────────────── sources ───────────────────────── -->
  <section class="space-y-3">
    <h2 class="h2">Sources</h2>
    <ul class="text-sm space-y-2 text-surface-400">
      <li>
        OpenAI, <a class="text-primary-400 hover:underline" href="https://openai.com/index/building-scientific-understanding-with-academic-communities/" target="_blank" rel="noopener">“Working with academic communities to build scientific understanding”</a> (October 2026) — the announcement this page describes.
      </li>
      <li>
        OpenAI, <a class="text-primary-400 hover:underline" href="https://github.com/openai/math/blob/main/preprints/Local-smoothing-for-the-Euclidean-wave-equation-October-25-2026/paper.pdf" target="_blank" rel="noopener">“Local smoothing for the Euclidean wave equation”</a>, 25 October 2026 — the preprint, with its list of claimed corollaries.
      </li>
      <li>
        H. Wang and J. Zahl, <a class="text-primary-400 hover:underline" href="https://arxiv.org/abs/2502.17655" target="_blank" rel="noopener">“Volume estimates for unions of convex sets, and the Kakeya set conjecture in three dimensions”</a>, 2025.
      </li>
      <li>
        L. Guth, H. Wang and R. Zhang, “A sharp square function estimate for the cone in ℝ³”, <em>Annals of Mathematics</em> 192 (2020) — local smoothing in two spatial dimensions.
      </li>
      <li>
        C. Sogge, “Local smoothing of Fourier integral operators and Carleson–Sjölin estimates”, <em>J. Amer. Math. Soc.</em> 6 (1993); the conjecture itself dates to his 1991 work.
      </li>
      <li>
        A. Besicovitch, “Sur deux questions d'intégrabilité des fonctions” (1919) and “On Kakeya's problem and a similar one”, <em>Math. Z.</em> 27 (1928); O. Perron, “Über einen Satz von Besicovitch”, <em>Math. Z.</em> 28 (1928).
      </li>
      <li>
        Terence Tao's applets, <a class="text-primary-400 hover:underline" href="https://teorth.github.io/tao-web/apps/besicovitch-sets.html" target="_blank" rel="noopener">besicovitch-sets</a> and <a class="text-primary-400 hover:underline" href="https://teorth.github.io/tao-web/apps/kakeya3d.html" target="_blank" rel="noopener">kakeya3d</a> — the source of the index formula used for the Perron tree in section 2.
      </li>
    </ul>
    <p class="text-xs text-surface-500 pt-2">
      Every experiment on this page is plain TypeScript running in your browser: a rasteriser, a
      chamfer distance transform, a box counter and an explicit finite-difference wave solver. No
      WebAssembly, no libraries, no precomputed data — resize the window and it measures again.
    </p>
  </section>
</div>
