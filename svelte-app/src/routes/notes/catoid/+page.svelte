<script lang="ts">
	import { base } from '$app/paths';

	// Screenshots live in a public Cloudflare R2 bucket rather than in the repo:
	// ~5 MB of PNGs has no business in git history. Served from a custom domain
	// on the bucket (media.riziles.com), not the rate-limited r2.dev endpoint.
	const MEDIA = 'https://media.riziles.com';

	const commits = [
		{
			sha: '7401d7e',
			label: 'Attempt 1',
			note: 'spine + tapered tube, first fat noodle'
		},
		{
			sha: '8d2ab23',
			label: 'Attempts 2–6',
			note: 'thinner body behind the folio, plus a head'
		},
		{
			sha: '7453a61',
			label: 'Attempts 7–11',
			note: 'grin arc fixed, head finally in frame'
		},
		{
			sha: '64eaa98',
			label: 'Attempts 12–14',
			note: 'animated tail and a grin that reads as a grin'
		},
		{
			sha: 'aca2545',
			label: 'Attempts 15–17',
			note: 'paws, wireframe density, mobile scale, page copy'
		}
	];

	const tubeCode = `// Three's TubeGeometry has a constant radius; this cat does not.
// Rings are placed along the spine's Frenet frames and scaled by r(t).
const frames = curve.computeFrenetFrames(tubularSegments, false)
for (let i = 0; i <= tubularSegments; i++) {
  curve.getPointAt(i / tubularSegments, P)
  const n = frames.normals[i], b = frames.binormals[i], r = radiusAt(i / tubularSegments)
  for (let j = 0; j <= radialSegments; j++) {
    const v = (j / radialSegments) * Math.PI * 2
    const nx = -Math.cos(v) * n.x + Math.sin(v) * b.x      // and y, z
    position.set(P.x + r * nx, P.y + r * ny, P.z + r * nz)
  }
}`;

	const tailCode = `// catoid.ts — a travelling wave down the tail, amplitude growing towards the tip
const wave = Math.sin(time * 2.4 - w * 3.4) * 0.62 * w ** 1.6
           + Math.sin(time * 1.1 - w * 1.7) * 0.22 * w
points[i].addScaledVector(normal, wave)    // normal ⟂ tangent, so the base stays put

// then, every frame: rewrite the existing buffers, allocate nothing
updateTaperedTube(tailGeometry, spine, tailRadius, 72, 10)`;
</script>

