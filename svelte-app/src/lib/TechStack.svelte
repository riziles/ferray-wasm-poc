<script lang="ts">
  const crates = [
    { name: 'ferray-core', size: '155 KB', desc: 'Multi-dimensional arrays, layout, indexing', ok: true },
    { name: 'ferray-ufunc', size: '142 KB', desc: 'Trig, exp/log, arithmetic (libm on WASM)', ok: true },
    { name: 'ferray-stats', size: '68 KB', desc: 'min/max/mean/median/std', ok: true },
    { name: 'ferray-fft', size: '89 KB', desc: 'Real FFT, magnitude spectrum', ok: true },
    { name: 'ferray-window', size: '47 KB', desc: 'Hanning, Hamming, Blackman, etc.', ok: true },
    { name: 'ferray-linalg', size: '—', desc: 'Matrix ops (available, not yet demoed)', ok: true },
    { name: 'ferray-random', size: '—', desc: 'Distributions (available, not yet demoed)', ok: true },
    { name: 'ferray-io', size: '—', desc: 'CSV/Parquet (blocked: zstd-sys)', ok: false },
    { name: 'ferray-python', size: '—', desc: 'Python bindings (blocked: pyo3)', ok: false },
  ];
</script>

<div class="card card-demo p-6 space-y-4">
  <h2 class="h2">🧬 Ferray Workspace Crate Matrix</h2>
  <p class="text-sm text-surface-400">
    Which ferray workspace crates compile to WASM. 7 of 9 compile; only <code class="code-block">ferray-io</code>
    (zstd-sys) and <code class="code-block">ferray-python</code> (pyo3) are blocked by native-C dependencies.
  </p>

  <div class="overflow-x-auto">
    <table class="table table-hover">
      <thead>
        <tr>
          <th>Crate</th>
          <th>Est. Size</th>
          <th>Description</th>
          <th>WASM</th>
        </tr>
      </thead>
      <tbody>
        {#each crates as c}
          <tr>
            <td><code class="code-block">{c.name}</code></td>
            <td class="text-xs text-surface-400">{c.size}</td>
            <td class="text-xs text-surface-300">{c.desc}</td>
            <td>
              {#if c.ok}
                <span class="badge preset-tonal-success">✅</span>
              {:else}
                <span class="badge preset-tonal-error">❌</span>
              {/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
