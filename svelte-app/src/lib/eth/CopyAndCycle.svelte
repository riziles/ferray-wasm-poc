<script lang="ts">
  // The counting-preserving rewrite, run on a formula you can read: split every
  // variable that occurs more than once into copies, add Tovey's implication
  // cycle, and check that the number of satisfying assignments did not move.
  // Both counts are computed in the browser by exact model counting — the two
  // numbers on screen are the whole claim.

  import {
    clauseText,
    copyAndCycle,
    countModels,
    histogram,
    randomFormula
  } from './copycycle';

  let n = $state(5);
  let m = $state(4);
  let seed = $state(7);

  const formula = $derived(randomFormula(n, m, seed));
  const rewrite = $derived(copyAndCycle(formula));

  const before = $derived(countModels(formula));
  const after = $derived(countModels(rewrite.formula));

  const histBefore = $derived(histogram(rewrite.occBefore));
  const histAfter = $derived(histogram(rewrite.occAfter));
  const splitVars = $derived(rewrite.copies.filter((c) => c > 1).length);
  const worstAfter = $derived(Math.max(...rewrite.occAfter));

  const bar = (vars: number, max: number) => `${Math.max(4, (vars / max) * 100)}%`;
  const maxCount = (h: { vars: number }[]) => Math.max(...h.map((x) => x.vars), 1);
</script>

<div class="rounded-lg border border-surface-700 bg-surface-950/40">
  <div class="p-3 border-b border-surface-800 space-y-3">
    <div class="grid sm:grid-cols-2 gap-3">
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap w-16">variables</span>
        <input type="range" min="3" max="6" step="1" bind:value={n} class="flex-1" />
        <span class="font-mono w-4 text-right">{n}</span>
      </label>
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap w-16">clauses</span>
        <input type="range" min="2" max="8" step="1" bind:value={m} class="flex-1" />
        <span class="font-mono w-4 text-right">{m}</span>
      </label>
    </div>
    <div class="flex flex-wrap items-center gap-2">
      <button
        class="badge preset-filled-primary-500"
        onclick={() => (seed = (seed + 1) % 1000)}>new random formula</button
      >
      <span class="text-xs text-surface-500">
        {splitVars} of {formula.n} variables split · {formula.n} → {rewrite.formula.n} variables ·{' '}
        {formula.clauses.length} → {rewrite.formula.clauses.length} clauses
      </span>
    </div>
  </div>

  <div class="grid md:grid-cols-2 divide-y md:divide-y-0 md:divide-x divide-surface-800">
    <div class="p-3 space-y-2">
      <div class="text-[11px] uppercase tracking-wider text-surface-500">before · arbitrary 3-CNF</div>
      <div class="font-mono text-[11px] leading-relaxed text-surface-300 max-h-40 overflow-auto space-y-0.5">
        {#each formula.clauses as c, i}
          <div class="whitespace-nowrap">
            <span class="text-surface-600">{i + 1}.</span>
            {clauseText(c)}
          </div>
        {/each}
      </div>
      <div class="text-xs text-surface-400">
        occurrences:
        {#each histBefore as h, i}
          {#if i > 0}<span class="text-surface-600"> · </span>{/if}<span class="font-mono text-surface-200">{h.k}×</span>
          <span class="text-surface-500">({h.vars} var{h.vars === 1 ? '' : 's'})</span>
        {/each}
      </div>
    </div>

    <div class="p-3 space-y-2">
      <div class="text-[11px] uppercase tracking-wider text-surface-500">
        after · copies + implication cycle
      </div>
      <div class="font-mono text-[11px] leading-relaxed text-surface-300 max-h-40 overflow-auto space-y-0.5">
        {#each rewrite.formula.clauses as c, i}
          <div
            class="whitespace-nowrap"
            class:text-success-300={i >= formula.clauses.length}
          >
            <span class="text-surface-600">{i + 1}.</span>
            {clauseText(c)}
          </div>
        {/each}
      </div>
      <div class="text-xs text-surface-400">
        occurrences:
        {#each histAfter as h, i}
          {#if i > 0}<span class="text-surface-600"> · </span>{/if}<span class="font-mono text-surface-200">{h.k}×</span>
          <span class="text-surface-500">({h.vars} var{h.vars === 1 ? '' : 's'})</span>
        {/each}
      </div>
    </div>
  </div>

  <div class="p-3 border-t border-surface-800 space-y-3">
    <div class="flex flex-wrap items-center gap-3 text-xs">
      <span class="text-surface-400">
        #F before <span class="font-mono text-surface-100 ml-1">{before.toLocaleString('en-US')}</span>
      </span>
      <span class="text-surface-400">
        #F after <span class="font-mono text-surface-100 ml-1">{after.toLocaleString('en-US')}</span>
      </span>
      <span
        class="badge"
        class:preset-filled-success-500={before === after}
        class:preset-filled-error-500={before !== after}
      >
        {before === after ? '✓ count preserved' : '✗ count changed — that would be a bug'}
      </span>
      <span
        class="badge"
        class:preset-filled-success-500={worstAfter <= 3}
        class:preset-filled-error-500={worstAfter > 3}
      >
        max occurrences {worstAfter} {worstAfter <= 3 ? '≤ 3 ✓' : ''}
      </span>
    </div>

    <div class="space-y-1">
      <div class="text-[10px] uppercase tracking-wider text-surface-600">occurrences per variable</div>
      <div class="flex items-end gap-3">
        {#each [histBefore, histAfter] as h, side}
          <div class="flex-1 space-y-1">
            <div class="text-[10px] text-surface-500">{side === 0 ? 'before' : 'after'}</div>
            {#each h as row}
              <div class="flex items-center gap-2">
                <span class="font-mono text-[10px] text-surface-500 w-6">{row.k}×</span>
                <div class="h-2 rounded-sm flex-1 bg-surface-800 overflow-hidden">
                  <div
                    class="h-full {side === 0 ? 'bg-surface-500' : 'bg-primary-500'}"
                    style="width: {bar(row.vars, maxCount(h))}"
                  ></div>
                </div>
                <span class="font-mono text-[10px] text-surface-400 w-4">{row.vars}</span>
              </div>
            {/each}
          </div>
        {/each}
      </div>
    </div>

    <p class="text-xs text-surface-500 leading-relaxed">
      The green clauses are the implication cycles: for a variable split into copies x₁, x₂, x₃ the
      cycle is (¬x₁∨x₂) ∧ (¬x₂∨x₃) ∧ (¬x₃∨x₁), which forces all three copies to take the same value.
      Every copy now appears in one inherited clause and two cycle clauses — exactly three occurrences,
      which is the hypothesis the rest of the paper runs on. Note what this does <em>not</em> show: the
      rewrite makes the formula <em>bigger</em>, and it is only the first of the paper's four moves. The
      count is what survives, and that is the one thing here that is checked rather than asserted.
    </p>
  </div>
</div>
