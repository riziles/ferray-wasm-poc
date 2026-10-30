<script lang="ts">
  import { onMount } from 'svelte';
  import NeedlePuzzle from '$lib/kakeya/NeedlePuzzle.svelte';
  import PerronDemo from '$lib/kakeya/PerronDemo.svelte';

  let scroll = $state(0);

  function onScroll() {
    const doc = document.documentElement;
    const max = doc.scrollHeight - window.innerHeight;
    scroll = max > 0 ? doc.scrollTop / max : 0;
  }

  onMount(() => {
    window.addEventListener('scroll', onScroll, { passive: true });
    // The puzzle builds its rasters on mount; give the browser a frame first.
    onScroll();
    return () => window.removeEventListener('scroll', onScroll);
  });
</script>

<svelte:window on:scroll={onScroll} />

<div class="fixed top-0 left-0 right-0 h-0.5 z-20 bg-surface-800">
  <div class="h-full bg-gradient-to-r from-primary-500 via-secondary-500 to-warning-500" style="width: {scroll * 100}%"></div>
</div>

<div class="container mx-auto px-4 py-8 max-w-4xl space-y-16 pb-24">
  <header class="space-y-6">
    <div class="flex flex-wrap items-center gap-2">
      <span class="badge preset-filled-primary-500">interactive explainer</span>
      <span class="badge preset-tonal-warning">no WASM required</span>
      <span class="badge preset-tonal">pure TypeScript</span>
      <span class="badge preset-tonal">4 live experiments</span>
    </div>
    <h1 class="h1 leading-tight">
      🪡 The Kakeya Conjecture
      <span class="block text-surface-400 text-2xl md:text-3xl mt-2">
        a needle, a room of zero area, and a proof OpenAI says its models found
      </span>
    </h1>
    <p class="text-lg text-surface-300 max-w-3xl">
      In 1917 Sōichi Kakeya asked how small a room could be and still let a unit needle turn all
      the way around. The answer turned out to be “no minimum at all” — and the question that
      replaced it, about the <em>dimension</em> of such sets, is one of the deepest in analysis. In
      October 2026 OpenAI announced that its models had proved the local smoothing conjecture in
      every dimension, which implies the Kakeya conjecture everywhere. This page is an attempt to
      make all of that tangible: four experiments you can turn, slide and count.
    </p>
  </header>

  <section class="space-y-4">
    <h2 class="h2">1. The room problem</h2>
    <p class="text-surface-300">
      A <strong>unit needle</strong> is a line segment of length 1. Drag the needle, turn the dial,
      and try to find a legal placement for every direction. The magnet option does the searching
      for you — it places the needle as deep inside the shape as it can.
    </p>
    <NeedlePuzzle />
  </section>

  <section class="space-y-4">
    <h2 class="h2">2. The room can have no area at all</h2>
    <p class="text-surface-300">
      In 1919 Abram Besicovitch was working on a problem about integrals and needed a set that
      contains a unit segment in every direction but has arbitrarily small area. It should be
      impossible; it is not. Perron's version of the construction is a tree: cut a triangle into
      pieces that all keep their apex on the same line, then slide neighbouring pieces over each
      other. Every piece still holds a unit needle for its own direction — and the pieces overlap
      more and more.
    </p>
    <PerronDemo />
  </section>
</div>
