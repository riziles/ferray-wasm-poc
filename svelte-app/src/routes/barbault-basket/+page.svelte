<script lang="ts">
  import { getContext } from 'svelte';
  import BarbaultBasket from '$lib/BarbaultBasket.svelte';
  import type { WasmApi } from '$lib/wasm/loader';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());
</script>

<main>
  <div class="space-y-6">
    <div>
      <h1 class="text-3xl font-bold">🪐 Barbault Basket</h1>
      <p class="text-surface-400">
        The Barbault Cyclic Index measures how clustered the outer planets are on the ecliptic.
        Around July 20, 2026, four planets align at ~4° of their signs — the "Barbault Basket."
      </p>
    </div>

    <BarbaultBasket {wasm} />

    <div class="bg-surface-800/50 p-4 rounded space-y-2 text-sm">
      <h3 class="font-semibold">What is the Barbault Basket?</h3>
      <p>
        French astrologer <strong>André Barbault</strong> spent decades studying the geometry of the
        outer planets. He created the <strong>Cyclic Index</strong>: sum the 10 pair-wise angular distances
        between Jupiter through Pluto along the ecliptic. Low values = tight clustering; high values = dispersion.
      </p>
      <p>
        A <strong>"basket"</strong> happens when 4+ outer planets cluster in one hemisphere of the zodiac
        while the remaining planet (usually Saturn) sits apart like a handle. The July 2026 basket is special
        because Jupiter, Uranus, Neptune, and Pluto all occupy the <strong>4th degree</strong> of their
        respective signs — forming sextiles (60°) and trines (120°).
      </p>
      <div class="flex gap-4 flex-wrap mt-2">
        <div class="text-xs space-y-1">
          <div>☀️ <strong>July 20, 2026 basket:</strong></div>
          <div class="font-mono ml-4">
            Jupiter 4° Leo · Uranus 4° Gemini · Neptune 4° Aries · Pluto 4° Aquarius
          </div>
        </div>
      </div>
    </div>

    <div class="bg-surface-800/50 p-4 rounded space-y-2 text-sm">
      <h3 class="font-semibold">Computation approach</h3>
      <p>
        Positions use <strong>mean orbital elements</strong> from Meeus' <em>Astronomical Algorithms</em>
        (Chapter 31), solved via Kepler's equation with Newton's method.
      </p>
      <p class="text-xs text-surface-500">
        Accuracy: ~0.1° for outer planets over decades. For precise sky positions, see NASA JPL Horizons.
      </p>
    </div>
  </div>
</main>
