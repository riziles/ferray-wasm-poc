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

	// Wide, U-shaped grin: angles run from just past the left cheek, under the
	// chin, up to just past the right cheek. Teeth are placed on that arc and
	// rotated radially, and shrink towards the corners.
	const GRIN_START = -Math.PI * 0.94;
	const GRIN_END = -Math.PI * 0.06;

	const teeth = $derived.by(() => {
		const count = 21;
		return Array.from({ length: count }, (_, i) => {
			const u = i / (count - 1);
			const angle = THREE.MathUtils.lerp(GRIN_START, GRIN_END, u);
			const edge = Math.abs(u - 0.5) * 2; // 0 mid, 1 at the corners
			return {
				key: i,
				x: Math.cos(angle) * 0.7,
				y: Math.sin(angle) * 0.42 - 0.04,
				rot: angle + Math.PI / 2,
				w: 0.085 - edge * 0.03,
				h: 0.17 - edge * 0.09
			};
		});
	});

	const whiskers = $derived(
		[-1, 1].flatMap((side) =>
			[-0.28, 0, 0.28].map((tilt, i) => ({
				key: `${side}-${i}`,
				position: [side * 0.34, -0.06 + tilt * 0.35, 0.44] as [number, number, number],
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
			position={[side * 0.37, 0.47, 0.1]}
			rotation={[0, 0, -side * 0.34]}
			dispose={false}
		>
			<T.ConeGeometry args={[0.14, 0.3, 20]} />
			<T.MeshStandardMaterial color="#f9a8d4" roughness={0.6} />
		</T.Mesh>
	{/each}

	<!-- eyes: glowing almond, vertical slit pupil -->
	{#each [-1, 1] as side (side)}
		<T.Mesh
			position={[side * 0.26, 0.14, 0.44]}
			scale={[1, 0.78, 0.62]}
			rotation={[0, 0, side * -0.12]}
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

	<!-- the grin: a dark lip line with a row of glowing teeth over it -->
	<T.Mesh position={[0, -0.04, 0.52]} scale={[1, 0.6, 1]} dispose={false}>
		<T.TorusGeometry args={[0.7, 0.03, 10, 64, Math.PI * 0.88]} />
		<T.MeshStandardMaterial color="#1b0a14" roughness={0.5} />
	</T.Mesh>

	{#each teeth as tooth (tooth.key)}
		<T.Mesh
			position={[tooth.x, tooth.y, 0.53]}
			rotation={[0, 0, tooth.rot]}
			dispose={false}
		>
			<T.BoxGeometry args={[tooth.w, tooth.h, 0.05]} />
			<T.MeshStandardMaterial
				color="#fdfcf5"
				emissive="#fff8e1"
				emissiveIntensity={0.75}
				roughness={0.28}
			/>
		</T.Mesh>
	{/each}
</T.Group>
