<script lang="ts">
  import { onMount } from 'svelte';

  let { wasm } = $props<{ wasm: ReturnType<typeof import('./wasm/loader').getWasm> }>();

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
      score = { mean: result[0], std: result[1], min: result[2], max: result[3],
        constancy: 1.0 - Math.min(1.0, result[1] / (Math.abs(result[0]) + 0.001)) };
    }, 10);
  }

  function findCollision() {
    if (!wasm) return;
    searching = true; collisionResult = '';
    setTimeout(() => {
      const params = [p.a, p.b, p.c, p.d, p.e, p.f];
      const r = wasm.counterexample_find_collision(params);
      if (r.length > 0) collisionResult = `COLLISION FOUND! dist=${r[6].toExponential(2)}\nP₁ = (${r[0].toFixed(4)}, ${r[1].toFixed(4)}, ${r[2].toFixed(4)})\nP₂ = (${r[3].toFixed(4)}, ${r[4].toFixed(4)}, ${r[5].toFixed(4)})`;
      else collisionResult = 'No collision found (30 Newton seeds)';
      searching = false;
    }, 10);
  }

  function loadAlpoge() { p = { a: 1, b: 1, c: 1, d: 3, e: 3, f: 2 }; setTimeout(runScore, 50); }
  function randomize() {
    const r = () => Math.round((Math.random() * 6 - 3) * 10) / 10;
    p = { a: r(), b: r(), c: r(), d: r(), e: r(), f: r() }; setTimeout(runScore, 50);
  }

  // Auto-search
  let autoSearching = $state(false); let autoIter = $state(0); let autoBest = $state(0); let autoCancel: (() => void) | null = null;
  function startAutoSearch() {
    autoSearching = true; autoIter = 0; autoBest = score.constancy;
    let cancelled = false; autoCancel = () => { cancelled = true; };
    const keys = ['a','b','c','d','e','f'] as const;
    function step() {
      if (cancelled || !autoSearching) return; autoIter++;
      const key = keys[Math.floor(Math.random() * keys.length)], oldVal = p[key];
      p[key] = Math.round((oldVal + (Math.random() - 0.5) * 0.2) * 10) / 10;
      const r = wasm!.counterexample_score([p.a,p.b,p.c,p.d,p.e,p.f], 20);
      const nc = 1 - Math.min(1, r[1] / (Math.abs(r[0]) + 0.001));
      if (nc > score.constancy + 0.001) { score = { mean: r[0], std: r[1], min: r[2], max: r[3], constancy: nc }; if (nc > autoBest) autoBest = nc; }
      else p[key] = oldVal;
      if (autoIter % 50 === 0 && score.constancy < 0.99) { const rr = () => Math.round((Math.random()*6-3)*10)/10; p = { a:rr(),b:rr(),c:rr(),d:rr(),e:rr(),f:rr() }; }
      if (score.constancy >= 0.9999) { autoSearching = false; setTimeout(() => runScore(), 10); return; }
      setTimeout(step, 0);
    }
    setTimeout(step, 0);
  }
  function stopAutoSearch() { autoSearching = false; autoCancel?.(); }

  // Custom expression mode
  let customMode = $state(false);
  let customF1 = $state('z*(1+x*y)^3 + y^2*(1+x*y)*(4+3*x*y)');
  let customF2 = $state('y + 3*x*(1+x*y)^2*z + 3*x*y^2*(4+3*x*y)');
  let customF3 = $state('2*x - 3*x^2*y - x^3*z');
  let parseError = $state('');

  function runCustomScore() {
    if (!wasm) return; parseError = '';
    try {
      const r = wasm.custom_score(customF1, customF2, customF3, 100);
      score = { mean: r[0], std: r[1], min: r[2], max: r[3],
        constancy: 1 - Math.min(1, r[1] / (Math.abs(r[0]) + 0.001)) };
    } catch (e: any) { parseError = String(e); }
  }

  onMount(runScore);
</script>

