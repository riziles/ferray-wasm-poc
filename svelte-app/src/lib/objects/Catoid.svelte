<script lang="ts">
	import { T } from '@threlte/core';
	import { onDestroy } from 'svelte';
	import * as THREE from 'three';
	import { CATOID_DEF, CATOID_INDEX, bodyCurve, bodyRadius, taperedTube } from './catoid';
	import { materialFor, type MaterialKind } from './shapes';

	interface Props {
		material: MaterialKind;
	}

	let { material }: Props = $props();

	const curve = bodyCurve();
	const body = taperedTube(curve, bodyRadius, 340, 20);
	onDestroy(() => body.dispose());

	const skin = $derived(materialFor(material, CATOID_DEF));

	// Placeholder head: just enough to see where the neck starts and which way
	// it points, until the real head lands.
	const headPos = curve.getPointAt(0);
	const headTangent = curve.getTangentAt(0).normalize();
	const headQuat = new THREE.Quaternion()
		.setFromUnitVectors(new THREE.Vector3(0, 0, 1), headTangent)
		.toArray() as [number, number, number, number];
</script>

<T.Group>
	<T.Mesh
		geometry={body}
		material={skin}
		dispose={false}
		userData={{ shapeIndex: CATOID_INDEX }}
	/>

	<T.Mesh
		position={headPos.toArray()}
		quaternion={headQuat}
		material={skin}
		dispose={false}
		userData={{ shapeIndex: CATOID_INDEX }}
	>
		<T.SphereGeometry args={[0.62, 32, 24]} />
	</T.Mesh>
</T.Group>
