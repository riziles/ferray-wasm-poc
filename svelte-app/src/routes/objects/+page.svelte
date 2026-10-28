<script lang="ts">
	import Folio from '$lib/objects/Folio.svelte';

	const rustNote = `// The shapes are parametric surfaces: f(u, v) → (x, y, z).
// Nothing here is pre-baked — every vertex is evaluated on load,
// and the normals come from the same grid.

const klein = (u, v, out) => {
  const U = u * TAU, V = v * TAU;
  const r = 2 + Math.cos(U / 2) * Math.sin(V) - Math.sin(U / 2) * Math.sin(2 * V);
  out.set(r * Math.cos(U), r * Math.sin(U),
          Math.sin(U / 2) * Math.sin(V) + Math.cos(U / 2) * Math.sin(2 * V));
};

parametric(klein, 120, 48);   // 121 × 49 rings → 11 520 quads`;

	const sceneNote = `<!-- FolioScene.svelte — the whole scene graph, declaratively -->
<T.PerspectiveCamera makeDefault position={[0, 0.8, 13.2]} fov={40} />
<T.AmbientLight intensity={0.45} />
<T.DirectionalLight position={[5, 7, 9]} intensity={2.2} />

{#each layout as item (item.def.id)}
  <FolioItem def={item.def} x={item.x} y={item.y} {material} {spin} {float} />
{/each}`;
</script>

