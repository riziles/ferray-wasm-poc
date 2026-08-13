<script lang="ts">
  import { getContext } from 'svelte';
  import type { WasmApi } from '$lib/wasm/loader';
  import Wormhole from '$lib/Wormhole.svelte';

  const getWasm = getContext<() => WasmApi>('wasm');
  const wasm = $derived(getWasm());

  const ellis = 'r(z) = b₀ · cosh(z / b₀)          // Ellis 1973 — a catenoid';
  const flamm = 'z(r) = ± 2·√(rₛ·(r − rₛ))   ⇔   r(z) = rₛ + z² / (4·rₛ)   // Flamm 1916';

  const rustCode = `// Embedding surface: radius → height above the sheet plane.
// profile 0 = Ellis catenoid (traversable wormhole)
// profile 1 = Flamm paraboloid (Schwarzschild / Einstein–Rosen bridge)
fn profile_z(profile: u32, q: f64, r: f64) -> f64 {
    let r = r.max(q);                 // q = throat radius / sheet radius
    match profile {
        1 => 2.0 * (q * (r - q)).sqrt(),      // Flamm
        _ => q * acosh(r / q),                // Ellis
    }
}

// One software-3D frame: mesh the surface of revolution,
// yaw/pitch → perspective → painter sort → Lambert-shaded quads.
let n = norm3(cross3(sub3(v1, v0), sub3(v3, v0)));   // view-space normal
let shade = 0.42 + 0.58 * dot3(n, light).abs();       // two-sided Lambert
recs.sort_by(|a, b| b.depth.cmp(a.depth));            // far → near`;
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl space-y-8">

  <h1 class="h1">🌀 Wormhole Embedding Diagram</h1>
  <p class="text-lg text-surface-400">
    The iconic "two sheets joined by a throat" picture — as an actual 3D model.
    Drag it around, stretch the throat, and watch the classic construction weld itself together.
  </p>

  <Wormhole {wasm} />

  <section class="card card-demo p-6 space-y-4">

    <h2 class="h2">What is this surface?</h2>

    <p class="text-surface-300">
      General relativity describes space around a wormhole as curved — but curved
      <em>how</em>? An
      <a href="https://en.wikipedia.org/wiki/Embedding_diagram" target="_blank" class="text-primary-400 underline">embedding diagram</a>
      answers by taking a constant-time slice of the spacetime and bending it into ordinary 3D
      space so that distances <strong>on the surface</strong> match the metric's distances. The
      result is exactly the surface of revolution you're rotating above: far from the throat it
      flattens into two "sheets" (our two universes / distant regions), and near the throat it
      narrows to a minimum circle of radius <code class="code-block">b₀</code>.
    </p>

    <p class="text-surface-300">
      Two classic profiles are included:
    </p>
    <ul class="list-disc list-inside text-surface-300 space-y-1">
      <li>
        <strong>Ellis wormhole</strong> (Ellis 1973, Bronnikov 1973 — the first traversable
        wormhole solution): the slice is a <em>catenoid</em>,
        <code class="code-block">r(z) = b₀·cosh(z/b₀)</code>.
      </li>
      <li>
        <strong>Flamm's paraboloid</strong> (Flamm 1916 — the spatial slice of Schwarzschild,
        later re-popularized by Einstein &amp; Rosen's 1935 "bridge"):
        <code class="code-block">r(z) = rₛ + z²/(4rₛ)</code>.
      </li>
    </ul>

    <pre class="code-block p-4 overflow-x-auto text-xs"><code>{ellis}
{flamm}</code></pre>

    <h3 class="h3 mt-4">The construction slider</h3>
    <p class="text-surface-300">
      Topologically, the diagram is built the way the screenshot that inspired this demo shows:
      take two copies of the plane, cut a disk out of each, and <em>identify</em> (glue) the two
      boundary circles. The glued circle sweeps out the throat
      (<code class="code-block">S¹ × ℝ</code>). Drag the <strong>construction</strong> slider to
      0% to cut the sheets apart (glowing rims = the boundary circles), then back to 100% to weld
      them into one continuous manifold.
    </p>

    <h3 class="h3 mt-4">The flat annulus chart</h3>
    <p class="text-surface-300">
      The <strong>flat annulus chart</strong> view unrolls the same surface onto a plane:
      keep the angle θ, and replace height with the <em>arc length</em> s(r) measured along
      the meridian from the throat. Every meridian straightens into a radial line without
      stretching, so the entire manifold becomes one annulus — inner rim = the bottom sheet's
      edge, the middle circle = the throat, outer rim = the top sheet's edge. Colors and the
      traveler are carried through the same map, so you can watch the particle cross the
      throat circle in the flat picture exactly when it does so in 3D. The arc lengths have
      tidy closed forms:
    </p>
    <pre class="code-block p-4 overflow-x-auto text-xs"><code>Ellis:  s(r) = √(r² − b₀²)
