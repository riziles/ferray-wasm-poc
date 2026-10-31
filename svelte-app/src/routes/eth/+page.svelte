<script lang="ts">
  import { onMount } from 'svelte';
  import { base } from '$app/paths';
  import ExponentLadder from '$lib/eth/ExponentLadder.svelte';
  import RealityRuler from '$lib/eth/RealityRuler.svelte';
  import CopyAndCycle from '$lib/eth/CopyAndCycle.svelte';
  import VacuityLedger from '$lib/eth/VacuityLedger.svelte';

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
    { id: 'hypothesis', n: 1, title: 'What ETH actually asserts', tag: 'text' },
    { id: 'claim', n: 2, title: 'The theorem, as stated', tag: 'text' },
    { id: 'thread', n: 3, title: 'The thread: “just seems absurd to me”', tag: 'live' },
    { id: 'structure', n: 4, title: 'Every formula has structure now', tag: 'text' },
    { id: 'ladder', n: 5, title: 'The exponent is a joke, the limit is not', tag: 'live' },
    { id: 'damage', n: 6, title: 'What breaks, and what does not', tag: 'live' },
    { id: 'mechanism', n: 7, title: 'What the proof actually does', tag: 'live' },
    { id: 'settled', n: 8, title: 'What is not settled', tag: 'text' }
  ];

  // The conversation, verbatim, in posting order. Times are UTC on 27 October
  // 2026; the rkey is the tail of https://bsky.app/profile/moultano.bsky.social/post/<rkey>.
  type Post = { rkey: string; who: string; time: string; text: string; likes: number };
  const thread: Post[] = [
    {
      rkey: '3myt7eostx22o',
      who: 'moultano',
      time: '02:37',
      text: 'The complexity theory result has unsettled me more than the others did. I would guess proximity does that.',
      likes: 80
    },
    {
      rkey: '3myt7kzwfcc2o',
      who: 'moultano',
      time: '02:41',
      text: 'Were we just playing around in a little puddle?',
      likes: 32
    },
    {
      rkey: '3mytcmuujbs2q',
      who: 'norvid-studies',
      time: '03:36',
      text: 'say more about that?',
      likes: 18
    },
    {
      rkey: '3mytcukyijs2o',
      who: 'moultano',
      time: '03:40',
      text: '3 SAT in sub exponential time just seems absurd to me. I never would have thought that was true.',
      likes: 28
    },
    {
      rkey: '3mytd4r7ja22o',
      who: 'moultano',
      time: '03:45',
      text: "It's like the most basic example of a problem where it seems obvious that you can't do much better than just checking all the possibilities. It seems like it can't possibly have any exploitable structure.",
      likes: 43
    },
    {
      rkey: '3mytd5llssk2o',
      who: 'moultano',
      time: '03:45',
      text: "It's just too arbitrary.",
      likes: 27
    },
    {
      rkey: '3mytdcdsrvk2w',
      who: 'bigmuffin69',
      time: '03:48',
      text: "It would bother me if it was realizable, i just find it as strange as sub nlogn multiplication\n\nIf it was polynomial it would fuck me up even if it was galactic tho",
      likes: 5
    },
    {
      rkey: '3mytdh53wqk2o',
      who: 'moultano',
      time: '03:50',
      text: 'The reductions that create the NP-complete class are a little hard to understand but the basic concept is so intuitive. Are there binary problems where you can check a solution, but not find it faster than checking all of them?',
      likes: 25
    },
    {
      rkey: '3mytdh5l7ys2o',
      who: 'moultano',
      time: '03:50',
      text: "And then you describe the most general problem in that class, making a Boolean expression true. It's really really weird that you can do that in the worst case faster* than just checking now.\n\n*in an asymptotic sense.",
      likes: 25
    },
    {
      rkey: '3mytdiqx3jc2w',
      who: 'bigmuffin69',
      time: '03:51',
      text: "++ i do feel excited(?) that we now know that npc isnt all exp algos. Like we have empirical evidence that if*** p =/ np there isn't a clean separation via exponentials, its something more exotic",
      likes: 6
    },
    {
      rkey: '3mytdqojzas2o',
      who: 'moultano',
      time: '03:56',
      text: "What's weird about this is that it's describing worst case complexity, so it isn't just that some boolean circuits have some structure to them, it's that they *all* do. What possible structure could there be in *all* boolean circuits? In common? How does that even make sense?",
      likes: 38
    }
  ];

  const profile = 'https://bsky.app/profile/moultano.bsky.social';
  const postUrl = (p: Post) => `${profile}/post/${p.rkey}`;
  const plink = (rkey: string) => `${profile}/post/${rkey}`;
