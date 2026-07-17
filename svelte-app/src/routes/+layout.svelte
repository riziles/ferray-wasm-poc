<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import { page } from '$app/state';
  import { base } from '$app/paths';
  import { initWasm, type WasmApi } from '$lib/wasm/loader';
  import '../app.css';

  let { children } = $props<{ children: import('svelte').Snippet }>();

  let wasm = $state<WasmApi | null>(null);
  let error = $state<string | null>(null);

  setContext('wasm', () => wasm);

  onMount(async () => {
    try {
      wasm = await initWasm();
    } catch (e) {
      error = String(e);
    }
  });
</script>

{#if error}
  <main class="min-h-screen flex items-center justify-center bg-surface-900 p-8">
    <div class="card preset-filled-error p-8 max-w-md text-center">
      <h1 class="h1 mb-2">⚠️ WASM Init Failed</h1>
      <p class="text-xs font-mono opacity-80 break-all">{error}</p>
    </div>
  </main>
{:else if !wasm}
  <main class="min-h-screen flex items-center justify-center bg-surface-900 p-8">
    <div class="flex flex-col items-center gap-4">
      <div class="spinner"></div>
      <p class="text-lg text-surface-400">Loading Ferray WASM…</p>
    </div>
  </main>
{:else}
  <main class="min-h-screen bg-surface-900">
    {#if page.url.pathname !== base + '/' && page.url.pathname !== base}}
      <nav class="border-b border-surface-700 bg-surface-950/50 backdrop-blur sticky top-0 z-10">
        <div class="container mx-auto px-4 py-3 flex items-center gap-4">
          <a href="{base}/" class="text-sm text-surface-400 hover:text-primary-400 transition-colors">
            ← Ferray × WASM
          </a>
        </div>
      </nav>
    {/if}
    {@render children()}
  </main>
{/if}