<div class="card card-demo p-6 space-y-4">

  <div class="flex gap-2 items-center flex-wrap">
    <h3 class="h3 flex-1">🔬 Map Parameters</h3>
    <div class="flex rounded border border-surface-600 overflow-hidden">
      <button class="px-3 py-1 text-xs {customMode ? 'bg-surface-800 text-surface-400' : 'bg-primary-700 text-white'}"
        onclick={() => customMode = false}>Parameter Family</button>
      <button class="px-3 py-1 text-xs {customMode ? 'bg-primary-700 text-white' : 'bg-surface-800 text-surface-400'}"
        onclick={() => customMode = true}>Custom Expression</button>
    </div>
  </div>

  {#if !customMode}
    <div class="flex gap-2 items-center flex-wrap">
      <button class="btn preset-tonal text-xs" onclick={loadAlpoge}>📋 Load Alpöge</button>
      <button class="btn preset-tonal text-xs" onclick={randomize}>🎲 Randomize</button>
      {#if autoSearching}
        <button class="btn preset-filled-warning text-xs animate-pulse" onclick={stopAutoSearch}>⏹ Stop ({autoIter})</button>
      {:else}
        <button class="btn preset-tonal text-xs" onclick={startAutoSearch}>🤖 Auto-Search</button>
      {/if}
    </div>
    {#if autoSearching || autoBest > 0}
      <div class="text-xs text-surface-500">
        {autoSearching ? '🤖 Searching…' : '✅ Search stopped'} · iteration {autoIter} · Best: <span class="font-mono text-primary-400">{(autoBest * 100).toFixed(2)}%</span>
      </div>
    {/if}
    <div class="grid grid-cols-2 md:grid-cols-3 gap-3">
      {#each paramNames as { key, label, def }}
        <label class="label">
          <span class="text-xs font-mono text-surface-400">{label}</span>
          <div class="flex gap-1 items-center">
            <input type="range" class="input flex-1" min={def - 5} max={def + 5} step="0.1" bind:value={p[key]} oninput={runScore} />
            <span class="text-xs font-mono w-10 text-right">{p[key].toFixed(1)}</span>
          </div>
        </label>
      {/each}
    </div>
    <div class="bg-surface-800 p-3 rounded text-xs space-y-1 font-mono">
      <div class="text-surface-400">Map definition:</div>
      <div>F₁ = <span class="text-primary-300">{p.a.toFixed(1)}</span>·z(1+xy)³ + <span class="text-primary-300">{p.b.toFixed(1)}</span>·y²(1+xy)(4+3xy)</div>
      <div>F₂ = <span class="text-primary-300">{p.c.toFixed(1)}</span>·y + <span class="text-primary-300">{p.d.toFixed(1)}</span>·x(1+xy)²z + <span class="text-primary-300">{p.e.toFixed(1)}</span>·xy²(4+3xy)</div>
      <div>F₃ = <span class="text-primary-300">{p.f.toFixed(1)}</span>·x − 3x²y − x³z</div>
    </div>
  {:else}
    <div class="space-y-3">
      {#each [{ v: customF1, set: (s: string) => customF1 = s, label: 'F₁' }, { v: customF2, set: (s: string) => customF2 = s, label: 'F₂' }, { v: customF3, set: (s: string) => customF3 = s, label: 'F₃' }] as field}
        <label class="label">
          <span class="text-xs font-mono text-surface-400">{field.label}(x, y, z) =</span>
          <input type="text" class="input font-mono text-xs" value={field.v} oninput={e => field.set(e.currentTarget.value)} />
        </label>
      {/each}
      <div class="flex gap-2 items-center">
        <button class="btn preset-filled-primary text-xs" onclick={runCustomScore}>Parse & Score</button>
        <button class="btn preset-tonal text-xs" onclick={() => { customF1='z*(1+x*y)^3 + y^2*(1+x*y)*(4+3*x*y)'; customF2='y + 3*x*(1+x*y)^2*z + 3*x*y^2*(4+3*x*y)'; customF3='2*x - 3*x^2*y - x^3*z'; setTimeout(runCustomScore, 50); }}>📋 Load Alpöge</button>
      </div>
      {#if parseError}
        <div class="bg-red-900/30 border border-red-800/40 p-2 rounded text-xs font-mono text-red-400">{parseError}</div>
      {/if}
      <div class="text-xs text-surface-500">
        Variables: <code class="code-block">x y z</code> · Operators: <code class="code-block">+ - * / ^</code> · Integer powers only
      </div>
    </div>
  {/if}

  <!-- Shared score display + collision search (works in both modes) -->
  <div class="flex gap-3 flex-wrap items-center">
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

    <div class="w-64">
      <button class="btn preset-tonal-warning w-full text-xs" onclick={findCollision}
        disabled={searching || score.constancy < 0.99}
        title="Search for two distinct points that map to the same output">
        {searching ? '🔍 Searching...' : '🔍 Find Collision'}
      </button>
      {#if collisionResult}
        <pre class="mt-2 text-xs font-mono text-surface-300 whitespace-pre-wrap
          {collisionResult.startsWith('COLLISION') ? 'text-green-400' : 'text-surface-500'}">
          {collisionResult}</pre>
        {#if collisionResult.startsWith('No collision')}
          <span class="text-xs text-surface-500 mt-1 block">Newton's method from 30 seeds — not exhaustive.</span>
        {/if}
      {/if}
    </div>
  </div>
</div>
