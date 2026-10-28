<script lang="ts">
	import { T, useTask } from '@threlte/core';
	import { untrack } from 'svelte';
	import * as THREE from 'three';
	import { geometryFor, materialFor, type MaterialKind, type ShapeDef } from './shapes';

	interface Props {
		index: number;
		def: ShapeDef;
		x: number;
		y: number;
		phase: number;
		active: boolean;
		hovered: boolean;
		dimmed: boolean;
		material: MaterialKind;
		spin: number;
		float: boolean;
	}

	let {
		index,
		def,
		x,
		y,
		phase,
		active,
		hovered,
		dimmed,
		material: kind,
		spin,
		float
	}: Props = $props();

	const geometry = $derived(geometryFor(def, kind));
	const material = $derived(materialFor(kind, def));

	let group = $state<THREE.Group>();
	let mesh = $state<THREE.Mesh>();

	// Layout target; the group eases toward it so resizing the window and
	// dragging the "spread" slider glide instead of snapping. The initial value
	// is read untracked — after that only the effect below moves it.
	const target = new THREE.Vector3(untrack(() => x), untrack(() => y), 0);
	$effect(() => {
		target.set(x, y, 0);
	});

	let t = 0;

	useTask((delta) => {
		const g = group;
		const m = mesh;
		if (!g || !m) return;

		t += delta;
		const ease = 1 - Math.exp(-7 * delta);

		g.position.x += (target.x - g.position.x) * ease;
		g.position.y += (target.y - g.position.y) * ease;

		// Bobbing lives on the mesh, not the group, so it never fights the easing.
		const bob = float ? Math.sin(t * 0.8 + phase) * 0.14 : 0;
		m.position.y += (bob - m.position.y) * ease;

		m.rotation.y += delta * def.spin * spin * (active ? 2.4 : 1);
		m.rotation.x = Math.sin(t * 0.35 + phase) * 0.1;

		const want = active ? 1.24 : hovered ? 1.12 : dimmed ? 0.86 : 1;
		m.scale.setScalar(THREE.MathUtils.damp(m.scale.x, want, 7, delta));
	});
</script>

<T.Group bind:ref={group} position={[x, y, 0]}>
	<T.Mesh bind:ref={mesh} {geometry} {material} dispose={false} userData={{ shapeIndex: index }} />
</T.Group>
