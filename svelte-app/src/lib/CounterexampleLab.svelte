<script lang="ts">
  import { onMount } from 'svelte';

  let { wasm } = $props<{ wasm: ReturnType<typeof import('./wasm/loader').getWasm> }>();

  // 6-parameter family: [a, b, c, d, e, f]
  // Alpöge: a=1, b=1, c=1, d=3, e=3, f=2
  let p = $state({ a: 1.0, b: 1.0, c: 1.0, d: 3.0, e: 3.0, f: 2.0 });

  let score = $state({ mean: 0, std: 0, min: 0, max: 0, constancy: 0 });
  let collisionResult = $state('');
  let searching = $state(false);

  const paramNames = [
    { key: 'a', label: 'a · z(1+xy)³', def: 1 },
    { key: 'b', label: 'b · y²(1+xy)(4+3xy)', def: 1 },
    { key: 'c', label: 'c · y', def: 1 },
    { key: 'd', label: 'd · x(1+xy)²z', def: 3 },
    { key: 'e', label: 'e · xy²(4+3xy)', def: 3 },
    { key: 'f', label: 'f · x', def: 2 },
  ] as const;

  function runScore() {
    if (!wasm) return;
    const params = [p.a, p.b, p.c, p.d, p.e, p.f];
    setTimeout(() => {
      const result = wasm.counterexample_score(params, 100);
      score = {
        mean: result[0],
        std: result[1],
        min: result[2],
        max: result[3],
        constancy: 1.0 - Math.min(1.0, result[1] / (Math.abs(result[0]) + 0.001)),
      };
    }, 10);
  }

  function findCollision() {
    if (!wasm) return;
    searching = true;
    collisionResult = '';
    setTimeout(() => {
      const params = [p.a, p.b, p.c, p.d, p.e, p.f];
      const r = wasm.counterexample_find_collision(params);
      if (r.length > 0) {
        collisionResult = `COLLISION FOUND! dist=${r[6].toExponential(2)}\n`
          + `P₁ = (${r[0].toFixed(4)}, ${r[1].toFixed(4)}, ${r[2].toFixed(4)})\n`
          + `P₂ = (${r[3].toFixed(4)}, ${r[4].toFixed(4)}, ${r[5].toFixed(4)})`;
      } else {
        collisionResult = 'No collision found (tried 20 random starts)';
      }
      searching = false;
    }, 10);
  }

  function loadAlpoge() {
    p = { a: 1, b: 1, c: 1, d: 3, e: 3, f: 2 };
    setTimeout(runScore, 50);
  }

  function randomize() {
    p = {
      a: Math.round((Math.random() * 6 - 3) * 10) / 10,
      b: Math.round((Math.random() * 6 - 3) * 10) / 10,
      c: Math.round((Math.random() * 6 - 3) * 10) / 10,
      d: Math.round((Math.random() * 6 - 3) * 10) / 10,
      e: Math.round((Math.random() * 6 - 3) * 10) / 10,
      f: Math.round((Math.random() * 6 - 3) * 10) / 10,
    };
    setTimeout(runScore, 50);
  }

  onMount(runScore);
</script>

<div class="card card-demo p-6 space-y-4">

  <div class="flex gap-2 items-center flex-wrap">
    <h3 class="h3 flex-1">🔬 Map Parameters</h3>
    <button class="btn preset-tonal text-xs" onclick={loadAlpoge}>📋 Load Alpöge</button>
    <button class="btn preset-tonal text-xs" onclick={randomize}>🎲 Randomize</button>
  </div>

  <div class="grid grid-cols-2 md:grid-cols-3 gap-3">
    {#each paramNames as { key, label, def }}
      <label class="label">
        <span class="text-xs font-mono text-surface-400">{label}</span>
        <div class="flex gap-1 items-center">
          <input type="range" class="input flex-1"
            min={def - 5} max={def + 5} step="0.1"
            bind:value={p[key]} oninput={runScore} />
          <span class="text-xs font-mono w-10 text-right">{p[key].toFixed(1)}</span>
        </div>
      </label>
    {/each}
  </div>

  <div class="flex gap-3 flex-wrap items-center">
    <!-- Score display -->
    <div class="flex-1 bg-surface-800 p-3 rounded space-y-1">
      <div class="flex gap-2 items-baseline">
        <span class="text-xs text-surface-500">det(J<sub>F</sub>)</span>
        <span class="text-sm font-mono {score.std < 1e-6 ? 'text-green-400' : 'text-red-400'}">
          μ={score.mean.toFixed(4)} σ={score.std.toExponential(2)}
        </span>
      </div>
      <div class="flex gap-2 items-baseline">
        <span class="text-xs text-surface-500">range</span>
        <span class="text-xs font-mono text-surface-400">[{score.min.toFixed(4)}, {score.max.toFixed(4)}]</span>
      </div>
      <div class="mt-1 h-2 bg-surface-700 rounded overflow-hidden">
        <div class="h-full transition-all duration-300 rounded"
          style="width:{Math.min(100, score.constancy * 100)}%;
            background:{score.constancy > 0.999 ? '#22c55e' : score.constancy > 0.9 ? '#f59e0b' : '#ef4444'}">
        </div>
      </div>
      <div class="text-xs text-surface-500">
        {#if score.constancy > 0.9999}
          <span class="text-green-400">✅ CONSTANT! This is a candidate counterexample.</span>
        {:else if score.constancy > 0.99}
          <span class="text-amber-400">⚠ Nearly constant — try fine-tuning parameters.</span>
        {:else}
          <span class="text-red-400">❌ Determinant varies — not a counterexample.</span>
        {/if}
      </div>
    </div>

    <!-- Collision search -->
    <div class="w-64">
      <button class="btn preset-tonal-warning w-full text-xs" onclick={findCollision}
        disabled={searching || score.constancy < 0.99}>
        {searching ? '🔍 Searching...' : '🔍 Find Collision'}
      </button>
      {#if collisionResult}
        <pre class="mt-2 text-xs font-mono text-surface-300 whitespace-pre-wrap
          {collisionResult.startsWith('COLLISION') ? 'text-green-400' : 'text-surface-500'}">
          {collisionResult}</pre>
      {/if}
    </div>
  </div>

  <div class="bg-surface-800 p-3 rounded text-xs space-y-1 font-mono">
    <div class="text-surface-400">Map definition:</div>
    <div>F₁ = <span class="text-primary-300">{p.a.toFixed(1)}</span>·z(1+xy)³ + <span class="text-primary-300">{p.b.toFixed(1)}</span>·y²(1+xy)(4+3xy)</div>
    <div>F₂ = <span class="text-primary-300">{p.c.toFixed(1)}</span>·y + <span class="text-primary-300">{p.d.toFixed(1)}</span>·x(1+xy)²z + <span class="text-primary-300">{p.e.toFixed(1)}</span>·xy²(4+3xy)</div>
    <div>F₃ = <span class="text-primary-300">{p.f.toFixed(1)}</span>·x − 3x²y − x³z</div>
  </div>
</div>