Flamm:  s(r) = √(r·(r − rₛ)) + rₛ·ln((√r + √(r − rₛ)) / √rₛ)</code></pre>

    <h3 class="h3 mt-4">What it is <em>not</em></h3>
    <p class="text-surface-300">
      The funnel is <strong>not</strong> what a wormhole "looks like" — nothing sits "below" the
      sheets, and light doesn't travel along the surface. It is a picture of
      <em>spatial geometry</em> (how distances and volumes are distorted), not of optics. A real
      traveller crossing the Ellis throat feels no drama at <code class="code-block">b₀</code>;
      the orange particle in the demo follows the surface the way the diagram intends. For the
      Schwarzschild bridge, note the slice is only valid outside the horizon — the Einstein–Rosen
      bridge is not traversable.
    </p>

    <h3 class="h3 mt-4">Implementation notes</h3>
    <ul class="list-disc list-inside text-surface-300 space-y-1">
      <li>
        Everything is computed in Rust (WASM) every frame: a ~4k-quad surface of revolution,
        yaw/pitch rotation, perspective projection, painter's-algorithm depth sort, and two-sided
        Lambert shading. The draw list crosses the WASM boundary as one flat
        <code class="code-block">Vec&lt;f64&gt;</code>; Svelte just replays it on canvas 2D.
      </li>
      <li>
        Ring spacing is clustered toward the throat (<code class="code-block">s^1.4</code>) where
        the catenoid's curvature concentrates, so low mesh counts still look smooth.
      </li>
      <li>
        Pure scalar Rust — no ferray crates, no WebGL, no three.js. The whole demo is a few
        hundred lines and runs in a couple of milliseconds per frame.
      </li>
    </ul>

    <h3 class="h3 mt-4">Rust implementation (core)</h3>
    <pre class="code-block p-4 overflow-x-auto text-xs"><code>{rustCode}</code></pre>

    <h3 class="h3 mt-4">Further reading</h3>
    <ul class="list-disc list-inside text-surface-400 text-sm space-y-1">
      <li>Flamm, L. (1916) — "Beiträge zur Einsteinschen Gravitationstheorie", the original paraboloid</li>
      <li>Ellis, H. (1973) — <a href="https://doi.org/10.1063/1.1666161" target="_blank" class="text-primary-400 underline">J. Math. Phys. 14, 104</a>; Bronnikov (1973)</li>
      <li>Morris &amp; Thorne (1988) — <a href="https://doi.org/10.1119/1.15620" target="_blank" class="text-primary-400 underline">"Wormholes in spacetime and their use for interstellar travel"</a></li>
      <li>Wikipedia: <a href="https://en.wikipedia.org/wiki/Wormhole" target="_blank" class="text-primary-400 underline">Wormhole</a>, <a href="https://en.wikipedia.org/wiki/Embedding_diagram" target="_blank" class="text-primary-400 underline">Embedding diagram</a></li>
    </ul>
  </section>

</div>
