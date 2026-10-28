<script lang="ts">
	import { T, useTask } from '@threlte/core';
	import { onDestroy } from 'svelte';
	import * as THREE from 'three';
	import CatHead from './CatHead.svelte';
	import {
		CATOID_DEF,
		CATOID_INDEX,
		HEAD_ANCHOR,
		TAIL_RADIAL,
		TAIL_TUBULAR,
		bodyCurve,
		bodyRadius,
		tailCurve,
		tailRadius,
		taperedTube,
		updateTaperedTube
	} from './catoid';
	import { materialFor, type MaterialKind } from './shapes';

	interface Props {
		material: MaterialKind;
	}

	let { material }: Props = $props();

	const curve = bodyCurve();
	// Two densities: the wireframe preset needs coarse geometry or the lines
	// merge into a solid mass (same reason the folio shapes carry a wire variant).
	const bodySmooth = taperedTube(curve, bodyRadius, 340, 20);
	const bodyCoarse = taperedTube(curve, bodyRadius, 150, 8);
	const body = $derived(material === 'wire' ? bodyCoarse : bodySmooth);
	onDestroy(() => {
		bodySmooth.dispose();
		bodyCoarse.dispose();
	});

	const skin = $derived(materialFor(material, CATOID_DEF));

	// The tail's spine moves every frame, so its geometry is allocated once and
	// rewritten in place (see `updateTaperedTube`) instead of rebuilt.
	const tailSmooth = taperedTube(tailCurve(0), tailRadius, TAIL_TUBULAR, TAIL_RADIAL);
	const tailCoarse = taperedTube(tailCurve(0), tailRadius, 48, 6);
	const tail = $derived(material === 'wire' ? tailCoarse : tailSmooth);
	const wire = $derived(material === 'wire');
	onDestroy(() => {
		tailSmooth.dispose();
		tailCoarse.dispose();
	});

	let tuft = $state<THREE.Mesh>();
	const tip = new THREE.Vector3();
	let clock = 0;

	useTask((delta) => {
		clock += delta;
		const spine = tailCurve(clock);
		if (wire) updateTaperedTube(tailCoarse, spine, tailRadius, 48, 6);
		else updateTaperedTube(tailSmooth, spine, tailRadius, TAIL_TUBULAR, TAIL_RADIAL);
		if (tuft) {
			spine.getPointAt(1, tip);
			tuft.position.copy(tip);
		}
	});

	// Vestigial paws. Placing them by parameter is a trap: the spine laps 1.35
	// times, so a fixed t can land on top of the ring. Instead sample the spine
	// and keep only points low down — those are the body's underside.
	const paws = (() => {
		const samples: { t: number; p: THREE.Vector3 }[] = [];
		for (let i = 0; i <= 240; i++) {
			const t = i / 240;
			const p = curve.getPointAt(t);
			if (p.y < -3.3) samples.push({ t, p });
		}
		if (!samples.length) return [];

		const count = Math.min(4, samples.length);
		return Array.from({ length: count }, (_, k) => {
			const at = count === 1 ? 0 : k / (count - 1);
			const sample = samples[Math.round(at * (samples.length - 1))];
			return {
				key: k,
				position: [sample.p.x, sample.p.y - 0.26, sample.p.z] as [number, number, number]
			};
		});
	})();

	// The head sits where the spine starts, turned to face the camera: a cat
	// looking up at you from under the folio is the whole point of a grin.
	const headPos = HEAD_ANCHOR;
	const headEuler = new THREE.Euler(-0.05, 0.3, 0.12).toArray() as [number, number, number];
</script>

<T.Group>
	<T.Mesh
		geometry={body}
		material={skin}
		dispose={false}
		userData={{ shapeIndex: CATOID_INDEX }}
	/>

	{#each paws as paw (paw.key)}
		<T.Group position={paw.position}>
			<T.Mesh material={skin} dispose={false} scale={[0.9, 0.6, 1.3]}>
				<T.SphereGeometry args={[0.3, 20, 14]} />
			</T.Mesh>
			{#each [-1, 0, 1] as toe (toe)}
				<T.Mesh
					position={[toe * 0.13, -0.09, 0.24]}
					rotation={[Math.PI / 2.2, 0, 0]}
					dispose={false}
				>
					<T.ConeGeometry args={[0.045, 0.14, 10]} />
					<T.MeshStandardMaterial color="#fdfcf5" roughness={0.3} emissive="#e2e8f0" emissiveIntensity={0.2} />
				</T.Mesh>
			{/each}
		</T.Group>
	{/each}

	<T.Mesh
		geometry={tail}
		material={skin}
		dispose={false}
		userData={{ shapeIndex: CATOID_INDEX }}
	/>

	<!-- tuft at the tip, moved by the animation task -->
	<T.Mesh bind:ref={tuft} material={skin} dispose={false} userData={{ shapeIndex: CATOID_INDEX }}>
		<T.SphereGeometry args={[0.19, 20, 16]} />
	</T.Mesh>

	<T.Group
		position={headPos.toArray()}
		rotation={headEuler}
		scale={1.55}
		userData={{ shapeIndex: CATOID_INDEX }}
	>
		<CatHead {material} />
	</T.Group>
</T.Group>
