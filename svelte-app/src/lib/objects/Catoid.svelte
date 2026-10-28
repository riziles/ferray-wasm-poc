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
	const body = taperedTube(curve, bodyRadius, 340, 20);
	onDestroy(() => body.dispose());

	const skin = $derived(materialFor(material, CATOID_DEF));

	// The tail's spine moves every frame, so its geometry is allocated once and
	// rewritten in place (see `updateTaperedTube`) instead of rebuilt.
	const tail = taperedTube(tailCurve(0), tailRadius, TAIL_TUBULAR, TAIL_RADIAL);
	onDestroy(() => tail.dispose());

	let tuft = $state<THREE.Mesh>();
	const tip = new THREE.Vector3();
	let clock = 0;

	useTask((delta) => {
		clock += delta;
		const spine = tailCurve(clock);
		updateTaperedTube(tail, spine, tailRadius, TAIL_TUBULAR, TAIL_RADIAL);
		if (tuft) {
			spine.getPointAt(1, tip);
			tuft.position.copy(tip);
		}
	});

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
