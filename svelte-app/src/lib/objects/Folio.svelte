<script lang="ts">
	import { Canvas } from '@threlte/core';
	import * as THREE from 'three';
	import FolioScene from './FolioScene.svelte';
	import { CATOID_DEF, CATOID_INDEX } from './catoid';
	import { SHAPES, type MaterialKind } from './shapes';

	let host = $state<HTMLDivElement>();
	let width = $state(1024);
	const columns = $derived(width >= 720 ? 4 : 2);

	let spin = $state(1);
	let spread = $state(0);
	let float = $state(true);
	let autoRotate = $state(false);
	let catoid = $state(true);
	let material = $state<MaterialKind>('satin');
	let selected = $state<number | null>(null);
	let hovered = $state<number | null>(null);
	let resetToken = $state(0);

	function shapeInfo(index: number | null) {
		if (index === null) return null;
		return index === CATOID_INDEX ? CATOID_DEF : SHAPES[index];
	}

	const active = $derived(shapeInfo(selected));
	const caption = $derived(active ?? shapeInfo(hovered));

	const materials: { kind: MaterialKind; label: string }[] = [
		{ kind: 'satin', label: 'satin' },
		{ kind: 'chrome', label: 'chrome' },
		{ kind: 'normals', label: 'normals' },
		{ kind: 'wire', label: 'wireframe' }
	];

	function pick(index: number) {
		if (index === CATOID_INDEX) selected = selected === CATOID_INDEX ? null : CATOID_INDEX;
		else if (index < 0) selected = null;
		else selected = selected === index ? null : index;
	}
</script>

<div class="card card-demo p-4 sm:p-6 space-y-4">

	<!-- canvas -->
	<div
		bind:this={host}
		bind:clientWidth={width}
		class="relative rounded-lg overflow-hidden border border-surface-700 h-[560px] sm:h-[520px]"
		class:cursor-pointer={hovered !== null}
		class:cursor-grab={hovered === null}
	>
		<div class="absolute inset-0 bg-gradient-to-b from-surface-950 via-surface-900 to-black"></div>
		<div
			class="absolute inset-0 opacity-70"
			style="background: radial-gradient(ellipse 60% 55% at 50% 42%, rgba(56,189,248,0.10), transparent 70%);"
		></div>

		<Canvas
			createRenderer={(canvas) =>
				new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true })}
			dpr={[1, 2]}
			shadows={false}
			toneMapping={THREE.ACESFilmicToneMapping}
		>
			<FolioScene
				{columns}
				{material}
				{spin}
				{spread}
				{float}
				{autoRotate}
				{catoid}
				{selected}
				{resetToken}
				onselect={pick}
				onhover={(i) => (hovered = i)}
			/>
		</Canvas>

		<!-- info / caption overlay -->
		<div class="absolute left-3 top-3 right-3 sm:right-auto sm:max-w-sm pointer-events-none">
			{#if caption}
				<div
					class="rounded-lg border border-surface-600 bg-surface-950/80 backdrop-blur px-3 py-2 space-y-1"
				>
					<div class="flex items-center gap-2">
						<span
							class="h-2.5 w-2.5 rounded-full shrink-0"
							style="background:{caption.color}"
						></span>
						<span class="text-sm font-semibold">{caption.name}</span>
					</div>
					<p class="text-xs text-surface-400">{caption.tagline}</p>
					{#if active}
						<code class="block text-[11px] font-mono text-primary-300 break-words"
							>{caption.formula}</code
						>
					{/if}
				</div>
			{:else}
				<div
					class="rounded-lg border border-surface-700 bg-surface-950/60 backdrop-blur px-3 py-2 text-xs text-surface-400"
				>
					<span class="text-surface-200 font-medium">Eight objects, one folio.</span>
					Click any of them — or drag to orbit.
				</div>
			{/if}
		</div>

		<!-- hint -->
		<div
			class="absolute bottom-2 left-3 text-[11px] text-surface-500 pointer-events-none select-none"
		>
			drag · orbit&nbsp;&nbsp;·&nbsp;&nbsp;scroll · zoom&nbsp;&nbsp;·&nbsp;&nbsp;click · inspect
		</div>
	</div>

	<!-- object picker (also the keyboard/touch route to selecting something) -->
	<div class="flex flex-wrap gap-1.5">
		{#each SHAPES as shape, i (shape.id)}
			<button
				class="btn btn-sm preset-tonal-surface gap-1.5"
				class:preset-filled-primary={selected === i}
				onclick={() => pick(i)}
				aria-pressed={selected === i}
			>
				<span class="h-2 w-2 rounded-full" style="background:{shape.color}"></span>
				{shape.name}
			</button>
		{/each}
	</div>

	<!-- controls -->
	<div class="flex gap-x-4 gap-y-3 items-end flex-wrap">
		<label class="label w-40">
			<span>Spin: {spin.toFixed(2)}×</span>
			<input type="range" class="input" min="0" max="2.5" step="0.05" bind:value={spin} />
		</label>
		<label class="label w-40">
			<span>Spread: {(spread * 100).toFixed(0)}%</span>
			<input type="range" class="input" min="0" max="1" step="0.01" bind:value={spread} />
		</label>
		<label class="label w-36">
			<span>Material</span>
			<select class="input" bind:value={material}>
				{#each materials as m (m.kind)}
					<option value={m.kind}>{m.label}</option>
				{/each}
			</select>
		</label>
		<div class="flex gap-2 items-center flex-wrap pb-0.5">
			<button class="btn preset-tonal-surface" onclick={() => (float = !float)}>
				{float ? '🫧 float on' : '🫧 float off'}
			</button>
			<button class="btn preset-tonal-surface" onclick={() => (autoRotate = !autoRotate)}>
				{autoRotate ? '🔄 orbit on' : '🔄 orbit off'}
			</button>
			<button class="btn preset-tonal-surface" onclick={() => (catoid = !catoid)}>
				{catoid ? '🐈 catoid on' : '🐈 catoid off'}
			</button>
			<button class="btn preset-tonal-surface" onclick={() => (resetToken += 1)}>
				↺ reset view
			</button>
			{#if selected !== null}
				<button class="btn preset-tonal-surface" onclick={() => (selected = null)}>
					✕ release {active?.name.toLowerCase()}
				</button>
			{/if}
		</div>
	</div>
</div>
