<script lang="ts">
  // The exponent chart says α falls; this says where the crossings sit against
  // things that exist. Built from DOM elements rather than a canvas on purpose:
  // every label here is real text — selectable, copyable, and readable by a
  // screen reader — and each mark is a button, so hovering *or* tabbing to a row
  // shows what it means. A canvas would make all of that a picture.
  //
  // The axis is log10(log10 n): an ordinary log axis crams 10^23 to 10^2390 into
  // the last pixel.

  import {
    exponentOfBase,
    log10NForAbstractExponent,
    log10NForLeanExponent,
    log10NForManuscriptExponent,
    superscript
  } from './numerics';

  interface Mark {
    /** log10 of n. */
    lg: number;
    label: string;
    detail: string;
    kind: 'math' | 'physical';
  }

  const lgAbstract110 = log10NForAbstractExponent(exponentOfBase(1.1));
  const lgManuscript1308 = log10NForManuscriptExponent(exponentOfBase(1.308));
  const lgLeanOne = log10NForLeanExponent(1);

  const marks: Mark[] = (
    [
      {
        lg: Math.log10(55),
        label: 'n = 55 — α falls to ½',
        detail:
          'The abstract’s exponent 1/√ln n reaches ½ here. Below this n the bound is worse than 1.414ⁿ.',
        kind: 'math'
      },
      {
        lg: Math.log10(8.9e6),
        label: 'n ≈ 10⁷ — α falls to ¼',
        detail:
          '…and reaches ¼: still slower than every real algorithm, but the limit has become visible.',
        kind: 'math'
      },
      {
        lg: 27,
        label: '10²⁷ atoms — a person',
        detail: 'Atoms in a 70 kg adult, mostly hydrogen. The first quantity here that is smaller than n.',
        kind: 'physical'
      },
      {
        lg: 47,
        label: '10⁴⁷ ops — every computer, ever',
        detail:
          '10³⁰ operations per second sustained for the 1.4×10¹⁰ years since the Big Bang. Every computation ever performed, added together.',
        kind: 'physical'
      },
      {
        lg: 50,
        label: '10⁵⁰ atoms — the Earth',
        detail: 'Atoms in the Earth, ~10⁵⁰, dominated by iron, oxygen and silicon.',
        kind: 'physical'
      },
      {
        lg: lgAbstract110,
        label: `n ≈ 10${superscript(Math.round(lgAbstract110))} — abstract beats 1.1ⁿ`,
        detail:
          'Past this, the abstract’s exponent is smaller than a 1.1ⁿ algorithm’s 0.137 — in the limit. The hidden O_k(1) constants are why that sentence says nothing about running time at this n.',
        kind: 'math'
      },
      {
        lg: lgManuscript1308,
        label: `n ≈ 10${superscript(Math.round(lgManuscript1308))} — Thm 1.1 beats 1.308ⁿ`,
        detail:
          'Theorem 1.1’s exponent ln ln n/√ln n finally falls below 0.387, the exponent of the best deterministic #3-SAT counter.',
        kind: 'math'
      },
      {
        lg: 80,
        label: '10⁸⁰ atoms — observable universe',
        detail:
          'Atoms in the observable universe. A formula with this many variables would have more variables than the universe has atoms.',
        kind: 'physical'
      },
      {
        lg: 185,
        label: '10¹⁸⁵ Planck volumes — universe',
        detail:
          'How many Planck volumes fit in the observable universe. A grid with one variable per Planck volume has this many cells.',
        kind: 'physical'
      },
      {
        lg: lgLeanOne,
        label: `n ≈ 10${superscript(Math.round(lgLeanOne))} — Lean beats 2ⁿ`,
        detail:
          'Here the machine-checked bound (ln ln n)²/√ln n drops below 1 and stops being worse than trying every assignment. This is the only version of the theorem Lean proves.',
        kind: 'math'
      }
    ] satisfies Mark[]
  ).sort((a, b) => a.lg - b.lg);

  const LO = 0.45;
  const HI = 3.45;
  /** Position of a mark, as a percentage of the track. */
  const pct = (lg: number) => {
    const v = Math.log10(Math.max(lg, 0.2));
    return Math.min(100, Math.max(0, ((v - LO) / (HI - LO)) * 100));
  };

  // Grid lines at every tick; labels only where they will not collide.
  const ticks = [3, 10, 30, 100, 300, 1000, 2400];
  const labelTicks = [3, 10, 100, 1000];
  const pctAtoms = pct(80);

  let active = $state<number | null>(null);
  const shown = $derived(active === null ? null : marks[active]);
</script>