<div class="container mx-auto px-4 py-8 max-w-5xl space-y-8">

	<h1 class="h1">🧊 Object Folio</h1>
	<p class="text-lg text-surface-400">
		Eight interactive Three.js objects in one canvas — knots, surfaces and solids you can
		orbit, spin, strip down to wireframe, and inspect. Built with
		<a href="https://threlte.xyz" target="_blank" class="text-primary-400 underline">Threlte</a>,
		which turns the Three.js scene graph into Svelte components.
	</p>

	<Folio />

	<section class="card card-demo p-6 space-y-4">

		<h2 class="h2">What is in the folio?</h2>

		<div class="overflow-x-auto">
			<table class="w-full text-sm text-surface-300">
				<thead class="text-surface-400 text-xs uppercase tracking-wide">
					<tr class="border-b border-surface-700">
						<th class="text-left py-2 pr-3">Object</th>
						<th class="text-left py-2 pr-3">Built from</th>
						<th class="text-left py-2">The interesting bit</th>
					</tr>
				</thead>
				<tbody class="align-top">
					<tr class="border-b border-surface-800">
						<td class="py-2 pr-3 whitespace-nowrap"><strong>Torus knot (2, 3)</strong></td>
						<td class="py-2 pr-3"><code class="code-block">TorusKnotGeometry</code></td>
						<td class="py-2">
							The curve wraps twice around the torus axis and three times through the
							hole; the tube merely gives it thickness. Change <em>p</em> or <em>q</em>
							and you get a different knot.
						</td>
					</tr>
					<tr class="border-b border-surface-800">
						<td class="py-2 pr-3 whitespace-nowrap"><strong>Icosahedron</strong></td>
						<td class="py-2 pr-3"><code class="code-block">IcosahedronGeometry</code></td>
						<td class="py-2">
							One of the five Platonic solids. It is drawn with flat shading on purpose:
							the facets are the point.
						</td>
					</tr>
					<tr class="border-b border-surface-800">
						<td class="py-2 pr-3 whitespace-nowrap"><strong>Klein bottle</strong></td>
						<td class="py-2 pr-3"><code class="code-block">parametric</code></td>
						<td class="py-2">
							A closed surface with neither inside nor outside. In three dimensions it
							must pass through itself — the crossing you can see is the price of
							drawing a 4D object in 3D.
						</td>
					</tr>
					<tr class="border-b border-surface-800">
						<td class="py-2 pr-3 whitespace-nowrap"><strong>Möbius strip</strong></td>
						<td class="py-2 pr-3"><code class="code-block">parametric</code></td>
						<td class="py-2">
							One side, one edge. Because the material renders double-sided, you can
							watch a single face pass from "front" to "back" without ever crossing an
							edge.
						</td>
					</tr>
					<tr class="border-b border-surface-800">
						<td class="py-2 pr-3 whitespace-nowrap"><strong>Spherical harmonic</strong></td>
						<td class="py-2 pr-3"><code class="code-block">parametric</code></td>
						<td class="py-2">
							The radius is a product of powers of sines and cosines, so the sphere grows
							lobes. These are the same functions that shape atomic orbitals.
						</td>
					</tr>
					<tr class="border-b border-surface-800">
						<td class="py-2 pr-3 whitespace-nowrap"><strong>Torus</strong></td>
						<td class="py-2 pr-3"><code class="code-block">TorusGeometry</code></td>
						<td class="py-2">
							Gaussian curvature is exactly zero everywhere. It is also the shape that
							makes the torus knot possible.
						</td>
					</tr>
					<tr class="border-b border-surface-800">
						<td class="py-2 pr-3 whitespace-nowrap"><strong>Helical tube</strong></td>
						<td class="py-2 pr-3"><code class="code-block">TubeGeometry</code></td>
						<td class="py-2">
							The helix is the canonical curve of constant curvature <em>and</em>
							constant torsion — a spring, a screw thread, a strand of DNA.
						</td>
					</tr>
					<tr>
						<td class="py-2 pr-3 whitespace-nowrap"><strong>Superformula star</strong></td>
						<td class="py-2 pr-3"><code class="code-block">parametric</code></td>
						<td class="py-2">
							Gielis' superformula: two radial functions (longitude and latitude)
							multiplied together. A handful of exponents covers everything from
							starfish to rounded cube.
						</td>
					</tr>
				</tbody>
			</table>
		</div>

		<h3 class="h3 mt-4">Parametric geometry from scratch</h3>
		<p class="text-surface-300">
			Half of the folio is not a built-in Three.js primitive — it is a function
			<code class="code-block">f(u, v) → (x, y, z)</code> sampled on a grid, stitched into
			quads, and given normals by averaging the adjacent faces. The whole builder is about
			thirty lines:
		</p>
		<pre class="code-block p-4 overflow-x-auto text-xs"><code>{rustNote}</code></pre>

		<h3 class="h3 mt-4">Why Threlte?</h3>
		<p class="text-surface-300">
			Plain Three.js means an imperative scene graph: create the scene, create meshes, add
			them, remember to remove them, remember to dispose their geometries. Threlte maps each
			Three.js class onto a component, so the scene graph is just markup — a
			<code class="code-block">&lt;T.Mesh&gt;</code> is a mesh, its children are its geometry
			and material, and unmounting a component disposes the object:
		</p>
		<pre class="code-block p-4 overflow-x-auto text-xs"><code>{sceneNote}</code></pre>
		<p class="text-surface-300">
			Two things stay imperative because they are not scene-graph objects:
			<code class="code-block">OrbitControls</code> (it owns the camera, so it is constructed
			against the canvas element and ticked from <code class="code-block">useTask</code>), and
			the <code class="code-block">PMREMGenerator</code> pass that turns
			<code class="code-block">RoomEnvironment</code> into the reflection map behind the
			chrome preset.
		</p>

		<h3 class="h3 mt-4">Controls, honestly</h3>
		<ul class="list-disc list-inside text-surface-300 space-y-1">
			<li><strong>Drag</strong> to orbit, <strong>scroll</strong> to zoom — panning is disabled so you cannot lose the folio.</li>
			<li><strong>Click</strong> an object to inspect it: it grows, spins faster, and the rest of the folio steps back.</li>
			<li><strong>Material</strong> swaps the shader: matte standard, polished metal, raw normals (great for spotting the UV seam), or wireframe.</li>
			<li><strong>Spread</strong> pulls the folio apart — useful for the self-intersecting surfaces, where the objects overlap themselves.</li>
			<li>
				The layout is responsive: four columns by two on a wide canvas, two by four on a
				phone. Positions are eased, so resizing the window glides instead of snapping.
			</li>
			<li>
				Nothing here touches the WASM module — this page is pure TypeScript, so it loads
				even if the Rust build is missing.
			</li>
		</ul>

		<h3 class="h3 mt-4">Further reading</h3>
		<ul class="list-disc list-inside text-surface-400 text-sm space-y-1">
			<li>
				<a href="https://threejs.org/docs/" target="_blank" class="text-primary-400 underline">Three.js documentation</a>
				— geometry, materials, and the
				<a href="https://threejs.org/docs/#api/en/geometries/BufferGeometry" target="_blank" class="text-primary-400 underline">BufferGeometry</a> shape used here
			</li>
			<li>
				<a href="https://threlte.xyz/docs" target="_blank" class="text-primary-400 underline">Threlte documentation</a>
				— <code class="code-block">&lt;T&gt;</code>, <code class="code-block">&lt;Canvas&gt;</code>,
				<code class="code-block">useTask</code>
			</li>
			<li>
				Gielis, J. (2003) —
				<a href="https://doi.org/10.1111/1467-8659.00236" target="_blank" class="text-primary-400 underline">"A generic geometric transformation that unifies a wide range of natural and abstract shapes"</a>
			</li>
			<li>
				Wikipedia:
				<a href="https://en.wikipedia.org/wiki/Klein_bottle" target="_blank" class="text-primary-400 underline">Klein bottle</a>,
				<a href="https://en.wikipedia.org/wiki/M%C3%B6bius_strip" target="_blank" class="text-primary-400 underline">Möbius strip</a>,
				<a href="https://en.wikipedia.org/wiki/Torus_knot" target="_blank" class="text-primary-400 underline">torus knot</a>
			</li>
		</ul>
	</section>
</div>