</script>

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
      <span class="badge preset-filled-primary-500">reading notes</span>
      <span class="badge preset-tonal-warning">no WASM required</span>
      <span class="badge preset-tonal">3 live panels</span>
      <span class="badge preset-tonal">the Bluesky thread, verbatim</span>
      <a href="{base}/kakeya" class="badge preset-tonal">← the Kakeya explainer</a>
    </div>
    <h1 class="h1 leading-tight">
      🧨 The Exponential Time Hypothesis is false
      <span class="block text-surface-400 text-2xl md:text-3xl mt-2">
        and the strangest thing about it is that it is only barely false
      </span>
    </h1>
    <p class="text-lg text-surface-300 max-w-3xl">
      On 25 October 2026 OpenAI posted a preprint with an unusually blunt title:
      <em>The Exponential Time Hypothesis is False</em>. It gives a deterministic algorithm that
      exactly counts the satisfying assignments of a k-CNF in
      <span class="font-mono">2<sup>n/(log n)<sup>1/2+o(1)</sup></sup></span> time — subexponential,
      by a hair, which is all the hypothesis needed. Over the following hours Ryan Moulton
      (<span class="font-mono">@moultano.bsky.social</span>, an algorithmist) wrote the thread that
      everyone quoted: the result unsettles him, 3-SAT in subexponential time “just seems absurd”, and
      the reason it is absurd is one sentence long —
      <em>worst case means every formula has structure, and what structure could possibly be in all of
      them?</em>
    </p>
    <p class="text-surface-300 max-w-3xl">
      This page is about that sentence. It quotes the thread in full with links, states the theorem and
      its three exponents exactly, and puts the arithmetic on screen: the bound is subexponential in a
      sense that no computer will ever notice, and it still demolishes a hypothesis that organised
      twenty-seven years of complexity theory.
    </p>

    <div class="rounded-lg border border-warning-700/50 bg-warning-950/20 p-5 space-y-3">
      <h2 class="font-bold text-warning-300 flex items-center gap-2">
        <span>⚠️</span> Three things this page insists on
      </h2>
      <p class="text-sm text-surface-300">
        <strong>First:</strong> the paper is unrefereed, and the people shepherding it say so more
        plainly than OpenAI does — “our involvement should not be taken as certification of the claim”.
        There is a Lean development, and it proves a <em>weaker</em> bound than the paper states. Both
        facts are load-bearing below, not fine print.
      </p>
      <p class="text-sm text-surface-300">
        <strong>Second:</strong> there are three exponents in play — the abstract's, Theorem 1.1's, and
        the formalised one — and they differ by factors of log log n that decide, in the range anyone
        can plot, whether the theorem is better or worse than exhaustive search. Section 5 puts all
        three on one axis rather than quoting the flattering one.
      </p>
      <p class="text-sm text-surface-400">
        <strong>Third:</strong> the reaction is part of the object. The thread is quoted verbatim with
        a link to every post, including the two replies that push back, because “this is absurd” is
        the correct first reaction and the page should not smooth it away.
      </p>
    </div>

    <nav class="rounded-lg border border-surface-700 bg-surface-800/40 p-4">
      <div class="text-xs uppercase tracking-wider text-surface-500 mb-2">Contents</div>
      <ol class="grid sm:grid-cols-2 gap-x-6 gap-y-1 text-sm">
        {#each contents as c}
          <li class="flex items-baseline gap-2 text-surface-400">
            <span class="font-mono text-surface-600 w-4 shrink-0">{c.n}.</span>
            <a href="#{c.id}" class="hover:text-primary-400 transition-colors flex-1">{c.title}</a>
            <span class="font-mono text-[10px] text-surface-600">{c.tag}</span>
          </li>
        {/each}
      </ol>
    </nav>
  </header>

  <!-- ───────────────────────── 1 ───────────────────────── -->
  <section id="hypothesis" class="space-y-4 scroll-mt-16">
    <h2 class="h2">1. What ETH actually asserts</h2>
    <p class="text-surface-300">
      3-SAT — decide whether a conjunction of three-literal clauses has a satisfying assignment — is
      the canonical NP-complete problem. In 1999 Russell Impagliazzo and Ramamohan Paturi proposed
      turning the folklore belief that it is hard into an assumption with a shape:
    </p>
    <div class="rounded-lg border border-surface-700 bg-surface-950/60 p-4 font-mono text-sm overflow-x-auto">
      <strong>ETH.</strong> there is a c &gt; 0 such that every algorithm deciding 3-SAT needs time
      2<sup>cn</sup> on infinitely many n-variable formulas. Equivalently: 3-SAT ∉ DTIME(2<sup>o(n)</sup>).
    </div>
    <p class="text-surface-300">
      With the sparsification lemma (Impagliazzo, Paturi and Zane) this becomes robust: it survives
      restricting to sparse formulas, so it can be used as the premise of reductions. <strong>SETH</strong>
      strengthens it — for every ε &gt; 0 some clause width k needs 2<sup>(1−ε)n</sup> — and implies ETH.
      Refuting ETH therefore refutes SETH for free, and the OpenAI paper does exactly that, by giving an
      algorithm whose exponent is o(n) for every fixed k.
    </p>
    <p class="text-surface-300">
      The part that matters for everything below is what ETH is <em>not</em>. It is not P ≠ NP, and
      refuting it does not show P = NP. It is a statement about the <em>worst case</em> at a specific
      asymptotic rate, and the algorithm that refutes it runs in 2<sup>o(n)</sup> but not
      2<sup>n<sup>o(1)</sup></sup> — and 2<sup>o(n)</sup> does not survive polynomial-time reductions,
      which is the whole reason ETH was useful. The paper says this itself, in the flattest possible
      sentence: “Note that runtime bounds of the form 2<sup>o(n)</sup> do not transfer under general
      polynomial-time reductions … so our result does not imply that general NP problems admit
      subexponential time algorithms.”
    </p>
    <p class="text-surface-400 text-sm">
      So the shock is not “everything is easy”. It is narrower and, to the people who used ETH daily,
      worse: the map of which problems are near-optimal had a foundation, and the foundation is gone.
    </p>
  </section>

  <!-- ───────────────────────── 2 ───────────────────────── -->
  <section id="claim" class="space-y-4 scroll-mt-16">
    <h2 class="h2">2. The theorem, as stated</h2>
    <p class="text-surface-300">The abstract, verbatim:</p>
    <blockquote class="border-l-2 border-primary-500/60 pl-4 text-surface-300 text-sm space-y-2">
      <p>
        “We give a deterministic algorithm for exactly counting the satisfying assignments of an
        arbitrary k-CNF formula. For every fixed k ⩾ 2, on a formula with n variables and m clauses, our
        algorithm uses poly<sub>k</sub>(n, m) space and runs in time
        <span class="font-mono">(n + m)<sup>O<sub>k</sub>(1)</sup> 2<sup>n/(log n)<sup>1/2+o(1)</sup></sup></span>.
        Our algorithm refutes the Exponential Time Hypothesis (ETH), which postulates that deciding
        3-SAT on n variables requires 2<sup>Ω(n)</sup> time in the worst case.”
      </p>
    </blockquote>
    <p class="text-surface-300">
      Three things about that are worth slowing down for. First, it <strong>counts</strong>: exact
      #k-SAT is #P-complete, so this is stronger than deciding satisfiability — testing whether the
      count is positive decides SAT. Second, it runs in <strong>polynomial space</strong>, which rules
      out the cheap route (enumerate every assignment with a table). Third, Theorem 1.1 inside the
      paper states a sharper, differently-shaped bound:
    </p>
    <div class="rounded-lg border border-surface-700 bg-surface-950/60 p-4 font-mono text-sm overflow-x-auto">
      (n + m)<sup>O<sub>k</sub>(1)</sup> 2<sup>O<sub>k</sub>(n log log n / √log n)</sup> bit operations,
      (n + m)<sup>O<sub>k</sub>(1)</sup> bits of space;<br />
      and 2<sup>O<sub>k</sub>(n / √log n)</sup> when m ⩽ Dn for a fixed D.
    </div>
    <p class="text-surface-300">
      The corollaries are where the collateral damage starts, and they are not small:
    </p>
    <ul class="text-surface-300 space-y-2 text-sm list-disc pl-5">
      <li>
        <strong>1.2</strong> — Max-k-SAT is approximable to within εm of optimal in subexponential time
        (randomised), by sampling clauses and binary-searching the threshold.
      </li>
      <li>
        <strong>1.3</strong> — the number of independent sets, of proper q-colourings, and of perfect
        matchings on a graph is computable deterministically in 2<sup>o(v)</sup> time. Curticapean had
        ruled that out under #ETH.
      </li>
      <li>
        <strong>1.4</strong> — an <em>unconditional</em> circuit lower bound: some language in
        E<sup>NP</sup> has circuits of size greater than c·n·√log n for infinitely many n. A faster
        counting algorithm hands over a lower bound, because it is also a simulation: Bernardo
        Subercaseaux noticed it gives
        <span class="font-mono">NTIME(T) ⊆ DTIME(2<sup>O(T (log log T)³/√log T)</sup>)</span>.
      </li>
    </ul>
    <p class="text-sm text-surface-400 border-l-2 border-surface-600 pl-4">
      <strong>And the Lean version says what it can prove.</strong> The formalisation in
      <span class="font-mono">openai/math</span> establishes
      <span class="font-mono">2<sup>O<sub>k</sub>(n (log log n)²/√log n)</sup></span> — not the
      manuscript's bound, and the doc page says so in as many words: “For m clauses, the October 25
      manuscript gives the sharper time bound …”. A machine has checked the composition of the
      argument, not the headline rate.
    </p>
  </section>

  <!-- ───────────────────────── 3 ───────────────────────── -->
  <section id="thread" class="space-y-4 scroll-mt-16">
    <h2 class="h2">3. The thread: “just seems absurd to me”</h2>
    <p class="text-surface-300">
      When the result landed, Ryan Moulton — whose day job is algorithms, which is why he opens by
      blaming “proximity” — posted the reaction that circulated furthest. The whole exchange is below,
      in order, unedited, each post linked.
    </p>

    <ol class="space-y-2">
      {#each thread as p}
        <li
          class="rounded-lg border p-3 space-y-1 {p.who === 'moultano'
            ? 'border-primary-600/50 bg-primary-950/20'
            : 'border-surface-700 bg-surface-900/40'}"
        >
          <div class="flex items-center gap-2 text-[11px] font-mono">
            <a
              class="hover:underline {p.who === 'moultano' ? 'text-primary-300' : 'text-surface-300'}"
              href={postUrl(p)}
              target="_blank"
              rel="noopener">@{p.who}</a
            >
            <span class="text-surface-600">{p.time} UTC</span>
            <span class="text-surface-600 ml-auto">{p.likes} likes</span>
          </div>
          <p class="text-sm text-surface-200 whitespace-pre-line leading-relaxed">{p.text}</p>
        </li>
      {/each}
    </ol>

    <p class="text-surface-300 text-sm">
      The two replies are the counterweight. <span class="font-mono">@bigmuffin69</span> supplies the
      objection every working algorithmist reaches for — a speedup you cannot realise is a curiosity,
      except that the <em>existence</em> is what breaks the hypothesis — and then the second thought
      that is genuinely the point: NP-complete problems are not all exponential, so if P ≠ NP the
      separation is not a clean exponential one. <span class="font-mono">@norvid-studies</span> asks the
      question the thread then answers for nine posts, which is the right way to read it.
    </p>
    <p class="text-surface-400 text-sm">
      Everything from here on is an attempt to say what the thread says, with the arithmetic attached:
      the
      <a class="text-primary-400 hover:underline" href={plink('3mytcukyijs2o')} target="_blank" rel="noopener"
        >“absurd”</a
      >
      in section 5,
      <a class="text-primary-400 hover:underline" href={plink('3mytd4r7ja22o')} target="_blank" rel="noopener"
        >“no exploitable structure”</a
      >
      in section 4, and
      <a class="text-primary-400 hover:underline" href={plink('3myt7kzwfcc2o')} target="_blank" rel="noopener"
        >“a little puddle”</a
      >
      in section 8.
    </p>
  </section>

  <!-- ───────────────────────── 4 ───────────────────────── -->
  <section id="structure" class="space-y-4 scroll-mt-16">
    <h2 class="h2">4. Every formula has structure now</h2>
    <p class="text-surface-300">
      Moulton's closing observation is the one to keep. ETH is a statement about the <em>worst case</em>:
      there exist formulas that no algorithm handles in 2<sup>cn</sup>. Refuting it does not mean some
      clever trick handles a few more instances. It means, for large enough n, <em>every</em> k-CNF is
      counted faster than 2<sup>n</sup> — so every formula carries something the counting algorithm can
      exploit.
    </p>
    <p class="text-surface-300">
      His formulation is exact: “it isn't just that some boolean circuits have some structure to them,
      it's that they <em>all</em> do. What possible structure could there be in <em>all</em> boolean
      circuits? In common?” That is not a rhetorical complaint. It is the precise thing a proof of this
      shape has to deliver, and the answer in the paper is unrecognisable to anyone raised on the usual
      toolkit: it is not a restriction on the formulas at all, it is an algebraic change of variables
      that makes the sum factorise (section 7). The structure is not <em>in</em> the circuit. It is in
      what you are allowed to replace the variables with.
    </p>
    <p class="text-surface-300">
      Compare with what a fast algorithm normally does. Schöning's algorithm flips variables at random
      and gets (2 − 2/k)<sup>n</sup>; PPSZ derives extra clauses and guesses in a clever order; both
      squeeze the constant. Neither touches the exponent's <em>limit</em>, and both rely on some
      formulas being easy. A refutation of ETH has to beat 2<sup>cn</sup> for every c on all formulas,
      which is why “barely subexponential” is not a hedge — it is the entire content.
    </p>
    <p class="text-surface-400 text-sm">
      There is a philosophical version of the same shock, and the thread reaches it in a subreply —
      “who's the one grading and evaluating a puddle as distinct from an ocean?” If 3-SAT was never the
      ocean, then the intuitions complexity theory spent decades sharpening were intuitions about a
      puddle, and the map was drawn for the wrong terrain.
    </p>
  </section>

  <!-- ───────────────────────── 5 ───────────────────────── -->
  <section id="ladder" class="space-y-4 scroll-mt-16">
    <h2 class="h2">5. The exponent is a joke, the limit is not</h2>
    <p class="text-surface-300">
      “3 SAT in sub exponential time just seems absurd to me.” Here is why it should. Write the running
      time as 2<sup>α·n</sup>; brute force has α = 1, the best concrete 3-SAT decider has α ≈ 0.387,
      and the paper's α tends to 0. The three curves below approach that limit at very different rates,
      and one of them is above brute force for essentially every n that has a name.
    </p>
    <ExponentLadder />
    <div class="text-surface-300 space-y-3 text-sm">
      <p>
        Read the green curve as the honest shape of the claim: α falls, but it is above 0.6 until n
        passes 10<sup>75</sup>. Read the amber curve as a rebuke to anyone quoting the abstract: the
        machine-checked statement is worse than exhaustive search until n ≈ 10<sup>2390</sup>. Neither
        of those is an error. It is what 2<sup>o(n)</sup> looks like when you write the o down.
      </p>
      <p>
        This is the “galactic algorithm” objection, and it is correct — and it is not an objection. ETH
        is itself an asymptotic claim, so an asymptotic refutation is exactly the right weapon, however
        slowly the asymptotics arrive. What the objection <em>does</em> kill is a specific misreading:
        that this paper makes SAT tractable. It does not. The paper's algorithm is the first thing to
        break a barrier, not the best thing that will; within days Ryan O'Donnell and Shiva Kintali had
        posted simplified expositions optimising for understandability rather than bounds, which is a
        better signal than a better exponent.
      </p>
    </div>

    <p class="text-surface-300 text-sm pt-2">
      Put the same numbers on a ruler instead of an axis and the shape of the objection changes. The
      crossings are not merely far away; they are past every physical quantity that could index an
      <span class="font-mono">n</span>-variable problem. Hover or tab to a row for what each mark means:
    </p>
    <RealityRuler />
  </section>

  <!-- ───────────────────────── 6 ───────────────────────── -->
  <section id="damage" class="space-y-4 scroll-mt-16">
    <h2 class="h2">6. What breaks, and what does not</h2>
    <p class="text-surface-300">
      ETH was not a claim in a paper; it was infrastructure. Fine-grained complexity, parameterized
      complexity and hardness of approximation all built conditional lower bounds on it, and those
      statements now have a false premise — which makes them vacuous, not false. Click a hypothesis to
      see which rows were leaning on it.
    </p>
    <VacuityLedger />
    <p class="text-surface-300 text-sm">
      The distinction between <em>vacuous</em> and <em>false</em> is the one that gets lost in the
      coverage. A lower bound proved from ETH is a conditional: ETH ⇒ no fast algorithm. With ETH
      refuted the implication is still formally true and says nothing, because the premise is false.
      Fortnow's summary is the sentence to quote: “The fact that ETH and SETH fail doesn't mean their
      implications are false.” Several of them now rest on weaker hypotheses that may still hold —
      Orthogonal Vectors for LCS and regex matching, an unbounded-width CNF-SAT hypothesis for Fréchet
      distance and treewidth bounds — and nobody has a subexponential algorithm for Subset Sum or TSP,
      because the reductions to fixed-width SAT blow up the variable count.
    </p>
    <p class="text-surface-400 text-sm">
      And a large part of the field's practical advice survives untouched: if you want a fast edit
      distance algorithm you still do not have one, you just cannot cite SETH for why.
    </p>
  </section>

  <!-- ───────────────────────── 7 ───────────────────────── -->
  <section id="mechanism" class="space-y-4 scroll-mt-16">
    <h2 class="h2">7. What the proof actually does</h2>
    <p class="text-surface-300">
      The reason this reads as alien is that none of the usual ingredients are present. There is no
      branching on clauses, no measure-and-conquer, no local search. Instead the count is written as a
      polynomial in a variable T and recovered by interpolation, and the work is in arranging for the
      sum to factorise. Four moves:
    </p>
    <ol class="text-surface-300 space-y-3 text-sm list-decimal pl-5">
      <li>
        <strong>Sparsify, then flatten.</strong> The counting-preserving sparsification lemma expresses
        the count as a sum over sparse formulas, and each is rewritten as a bounded-occurrence 3-CNF
        with implication cycles forcing copied variables to agree. Everything downstream assumes each
        variable occurs a constant number of times.
      </li>
      <li>
        <strong>Replace each variable by a low-degree polynomial.</strong> Each occurrence (e, v) gets a
        row P<sub>e,v</sub>(T) of degree at most R with P(0) = the variable it replaces. The
        replacements are built from an auxiliary matrix M<sub>0</sub> = V<sub>0</sub> f(W<sub>0</sub>),
        where the Π<sub>s</sub> are commuting swaps and f interpolates (−1)<sup>j</sup> on a short
        interval of integers. That last trick — interpolating (−1)<sup>j</sup> to make most entries of a
        matrix vanish — comes from the matrix-rigidity literature, which is not where SAT algorithms
        usually shop.
      </li>
      <li>
        <strong>Fix a few shared measurements, then factor.</strong> At each evaluation point the
        variables are partitioned into groups, and clauses are assigned to groups. Once a few integer
        linear measurements C<sub>t,g</sub> are fixed, each clause depends only on its own group, so the
        sum over 2<sup>n</sup> assignments collapses into a product of per-group sums — each over
        2<sup>2n/r</sup> assignments. This is the whole engine.
      </li>
      <li>
        <strong>Recover the count by Hermite interpolation.</strong> What the routine computes is a
        handful of Taylor coefficients at t = 1, …, Q, not the value at 0. Since the degree is bounded,
        Q·J coefficients determine the polynomial, and its constant term is the count.
      </li>
    </ol>
    <p class="text-surface-300 text-sm">
      Only the first move is small enough to watch happen, and it is also the only one whose claim is
      exactly checkable: the rewrite is supposed to leave the number of satisfying assignments alone.
      So here is a real formula, rewritten, with both counts computed in the browser by exact model
      counting.
    </p>
    <CopyAndCycle />
    <p class="text-surface-300 text-sm">
      The paper writes the cost as three multiplied factors and then balances them; this is the
      clearest single paragraph in it:
    </p>
    <div class="rounded-lg border border-surface-700 bg-surface-950/60 p-4 font-mono text-xs overflow-x-auto">
      Σ<sub>j≤q</sub> 2<sup>j</sup>C(n,j) · (2D+1)<sup>2τn</sup> · 2<sup>2n/r</sup><br />
      <span class="text-surface-500"
        >Taylor data · measurements · one group &nbsp;(equation 2.11)</span
      >
    </div>
    <p class="text-surface-300 text-sm">
      and the parameters it settles on are λ = ⌈log₂ n⌉, ν = ⌈log₂ λ⌉, r ≈ √(λν²), Q ≈ λ/(128ν),
      R ≈ 4√λ, τ = 1/(rλ), D = r<sup>2Q</sup>. Everything exponential lives in D = r<sup>2Q</sup>, which
      is why the polynomial factor in (n + m)<sup>O<sub>k</sub>(1)</sup> is where the constants hide, and
      why nobody is going to implement this.
    </p>
    <p class="text-surface-400 text-sm">
      None of that is a criticism of the proof; it is why the result was not found by the people who
      spent careers on the problem. It is also why the check is taking a while: the argument is
      algebraic, long, and unfamiliar, and the Lean development covers a bound slightly weaker than the
      paper's headline.
    </p>
  </section>

  <!-- ───────────────────────── 8 ───────────────────────── -->
  <section id="settled" class="space-y-4 scroll-mt-16">
    <h2 class="h2">8. What is not settled</h2>
    <ul class="text-surface-300 space-y-3 text-sm list-disc pl-5">
      <li>
        <strong>Refereeing.</strong> None of this has been reviewed. The shepherds — Impagliazzo,
        Paturi and Williams — opened a public examination at
        <a class="text-primary-400 hover:underline" href="https://ethdiscussion.blog/" target="_blank" rel="noopener">ethdiscussion.blog</a>
        and stated: “Our involvement should not be taken as certification of the claim.” That is the
        right posture, and it is a stronger statement than the announcement's.
      </li>
      <li>
        <strong>The formalisation is weaker than the paper.</strong> Lean proves
        2<sup>O(n (log log n)²/√log n)</sup>; the manuscript claims 2<sup>O(n log log n/√log n)</sup>.
        The gap is a log log n in the exponent. Machine-checking the weaker statement is excellent
        evidence the argument composes, and no evidence at all for the sharper rate.
      </li>
      <li>
        <strong>A second group got there too.</strong> Anthropic released its own ETH refutation hours
        later, with Levent Alpöge suggesting the OpenAI work built on prior results. Two independent
        refutations is much stronger than one, and it also means the “who found it first” story is not
        the interesting one.
      </li>
      <li>
        <strong>The constants.</strong> Nobody can run this algorithm; the O<sub>k</sub>(1) polynomial
        and D = r<sup>2Q</sup> are not implementable, and the exponentials in the parameter list are
        bigger than the saving. Whether a practical 2<sup>(1−ε)n</sup> algorithm exists is a different,
        open question, and Williams is on record doubting this one is optimal: “I don't think that
        OpenAI's algorithm could possibly be the best algorithm you can get.”
      </li>
      <li>
        <strong>Cryptography.</strong> The reflexive worry is that faster SAT breaks things. Williams
        asked OpenAI's Dan Roberts directly, and reports they checked and were “reasonably confident”
        it implies no breaks — 2<sup>o(n)</sup> does not reach the key sizes in use, for the reason in
        section 1.
      </li>
      <li>
        <strong>Was ETH <em>morally</em> true?</strong> This is the puddle question, and it is not
        settled by the paper. If the only counterexamples need 10<sup>75</sup> variables, then the
        community's intuition was tracking something real that ETH stated in the wrong currency. The
        thread's last word is the right question to end on: whether the field was playing in a puddle,
        and who gets to decide where the ocean starts.
      </li>
    </ul>
    <p class="text-surface-400 text-sm">
      What is already clear is the shape of the thing: a hypothesis that organised a field for
      twenty-seven years was refuted by an algorithm nobody would ever run, in a proof nobody
      recognises, announced with a process the field is still arguing about. That is a stranger event
      than “AI solved a maths problem”, which is why the reaction on Bluesky was not
      <em>wow</em> but <em>wait, what does that even mean</em>.
    </p>
  </section>

  <!-- ───────────────────────── sources ───────────────────────── -->
  <section class="space-y-3">
    <h2 class="h2">Sources</h2>
    <ul class="text-sm space-y-2 text-surface-400">
      <li>
        OpenAI,
        <a
          class="text-primary-400 hover:underline"
          href="https://openai.com/index/building-scientific-understanding-with-academic-communities/"
          target="_blank"
          rel="noopener">“Working with academic communities to build scientific understanding”</a
        >, 25 October 2026 — the announcement, the three release strategies, and the shepherding group.
      </li>
      <li>
        OpenAI,
        <a
          class="text-primary-400 hover:underline"
          href="https://github.com/openai/math/blob/main/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026/paper.pdf"
          target="_blank"
          rel="noopener">“The Exponential Time Hypothesis is False”</a
        >, 17 October 2026, revised 25 October 2026 — all quotations, theorem and corollary numbers, and
        the parameter list in section 7.
      </li>
      <li>
        OpenAI,
        <a
          class="text-primary-400 hover:underline"
          href="https://github.com/openai/math/blob/main/lean/docs/378.md"
          target="_blank"
          rel="noopener">Lean formalisation notes (378)</a
        > — the formalised bound, and the explicit note that the manuscript's is sharper.
      </li>
      <li>
        R. Impagliazzo, R. Paturi and R. Williams,
        <a
          class="text-primary-400 hover:underline"
          href="https://ethdiscussion.blog/2026/10/26/eth-refutation-examination-and-discussion/"
          target="_blank"
          rel="noopener">open letter and examination site</a
        >, 26 October 2026 — “not to be taken as certification”, plus the community repository.
      </li>
      <li>
        R. Impagliazzo and R. Paturi, “Complexity of k-SAT”, <em>CCC</em> 1999, and Impagliazzo, Paturi,
        Zane, “Which problems have strongly exponential complexity?”, <em>JCSS</em> 2001 — ETH and the
        sparsification lemma.
      </li>
      <li>
        L. Fortnow,
        <a
          class="text-primary-400 hover:underline"
          href="https://blog.computationalcomplexity.org/2026/10/the-exponential-time-refutation.html"
          target="_blank"
          rel="noopener">“The Exponential Time Refutation”</a
        >, <em>Computational Complexity</em>, 2026 — the vacuity discussion in section 6, Subercaseaux's
        simulation, and the Subset Sum caveat.
      </li>
      <li>
        R. O'Donnell,
        <a
          class="text-primary-400 hover:underline"
          href="https://www.cs.cmu.edu/~odonnell/some_eth_false_exposition.pdf"
          target="_blank"
          rel="noopener">“Some ETH False exposition”</a
        >; S. Kintali,
        <a class="text-primary-400 hover:underline" href="https://shivakintali.github.io/papers/ETH.pdf" target="_blank" rel="noopener">notes on ETH</a> —
        independent simplifications.
      </li>
      <li>
        Anthropic,
        <a
          class="text-primary-400 hover:underline"
          href="https://github.com/anthropics/formal-math/tree/main/eth"
          target="_blank"
          rel="noopener">formal-math/eth</a
        > — their own refutation, released the same day.
      </li>
      <li>
        TechCrunch,
        <a
          class="text-primary-400 hover:underline"
          href="https://techcrunch.com/2026/10/27/frontier-ai-labs-say-their-models-refuted-the-exponential-time-hypothesis-what-does-that-mean/"
          target="_blank"
          rel="noopener">“Frontier AI labs say their models refuted the Exponential Time Hypothesis”</a
        >, 27 October 2026, and Science,
        <a
          class="text-primary-400 hover:underline"
          href="https://www.science.org/content/article/ai-cracks-27-year-old-pillar-computer-science-openai-and-anthropic-claim"
          target="_blank"
          rel="noopener">“AI cracks a 27-year-old pillar of computer science”</a
        > — the Williams and Impagliazzo quotes in section 8.
      </li>
      <li>
        The Bluesky conversation:
        <a class="text-primary-400 hover:underline" href={plink('3myt7eostx22o')} target="_blank" rel="noopener"
          >the root post</a
        >, with every other post linked in section 3 (<span class="font-mono">@moultano.bsky.social</span>,
        27 October 2026).
      </li>
    </ul>
    <p class="text-xs text-surface-500 pt-2">
      All four panels are plain TypeScript and CSS. The exponent chart evaluates 1/√ln n, ln ln n/√ln n
      and (ln ln n)²/√ln n at 260 sample points and bisects for the crossover values in the caption; the
      ruler places the same crossings against physical quantities on a log-log axis and is built from DOM
      text rather than a canvas, so its labels are selectable and every mark is a real button you can
      hover or tab to; the ledger is a table with a dependency filter; and the rewrite panel generates a
      random 3-CNF, splits its variables, counts both formulas by exact model counting (branching plus
      unit propagation) and compares the totals — the “count preserved” badge is a computation, not a
      caption. Nothing here is precomputed from the paper or copied from its figures.
    </p>
  </section>
</div>
