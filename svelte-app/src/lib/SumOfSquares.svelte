<script lang="ts">
  let { wasm } = $props<{ wasm: ReturnType<typeof import('./wasm/loader').getWasm> }>();

  let input = $state('1, 2, 3, 4, 5');
  let result = $state('');

  function compute() {
    try {
      const arr = input.split(',').map(s => parseFloat(s.trim()));
      if (arr.some(isNaN)) { result = 'Invalid input'; return; }
      const sum = wasm.sum_of_squares(new Float64Array(arr));
      result = String(sum);
    } catch (e) {
      result = 'Error: ' + String(e);
    }
  }
</script>

<div class="card card-demo p-6 space-y-4">

  <div class="flex gap-3 items-end">
    <label class="label flex-1">
      <span>Values (comma-separated)</span>
      <input type="text" class="input" bind:value={input} />
    </label>
    <button class="btn preset-filled" onclick={compute}>Compute</button>
  </div>

  {#if result}
    <div class="text-surface-200">
      Σ x² = <strong class="text-primary-400">{result}</strong>
    </div>
  {/if}
</div>
