<script lang="ts">
	import { T } from '@threlte/core';
	import { onDestroy } from 'svelte';
	import * as THREE from 'three';
	import CatHead from './CatHead.svelte';
	import {
		CATOID_DEF,
		CATOID_INDEX,
		HEAD_ANCHOR,
		bodyCurve,
		bodyRadius,
		taperedTube
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

	<T.Group
		position={headPos.toArray()}
		rotation={headEuler}
		scale={1.55}
		userData={{ shapeIndex: CATOID_INDEX }}
	>
		<CatHead {material} />
	</T.Group>
</T.Group>