<div class="rounded-lg border border-surface-700 bg-surface-950/40 overflow-hidden">
  <div class="p-3 sm:p-4 space-y-2">
    <div class="flex flex-wrap items-center gap-x-4 gap-y-1 text-[10px] uppercase tracking-wider">
      <span class="text-surface-500">n, on a log₁₀ log₁₀ scale →</span>
      <span class="text-primary-300">● crossover</span>
      <span class="text-surface-400">● physical</span>
      <span class="text-surface-600 normal-case tracking-normal">hover or tab a row</span>
    </div>

    <div>
      {#each marks as m, i}
        <button
          type="button"
          class="flex flex-col sm:flex-row sm:items-center gap-0.5 sm:gap-3 w-full py-1 sm:py-0 sm:h-7
            text-left rounded transition-colors
            {active === i ? 'bg-primary-500/10' : 'hover:bg-surface-800/60'}"
          onmouseenter={() => (active = i)}
          onmouseleave={() => (active = null)}
          onfocus={() => (active = i)}
          onblur={() => (active = null)}
        >
          <span
            class="w-full sm:w-60 shrink-0 text-left sm:text-right text-[11px] leading-tight
              sm:truncate {m.kind === 'math' ? 'text-primary-200' : 'text-surface-400'}"
            title={m.label}
          >
            {m.label}
          </span>

          <span class="relative w-full flex-1 h-4 sm:h-full" aria-hidden="true">
            <!-- "more variables than atoms in the universe" -->
            <span
              class="absolute inset-y-0 bg-error-500/5 border-l border-error-500/20"
              style="left: {pctAtoms}%; right: 0"
            ></span>
            {#each ticks as t}
              <span
                class="absolute inset-y-0 border-l border-surface-700/25"
                style="left: {pct(t)}%"
              ></span>
            {/each}
            <!-- from the label column to the dot -->
            <span
              class="absolute top-1/2 h-px {m.kind === 'math' ? 'bg-primary-500/30' : 'bg-surface-600/40'}"
              style="left: 0; width: {pct(m.lg)}%"
            ></span>
            <span
              class="absolute top-1/2 -translate-x-1/2 -translate-y-1/2 rounded-full transition-transform
                {m.kind === 'math' ? 'size-[7px] bg-primary-400' : 'size-[5px] bg-surface-400'}
                {active === i ? 'scale-150' : ''}"
              style="left: {pct(m.lg)}%"
            ></span>
          </span>
        </button>
      {/each}

      <!-- axis: the same flex layout, so the ticks share the tracks' geometry -->
      <div class="flex flex-col sm:flex-row sm:items-center gap-0.5 sm:gap-3 pt-1">
        <div class="w-full sm:w-60 shrink-0 text-left sm:text-right text-[10px] text-surface-600">
          log₁₀ n
        </div>
        <div class="relative w-full flex-1 h-5 border-t border-surface-600">
          {#each ticks as t}
            <span class="absolute top-0 h-1 border-l border-surface-600" style="left: {pct(t)}%"></span>
          {/each}
          {#each labelTicks as t}
            <span
              class="absolute top-1.5 -translate-x-1/2 text-[10px] font-mono text-surface-500"
              style="left: {pct(t)}%">10{superscript(t)}</span
            >
          {/each}
        </div>
      </div>
    </div>

    <!-- detail strip: same height whether or not something is hovered -->
    <div class="min-h-[3.25rem] rounded border border-surface-800 bg-surface-900/40 px-2.5 py-2" aria-live="polite">
      {#if shown}
        <div class="flex flex-wrap items-baseline gap-x-2 text-[11px]">
          <span class="font-mono {shown.kind === 'math' ? 'text-primary-300' : 'text-surface-300'}">
            n ≈ 10{superscript(Math.round(shown.lg))}
          </span>
          <span class="text-surface-500">{shown.kind === 'math' ? 'crossover' : 'physical quantity'}</span>
        </div>
        <p class="text-xs text-surface-400 leading-snug mt-0.5">{shown.detail}</p>
      {:else}
        <p class="text-xs text-surface-600 leading-snug">
          Hover or tab to a mark to see what it measures. Cyan marks are where one of the paper’s three
          exponents crosses a reference; grey marks are physical quantities.
        </p>
      {/if}
    </div>

    <p class="text-xs text-surface-500 leading-relaxed">
      The axis is <span class="font-mono">log₁₀ log₁₀ n</span>, because these marks span 10²³ to 10²³⁹⁰ and
      an ordinary log axis would stack them on the last pixel. The grey marks are here to be reassuringly
      wrong: every computer that has ever run has performed at most about 10⁴⁷ operations, and the
      machine-checked bound does not beat exhaustive search until n is larger than the number of Planck
      volumes in the observable universe. None of that refutes the paper — the claim it refutes is
      asymptotic — but it is what “subexponential” looks like once the o is measured against something.
    </p>
  </div>
</div>
