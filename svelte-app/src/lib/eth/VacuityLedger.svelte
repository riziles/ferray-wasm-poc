<script lang="ts">
  // "What does refuting ETH actually break?" — the damage is not uniform, so the
  // table is a dependency ledger you can query. Click a hypothesis to see which
  // rows were resting on it; rows that still have a footing say which one.
  //
  // Sources for the rows: the paper's sections 1.1.3 and 1.2, and Lance Fortnow's
  // post at blog.computationalcomplexity.org (2026-10-28), which is
  // careful about exactly this point — "the fact that ETH and SETH fail doesn't
  // mean their implications are false."

  type Hyp = 'ETH' | 'SETH' | '#ETH' | 'OVC' | 'CNF-SAT';

  interface Row {
    claim: string;
    deps: Hyp[];
    fate: 'vacuous' | 'weaker' | 'new';
    note: string;
  }

  const hypotheses: { id: Hyp; label: string; status: string; dead: boolean }[] = [
    { id: 'ETH', label: 'ETH', status: 'refuted here', dead: true },
    { id: 'SETH', label: 'SETH', status: 'refuted with it', dead: true },
    { id: '#ETH', label: '#ETH', status: 'refuted here', dead: true },
    { id: 'OVC', label: 'Orthogonal Vectors', status: 'still open', dead: false },
    { id: 'CNF-SAT', label: 'CNF-SAT (growing k)', status: 'untouched', dead: false }
  ];

  const rows: Row[] = [
    {
      claim: 'Clique needs 2^Ω(n) time on n-vertex graphs (2^Ω(√m) on m edges)',
      deps: ['ETH'],
      fate: 'vacuous',
      note: 'The reduction ran ETH into Clique; with ETH gone the implication has no premise.'
    },
    {
      claim: 'Clique has no f(k)·n^o(k) algorithm — the trivial n^O(k) is optimal',
      deps: ['ETH'],
      fate: 'vacuous',
      note: 'Same premise. Nobody has a faster clique algorithm; the lower bound simply is not proved any more.'
    },
    {
      claim: 'Hitting Set and Set Splitting have sharp exponential bounds',
      deps: ['SETH'],
      fate: 'vacuous',
      note: 'For Set Splitting at fixed set size k, the new algorithm does not just vacate the bound — it beats it, because the reduction has one variable per universe element.'
    },
    {
      claim: 'Edit distance, LCS and DTW admit no quadratic-factor speedup',
      deps: ['SETH'],
      fate: 'weaker',
      note: 'LCS and regular-expression matching still follow from the Orthogonal Vectors Conjecture, which SETH implied but which can hold on its own.'
    },
    {
      claim: 'Fréchet distance and graph-diameter approximation stay hard',
      deps: ['SETH'],
      fate: 'weaker',
      note: 'These also follow from CNF-SAT admitting no 2^(1−δ)n algorithm — a hypothesis with growing clause width, which this paper does not touch.'
    },
    {
      claim: 'Independent Set / Dominating Set by treewidth are optimal',
      deps: ['SETH'],
      fate: 'weaker',
      note: 'Survives on the growing-width CNF-SAT hypothesis, per the paper’s own section 1.1.3.'
    },
    {
      claim: 'Subset Sum and Bicriteria Path admit no pseudopolynomial speedup',
      deps: ['SETH'],
      fate: 'vacuous',
      note: 'No replacement algorithm follows: reductions from these problems to fixed-width k-SAT blow up the variable count. The best algorithms are still ~2^(n/2) and ~2ⁿ.'
    },
    {
      claim: 'Counting perfect matchings takes 2^Ω(v) (Curticapean, under #ETH)',
      deps: ['#ETH'],
      fate: 'vacuous',
      note: 'Corollary 1.3 now counts perfect matchings deterministically in 2^o(v+e) time.'
    },
    {
      claim: 'Some L ∈ E^NP has circuits of size > c·n·√log n for infinitely many n',
      deps: [],
      fate: 'new',
      note: 'Corollary 1.4. An unconditional circuit lower bound falls out of a faster counting algorithm — the same shape as Williams’ NEXP result, in a place nobody expected to look.'
    },
    {
      claim: 'NTIME(T) ⊆ DTIME(2^O(T (log log T)³ / √log T))',
      deps: [],
      fate: 'new',
      note: 'Bernardo Subercaseaux’s observation, reported by Fortnow: a nontrivial simulation of nondeterministic time by deterministic time, unconditionally.'
    },
    {
      claim: 'Max-k-SAT is approximable to OPT − εm in subexponential time',
      deps: [],
      fate: 'new',
      note: 'Corollary 1.2. This sits awkwardly beside the Unique Games-based inapproximability for the same problem, and that tension is part of what the community is checking.'
    }
  ];

  let selected = $state<Hyp | null>(null);

  const shown = $derived(
    rows.filter((r) => selected === null || r.deps.includes(selected) || r.fate === 'new')
  );
  const count = $derived(rows.filter((r) => selected !== null && r.deps.includes(selected)).length);
</script>

<div class="rounded-lg border border-surface-700 bg-surface-950/40">
  <div class="p-3 border-b border-surface-800 space-y-2">
    <div class="flex flex-wrap gap-2">
      <button
        class="badge transition-colors"
        class:preset-filled-primary-500={selected === null}
        class:preset-tonal={selected !== null}
        onclick={() => (selected = null)}>all rows</button
      >
      {#each hypotheses as h}
        <button
          class="badge transition-colors"
          class:preset-filled-primary-500={selected === h.id}
          class:preset-tonal={selected !== h.id}
          onclick={() => (selected = selected === h.id ? null : h.id)}
        >
          {h.label}
          <span class="opacity-60 ml-1">{h.status}</span>
        </button>
      {/each}
    </div>
    {#if selected}
      <p class="text-xs text-surface-400">
        {count} row{count === 1 ? '' : 's'} in this ledger rest{count === 1 ? 's' : ''} on
        <span class="font-mono text-surface-200">{selected}</span>. Rows marked
        <span class="text-success-300">new</span> are results the paper creates, not casualties.
      </p>
    {/if}
  </div>

  <ul class="divide-y divide-surface-800">
    {#each shown as r}
      <li
        class="p-3 space-y-1 transition-opacity"
        class:opacity-45={selected !== null && !r.deps.includes(selected) && r.fate !== 'new'}
      >
        <div class="flex items-start gap-2">
          <span
            class="badge shrink-0 text-[10px]"
            class:preset-filled-error-500={r.fate === 'vacuous'}
            class:preset-filled-warning-500={r.fate === 'weaker'}
            class:preset-filled-success-500={r.fate === 'new'}
          >
            {r.fate === 'vacuous' ? 'vacuous' : r.fate === 'weaker' ? 'still stands' : 'new'}
          </span>
          <div class="min-w-0">
            <div class="text-sm text-surface-200 leading-snug">{r.claim}</div>
            <div class="text-xs text-surface-500 mt-1 leading-relaxed">{r.note}</div>
            {#if r.deps.length}
              <div class="text-[10px] font-mono text-surface-600 mt-1">
                was conditional on {r.deps.join(' + ')}
              </div>
            {:else}
              <div class="text-[10px] font-mono text-surface-600 mt-1">unconditional</div>
            {/if}
          </div>
        </div>
      </li>
    {/each}
  </ul>
</div>
