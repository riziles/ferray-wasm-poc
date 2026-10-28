<script lang="ts">
	import { T } from '@threlte/core';
	import * as THREE from 'three';
	import type { MaterialKind } from './shapes';
	import { materialFor } from './shapes';

	import { CATOID_DEF } from './catoid';

	interface Props {
		material: MaterialKind;
	}

	let { material }: Props = $props();

	const skin = $derived(materialFor(material, CATOID_DEF));

	// The grin is a filled crescent, not a row of loose blocks: a dark ring
	// (the mouth) with a slightly smaller white ring on top of it (the teeth),
	// both drawn as `RingGeometry` arcs in the lower half of the face, then
	// squashed vertically. Thin radial separators convert the white ring into
	// individual teeth. Angles start at +x and sweep CCW, so the smile lives
	// between ~190° and ~350°.
	const GRIN_START = Math.PI * 1.04;
	const GRIN_SWEEP = Math.PI * 0.92;
	const TOOTH_COUNT = 13;

	const separators = $derived.by(() =>
		Array.from({ length: TOOTH_COUNT + 1 }, (_, i) => {
			const u = i / TOOTH_COUNT;
			const angle = GRIN_START + u * GRIN_SWEEP;
			return {
				key: i,
				position: [Math.cos(angle) * 0.6, Math.sin(angle) * 0.6, 0.01] as [
					number,
					number,
					number
				],
				rotation: [0, 0, angle + Math.PI / 2] as [number, number, number]
			};
		})
	);

	const whiskers = $derived(
		[-1, 1].flatMap((side) =>
			[-0.22, 0, 0.22].map((tilt, i) => ({
				key: `${side}-${i}`,
				position: [side * 0.33, 0.02 + tilt * 0.3, 0.46] as [number, number, number],
				rotation: [0, 0, side * (Math.PI / 2 - tilt)] as [number, number, number]
			}))
		)
	);
</script>

<T.Group>
	<!-- cranium, slightly squashed -->
	<T.Mesh material={skin} dispose={false} scale={[1.06, 0.94, 0.96]}>
		<T.SphereGeometry args={[0.62, 40, 28]} />
	</T.Mesh>

	<!-- muzzle -->
	<T.Mesh material={skin} dispose={false} position={[0, -0.2, 0.36]} scale={[1.0, 0.66, 0.8]}>
		<T.SphereGeometry args={[0.4, 32, 24]} />
	</T.Mesh>

	<!-- ears, with a pink inner |
	     cones are rotated so they lean outwards -->
	{#each [-1, 1] as side (side)}
		<T.Mesh
			material={skin}
			dispose={false}
			position={[side * 0.36, 0.5, 0]}
			rotation={[0, 0, -side * 0.34]}
		>
			<T.ConeGeometry args={[0.24, 0.46, 24]} />
		</T.Mesh>
		<T.Mesh
			position={[side * 0.35, 0.4, 0.14]}
			rotation={[0, 0, -side * 0.34]}
			dispose={false}
		>
			<T.ConeGeometry args={[0.11, 0.2, 20]} />
			<T.MeshStandardMaterial color="#f9a8d4" roughness={0.6} />
		</T.Mesh>
	{/each}

	<!-- eyes: glowing almond, vertical slit pupil -->
	{#each [-1, 1] as side (side)}
		<T.Mesh
			position={[side * 0.26, 0.14, 0.44]}
			scale={[1, 0.78, 0.62]}
			rotation={[0, 0, side * -0.2]}
			dispose={false}
		>
			<T.SphereGeometry args={[0.19, 28, 20]} />
			<T.MeshStandardMaterial
				color="#0b1a00"
				emissive="#d4ff2a"
				emissiveIntensity={1.1}
				roughness={0.25}
			/>
		</T.Mesh>
		<T.Mesh
			position={[side * 0.26, 0.14, 0.56]}
			scale={[0.22, 1.05, 0.5]}
			dispose={false}
		>
			<T.SphereGeometry args={[0.1, 16, 16]} />
			<T.MeshStandardMaterial color="#05050a" roughness={0.1} metalness={0.2} />
		</T.Mesh>
	{/each}

	<!-- nose -->
	<T.Mesh position={[0, -0.16, 0.66]} rotation={[Math.PI / 2, 0, 0]} dispose={false}>
		<T.ConeGeometry args={[0.07, 0.09, 12]} />
		<T.MeshStandardMaterial color="#f472b6" roughness={0.4} />
	</T.Mesh>

	<!-- whiskers -->
	{#each whiskers as w (w.key)}
		<T.Mesh position={w.position} rotation={w.rotation} dispose={false}>
			<T.CylinderGeometry args={[0.008, 0.005, 0.62, 6]} />
			<T.MeshStandardMaterial color="#f8fafc" roughness={0.5} emissive="#cbd5e1" emissiveIntensity={0.25} />
		</T.Mesh>
	{/each}

	<!-- the grin: dark mouth crescent, white crescent, radial tooth gaps -->
	<T.Group position={[0, -0.03, 0.5]} scale={[1, 0.64, 1]}>
		<T.Mesh dispose={false}>
			<T.RingGeometry args={[0.42, 0.86, 64, 1, GRIN_START, GRIN_SWEEP]} />
			<T.MeshStandardMaterial color="#160410" roughness={0.5} />
		</T.Mesh>

		<T.Mesh position={[0, 0, 0.012]} dispose={false}>
			<T.RingGeometry args={[0.455, 0.8, 64, 1, GRIN_START, GRIN_SWEEP]} />
			<T.MeshStandardMaterial
				color="#fffdf3"
				emissive="#fff3c4"
				emissiveIntensity={0.7}
				roughness={0.25}
			/>
		</T.Mesh>

		{#each separators as sep (sep.key)}
			<T.Mesh position={sep.position} rotation={sep.rotation} dispose={false}>
				<T.BoxGeometry args={[0.016, 0.3, 0.02]} />
				<T.MeshStandardMaterial color="#2a0714" roughness={0.6} />
			</T.Mesh>
		{/each}
	</T.Group>
</T.Group>
