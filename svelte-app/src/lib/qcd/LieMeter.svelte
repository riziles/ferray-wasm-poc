<script lang="ts">
  // A per-act honesty badge. The page's whole argument is that every picture
  // below is a cartoon, and the cartoons get worse as the physics gets better.
  let {
    honesty,
    label = 'Picture honesty'
  }: { honesty: number; label?: string } = $props();

  const tint = $derived(
    honesty > 65 ? 'success' : honesty > 35 ? 'warning' : 'error'
  );
</script>

<div class="flex items-center gap-2">
  <span class="text-[10px] uppercase tracking-wider text-surface-500">{label}</span>
  <div class="h-1.5 w-24 rounded-full bg-surface-700 overflow-hidden">
    <div
      class="h-full rounded-full transition-all duration-500"
      style="width: {honesty}%; background: var(--color-{tint}-500)"
    ></div>
  </div>
  <span class="text-[11px] font-mono" style="color: var(--color-{tint}-400)">{honesty}%</span>
</div>