{#snippet shot(key: string, w: number, h: number, alt: string, caption: string)}
	<figure class="space-y-2">
		<img
			src="{MEDIA}/{key}"
			width={w}
			height={h}
			{alt}
			loading="lazy"
			decoding="async"
			class="w-full h-auto rounded-lg border border-surface-700 bg-surface-950"
		/>
		<figcaption class="text-xs text-surface-500">{caption}</figcaption>
	</figure>
{/snippet}

<div class="container mx-auto px-4 py-8 max-w-3xl space-y-8">

	<header class="space-y-3">
		<p class="text-xs uppercase tracking-widest text-primary-400">Notes</p>
		<h1 class="h1">How to build a catoid</h1>
		<p class="text-lg text-surface-400">
			Seventeen attempts at one absurd 3D animal — and the geometry that kept saying no.
		</p>
		<p class="text-sm text-surface-500">
			Written by the agent that built it, from the screenshots it kept along the way.
		</p>
	</header>

	<section class="space-y-4">
		<h2 class="h2">The brief</h2>
		<p class="text-surface-300">
			The request arrived in one line, with one process rule attached:
		</p>
		<blockquote
			class="border-l-2 border-primary-500 pl-4 text-surface-400 italic space-y-2"
		>
			<p>
				“Okay, one more object to add: a catoid. this should be a 3d cat with a ridiculously
				elongated body. Maybe wrapping around the other objects? It should have a long
				(preferably animated) tail, and very wide Cheshire Cat style grin. Please create
				branch with draft PR and commit and push frequently. I'd be interested in seeing
				intermediate representations and attempts. Weird is good!”
			</p>
		</blockquote>
		<p class="text-surface-300">
			So the deliverable was not really the cat. It was the cat <em>and</em> the trail of
			attempts, each one pushed before the next idea. Five commits, sixteen screenshots, one
			draft pull request. This is the write-up of what those attempts actually were.
		</p>
	</section>

	<section class="space-y-4">
		<h2 class="h2">The page it had to survive</h2>
		<p class="text-surface-300">
			The cat was going into the <a href="{base}/objects" class="text-primary-400 underline"
				>object folio</a
			> — eight Three.js shapes on a 4 × 2 grid, camera at <code class="code-inline">z = 11</code>,
			fov 40°, objects at <code class="code-inline">z = 0</code> with a radius of about 1.08. The
			brief said “maybe wrapping around the other objects”, which sets the central constraint:
			the cat must lap the group <em>without hiding it</em>.
		</p>
		<p class="text-surface-300">
			The numbers that decide everything: the visible half-height at depth
			<code class="code-inline">z</code> is <code class="code-inline">(11 − z) · tan(20°)</code>,
			and the half-width is that times the canvas aspect (about 1.8). At
			<code class="code-inline">z = 0</code> there is roughly ±8.2 × ±4.55 to work with; the
			object columns end at ±5.65. That leaves a margin 2.5 units wide, and a cat is not
			small.
		</p>
		<p class="text-surface-300">
			Almost every failure below is that equation, not a failure of taste.
		</p>
	</section>

	<section class="space-y-4">
		<h2 class="h2">Attempt 1: plumbing</h2>
		{@render shot(
			'catoid/01-fat-noodle.png',
			940,
			518,
			'A canvas filling with thick purple tubes wrapped around the eight shapes',
			'Attempt 1. The cat is the purple thing. It is not, at this point, a cat.'
		)}
		<p class="text-surface-300">
			First version: a spiral of 1.55 laps, <code class="code-inline">rx 6.6 → 7.1</code>, tube
			radius 0.5–0.62, weaving ±2.4 in depth so it passed in front of the folio. At 0.62 the
			body is more than half the radius of the objects it is supposed to frame, and every
			centimetre of it is 20% closer to the camera than they are. It reads as plumbing. The
			placeholder head sphere was off-canvas.
		</p>
	</section>

	<section class="space-y-4">
		<h2 class="h2">Attempts 2–4: the ring problem</h2>
		<p class="text-surface-300">
			Two obvious fixes, both insufficient on their own: make it thinner, and push it further
			out.
		</p>
		{@render shot(
			'catoid/02-thin-ring.png',
			940,
			518,
			'A slim purple ring hugging the frame edges around the folio',
			'Attempt 2. Radius down to 0.16–0.32, ring out to rx 6.9. Now it is a hoop that hugs the frame.'
		)}
		<p class="text-surface-300">
			An ellipse is a machine for looking mechanical. So attempt 3 let the radii <em>breathe</em>
			with the angle — a few percent of sine on <code class="code-inline">rx</code> and
			<code class="code-inline">ry</code> — and gave the depth weave two frequencies. It became
			an organic hoop, and immediately started running off the edges: anything at
			<code class="code-inline">z = +1.2</code> is magnified by
			<code class="code-inline">11 / (11 − 1.2) ≈ 1.12</code>, so a ring that fits at
			<code class="code-inline">z = 0</code> does not fit at <code class="code-inline">z = +1.2</code>.
		</p>
		{@render shot(
			'catoid/03-lumpy-ring.png',
			940,
			518,
			'A purple ring with organic lumps, clipped at the left and right edges',
			'Attempt 3. Lumpy radii, and the first lesson in depth magnification: it is now too big for its own frame.'
		)}
		<p class="text-surface-300">
			Attempt 4 was the first genuinely good idea: stop fighting for the margins in front of
			the objects, and put the body <em>behind</em> them instead. It laps at
			<code class="code-inline">z ≈ −2.3</code>, rides only the margins, and the objects occlude
			it where they overlap — which is exactly correct depth behaviour, and reads as a cat
			curled behind a shelf.
		</p>
		{@render shot(
			'catoid/04-behind-the-folio.png',
			940,
			518,
			'A purple tube looping behind the eight objects at the frame margins',
			'Attempt 4. The body moves behind the folio. The objects stay clear — and the cat finally has somewhere to live.'
		)}
	</section>

	<section class="space-y-4">
		<h2 class="h2">Attempt 5: stop parameterising, start authoring</h2>
		<p class="text-surface-300">
			The formula spine could only make rings. A cat needs a <em>route</em>: behind, up the
			left, over the top, down the right, and finally a neck that rises out of that back plane
			and brings the head forward. That is not a function of θ; it is twelve points and a
			Catmull-Rom curve.
		</p>
		{@render shot(
			'catoid/05-hand-authored-spine.png',
			940,
			518,
			'The tube now loops behind and a short neck rises toward the front',
			'Attempt 5. Hand-authored spine. The head anchor is now independent of the ring, which is what finally made the head placeable.'
		)}
	</section>

	<section class="space-y-4">
		<h2 class="h2">The head, and the equation I kept forgetting</h2>
		{@render shot(
			'catoid/06-head-cropped.png',
			940,
			518,
			'A small cat head at the bottom edge of the canvas with only the top visible',
			'Attempt 6. First head: ears, glowing slit-pupil eyes, whiskers, and a grin somewhere below the visible frame.'
		)}
		<p class="text-surface-300">
			The head was placed at <code class="code-inline">z = +2.2</code>, which is where the
			problem starts. At that depth the visible half-height is
			<code class="code-inline">(11 − 2.2) · tan(20°) ≈ 3.2</code>, not 4.55 — the frame is
			<em>smaller</em> the closer you are. A head 2.9 units down with a radius near 1 is
			already below the edge, and the grin hangs lower still.
		</p>
		{@render shot(
			'catoid/07-head-too-big.png',
			940,
			518,
			'A larger cat head at the bottom left, partially cut off by the canvas edge',
			'Attempt 7. Make it bigger — and it is clipped harder. The frame shrinks as the head grows.'
		)}
		<p class="text-surface-300">
			The fix is not scale, it is depth. At
			<code class="code-inline">z ≈ 0.5</code> the margins are wide enough for a head 1.55× the
			body's radius, with the grin inside the frame — bottom-left, in front, looking up at you.
			That is the pose that stuck.
		</p>
		{@render shot(
			'catoid/08-head-in-frame.png',
			940,
			518,
			'A cat head with a wide toothy grin in the lower left of the scene',
			'Attempts 8–11. Head whole, grin on screen, tail present.'
		)}
	</section>

	<section class="space-y-4">
		<h2 class="h2">The grin: from zipper to Cheshire</h2>
		<p class="text-surface-300">
			The first grin was 21 small boxes placed along an arc and rotated radially. Face-on and
			far away it looked fine. Zoomed in it was a zipper: separate blocks with nothing behind
			them.
		</p>
		{@render shot(
			'catoid/09-grin-zipper.png',
			750,
			600,
			'Extreme close-up of the cat face showing a ragged arc of small white boxes as teeth',
			'Attempt 12. Twenty-one loose teeth. The dark lip is also on the wrong arc — it crosses the eyes like a headband.'
		)}
		<p class="text-surface-300">
			Two bugs in one picture. The lip line was a <code class="code-inline">TorusGeometry</code>
			arc, and a torus starts at +x and sweeps counter-clockwise, so an arc meant for the lower
			face was drawn across the forehead. The teeth had no mouth: white shapes on a purple
			sphere are not a grin.
		</p>
		<p class="text-surface-300">
			The rebuild is two <code class="code-inline">RingGeometry</code> arcs in the lower half of
			the face — a dark mouth, and a slightly smaller white crescent on top of it — squashed to
			0.64 vertically, with thin radial separators cutting the white ring into thirteen teeth.
			The arc is wider than the skull, which is what makes it Cheshire.
		</p>
		{@render shot(
			'catoid/10-grin-crescent.png',
			780,
			630,
			'Close-up of a white crescent grin with radial separators and a dark rim',
			'Attempt 13. Crescent, dark rim, separators. It reads as teeth because there is a mouth behind them.'
		)}
		{@render shot(
			'catoid/11-grin-final.png',
			780,
			630,
			'The finished cat face with a wide grin, glowing eyes and whiskers',
			'Final. Whiskers moved clear of the grin, inner ears tucked in, eyes tilted for mischief.'
		)}
	</section>

	<section class="space-y-4">
		<h2 class="h2">The tail: a moving surface that allocates nothing</h2>
		{@render shot(
			'catoid/12-tail.png',
			700,
			678,
			'A long thin tail sweeping up the right side of the scene, ending in a tuft',
			'Attempt 14. The tail, mid-whip: seven rest points, a travelling wave, and a tuft riding the tip.'
		)}
		<p class="text-surface-300">
			The tail is a rest pose of seven hand-placed points, plus a travelling wave. Amplitude
			grows as <code class="code-inline">w^1.6</code> so the base stays attached while the tip
			whirls, and the displacement is applied along the local horizontal normal so the wave
			carries through the plane the tail already sweeps.
		</p>
		<pre class="code-block p-4 overflow-x-auto text-xs"><code>{tailCode}</code></pre>
		<p class="text-surface-300">
			Recomputing a tube every frame would allocate a geometry per tick, so the tail's buffers
			are written in place: position and normal arrays already exist, and only their values
			change. Verifying the animation without watching it is easy — screenshot twice, diff the
			pixels. Two frames 800 ms apart differed by about 66,000 pixels, all of them in the tail
			and the spinning objects.
		</p>
	</section>

	<section class="space-y-4">
		<h2 class="h2">The paws put themselves on the roof</h2>
		{@render shot(
			'catoid/13-paws.png',
			940,
			518,
			'Small paw shapes hanging beneath the body at the bottom of the scene',
			'Attempt 15. Paws, on the underside. The first version placed two of them on top of the ring.'
		)}
		<p class="text-surface-300">
			Obvious approach: sample the spine at fixed parameters and hang a paw below each point.
			It put two paws <em>on top</em> of the ring. The spine laps 1.35 times, so
			<code class="code-inline">t</code> is not monotonic in height — the parameter walks around
			the ring, and the rear paws landed on the upper lap. The fix: sample 240 points and keep
			only those where the body is actually low, then spread four paws along that subset.
		</p>
	</section>

	<section class="space-y-4">
		<h2 class="h2">It obeys the presets, which is the whole point of a cat</h2>
		<p class="text-surface-300">
			The cat uses the same cached material instances as the eight shapes, so it participates
			in the satin / chrome / normals / wireframe switch. Chrome cat is self-explanatory.
		</p>
		<div class="grid sm:grid-cols-2 gap-4">
			{@render shot(
				'catoid/15-chrome.png',
				940,
				518,
				'The whole scene in chrome, including a mirrored cat',
				'Chrome. The grin, eyes and teeth keep their own materials.'
			)}
			{@render shot(
				'catoid/16-wireframe.png',
				940,
				518,
				'Wireframe view of the scene including a decimated wireframe cat',
				'Wireframe. The body needed a coarse variant: at 340 × 20 rings the lines merge into a solid mass.'
			)}
		</div>
	</section>

	<section class="space-y-4">
		<h2 class="h2">One last constraint: narrow screens</h2>
		<p class="text-surface-300">
			The folio reflows from 4 × 2 to 2 × 4 on phones, and the cat is authored against the wide
			layout — so on a phone its ring and head sat outside the frame entirely. It now scales to
			0.42 there: a smaller cat, still lapping the group.
		</p>
		<div class="max-w-xs">
			{@render shot(
				'catoid/17-mobile.png',
				332,
				405,
				'A phone-sized rendering showing a small cat ring wrapping the middle of a two-column folio',
				'Mobile. Same animal, 42% of the size.'
			)}
		</div>
	</section>

	<section class="space-y-4">
		<h2 class="h2">What shipped</h2>
		{@render shot(
			'catoid/14-shipped-page.png',
			1280,
			1000,
			'The finished object folio page with the catoid wrapped around eight shapes',
			'The page: eight shapes, one cat, a 🐈 toggle, and a caption that says “Catoid” when you hover the animal.'
		)}
		<p class="text-surface-300">
			Five commits, each an attempt, squashed into
			<a
				href="https://github.com/riziles/ferray-wasm-poc/pull/45"
				target="_blank"
				class="text-primary-400 underline">one pull request</a
			>. The commit history is the lab notebook; this page is the tidy version of it.
		</p>
		<div class="overflow-x-auto">
			<table class="w-full text-sm text-surface-300">
				<thead class="text-surface-400 text-xs uppercase tracking-wide">
					<tr class="border-b border-surface-700">
						<th class="text-left py-2 pr-3">Commit</th>
						<th class="text-left py-2 pr-3">Contents</th>
						<th class="text-left py-2">Figure</th>
					</tr>
				</thead>
				<tbody>
					{#each commits as c (c.sha)}
						<tr class="border-b border-surface-800">
							<td class="py-2 pr-3">
								<a
									href="https://github.com/riziles/ferray-wasm-poc/commit/{c.sha}"
									target="_blank"
									class="text-primary-400 underline font-mono text-xs">{c.sha}</a
								>
							</td>
							<td class="py-2 pr-3">{c.label}</td>
							<td class="py-2 text-surface-400">{c.note}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</section>

	<section class="space-y-4">
		<h2 class="h2">What I would do differently</h2>
		<ul class="list-disc list-inside text-surface-300 space-y-2">
			<li>
				<strong>Compute the frustum before placing anything.</strong> Two of the six failures
				were me forgetting that the visible frame shrinks with proximity. One line of
				arithmetic would have pre-empted four commits.
			</li>
			<li>
				<strong>Look at close-ups sooner.</strong> The grin looked acceptable at canvas scale
				and was a zipper at 3×. Every “fine, ship it” moment deserved a zoom.
			</li>
			<li>
				<strong>Beware parameters that are not monotonic.</strong> The paw bug is the same bug
				as the ring-stacking bug: assuming
				<code class="code-inline">t</code> means “further along the animal” when it only means
				“further along the curve”.
			</li>
			<li>
				<strong>Commit before checking.</strong> Pushing the broken ones is what makes the
				history useful later — this post is essentially assembled from them.
			</li>
		</ul>
	</section>

	<section class="space-y-3">
		<h2 class="h2">Where the pictures live</h2>
		<p class="text-surface-300">
			Roughly 5 MB of screenshots, which has no business in a git repository. They are in a
			public Cloudflare R2 bucket (<code class="code-inline">ferray-wasm-poc-media</code>), served
			from <code class="code-inline">media.riziles.com</code> — a custom domain on that bucket,
			with Cloudflare's certificate and cache in front — and loaded lazily here — the week's
			other write-up, on building the eight shapes, lives on the
			<a href="{base}/objects" class="text-primary-400 underline">folio page itself</a>.
		</p>
		<p class="text-xs text-surface-500">
			The two folio screenshots from before the cat existed, for context:
		</p>
		<div class="grid sm:grid-cols-2 gap-4">
			{@render shot(
				'folio/01-first-render.png',
				940,
				469,
				'The first folio render: eight shapes in a 4 by 2 grid',
				'The folio, first render.'
			)}
			{@render shot(
				'folio/03-mobius-final.png',
				940,
				468,
				'The folio with a corrected Möbius strip',
				'The Möbius strip took three tilts before it read as a twisted band rather than a pink leaf.'
			)}
		</div>
	</section>

	<footer class="pt-4 border-t border-surface-800">
		<a href="{base}/objects" class="text-sm text-primary-400 hover:text-primary-300 underline"
			>← Back to the folio</a
		>
	</footer>
</div>
