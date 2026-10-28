<script lang="ts">
	import { T, useThrelte, useTask } from '@threlte/core';
	import { onMount } from 'svelte';
	import * as THREE from 'three';
	import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
	import { RoomEnvironment } from 'three/addons/environments/RoomEnvironment.js';
	import FolioItem from './FolioItem.svelte';
	import { SHAPES, type MaterialKind } from './shapes';

	interface Props {
		columns: number;
		material: MaterialKind;
		spin: number;
		spread: number;
		float: boolean;
		autoRotate: boolean;
		selected: number | null;
		resetToken: number;
		/** Shape index, or -1 for "clicked empty space". */
		onselect: (index: number) => void;
		onhover: (index: number | null) => void;
	}

	let {
		columns,
		material,
		spin,
		spread,
		float,
		autoRotate,
		selected,
		resetToken,
		onselect,
		onhover
	}: Props = $props();

	const { renderer, scene, camera } = useThrelte();

	let cameraRef = $state<THREE.PerspectiveCamera>();
	let controls = $state<OrbitControls>();
	let hovered = $state<number | null>(null);

	// RoomEnvironment is a tiny procedurally-lit box; PMREM turns it into the
	// reflection map that makes the chrome preset look like something.
	onMount(() => {
		const pmrem = new THREE.PMREMGenerator(renderer);
		const room = new RoomEnvironment();
		const env = pmrem.fromScene(room, 0.04).texture;
		scene.environment = env;
		pmrem.dispose();

		return () => {
			scene.environment = null;
			env.dispose();
			room.dispose();
		};
	});

	// OrbitControls needs the camera instance, which only exists once the child
	// `<T.PerspectiveCamera>` has mounted — hence the effect rather than onMount.
	$effect(() => {
		const cam = cameraRef;
		if (!cam) return;

		const c = new OrbitControls(cam, renderer.domElement);
		c.enableDamping = true;
		c.dampingFactor = 0.075;
		c.enablePan = false;
		c.minDistance = 6;
		c.maxDistance = 26;
		c.minPolarAngle = 0.35;
		c.maxPolarAngle = Math.PI - 0.35;
		c.target.set(0, 0, 0);
		c.update();
		controls = c;

		return () => {
			c.dispose();
			controls = undefined;
		};
	});

	$effect(() => {
		if (!controls) return;
		controls.autoRotate = autoRotate;
		controls.autoRotateSpeed = 0.6;
	});

	// `fov` is a plain property on the camera, so changing it as a reactive prop
	// leaves a stale projection matrix (the wrapper resizes the canvas but never
	// calls updateProjectionMatrix). Wide canvases get a narrow lens, tall ones a
	// wide lens so the 2 × 4 layout still fits.
	$effect(() => {
		const cam = cameraRef;
		if (!cam) return;
		const fov = columns >= 4 ? 40 : 52;
		if (cam.fov !== fov) {
			cam.fov = fov;
			cam.updateProjectionMatrix();
		}
	});

	$effect(() => {
		if (!controls || resetToken === 0) return;
		controls.reset();
	});

	useTask((delta) => {
		controls?.update(delta);
	});

	// ---- picking ----------------------------------------------------------
	// @threlte/core 8.6 ships no interactivity layer (no raycaster), so hit
	// testing is done here: one raycaster against the scene, with a small
	// movement/time threshold so that orbiting never registers as a click.
	const raycaster = new THREE.Raycaster();
	const pointer = new THREE.Vector2();
	const PICK_TOLERANCE = 7; // px
	const CLICK_MAX_MS = 700;

	function pick(clientX: number, clientY: number): number | null {
		const cam = camera.current;
		if (!cam) return null;

		const rect = renderer.domElement.getBoundingClientRect();
		pointer.set(
			((clientX - rect.left) / rect.width) * 2 - 1,
			-((clientY - rect.top) / rect.height) * 2 + 1
		);
		raycaster.setFromCamera(pointer, cam);

		for (const hit of raycaster.intersectObjects(scene.children, true)) {
			const index = hit.object.userData?.shapeIndex;
			if (typeof index === 'number') return index;
		}
		return null;
	}

	onMount(() => {
		const el = renderer.domElement;
		let downAt: { x: number; y: number; time: number } | null = null;

		const setHovered = (index: number | null) => {
			if (index === hovered) return;
			hovered = index;
			onhover(index);
		};

		const onPointerMove = (e: PointerEvent) => setHovered(pick(e.clientX, e.clientY));

		const onPointerDown = (e: PointerEvent) => {
			downAt = { x: e.clientX, y: e.clientY, time: performance.now() };
		};

		const onPointerUp = (e: PointerEvent) => {
			const down = downAt;
			downAt = null;
			if (!down) return;

			const moved = Math.hypot(e.clientX - down.x, e.clientY - down.y);
			if (moved > PICK_TOLERANCE || performance.now() - down.time > CLICK_MAX_MS) return;

			onselect(pick(e.clientX, e.clientY) ?? -1);
		};

		const onPointerLeave = () => {
			downAt = null;
			setHovered(null);
		};

		el.addEventListener('pointermove', onPointerMove);
		el.addEventListener('pointerdown', onPointerDown);
		el.addEventListener('pointerup', onPointerUp);
		el.addEventListener('pointerleave', onPointerLeave);

		return () => {
			el.removeEventListener('pointermove', onPointerMove);
			el.removeEventListener('pointerdown', onPointerDown);
			el.removeEventListener('pointerup', onPointerUp);
			el.removeEventListener('pointerleave', onPointerLeave);
		};
	});

	// 4 × 2 on a wide canvas, 2 × 4 on a narrow one, with the row/column gaps
	// tuned so all eight objects stay inside the frustum either way.
	const layout = $derived.by(() => {
		const cols = Math.max(1, columns);
		const rows = Math.ceil(SHAPES.length / cols);
		const gapX = (cols >= 4 ? 3.05 : 3.4) * (1 + spread * (cols >= 4 ? 0.3 : 0.2));
		const gapY = (cols >= 4 ? 2.9 : 2.05) * (1 + spread * 0.3);

		return SHAPES.map((def, i) => ({
			def,
			x: (i % cols - (cols - 1) / 2) * gapX,
			y: ((rows - 1) / 2 - Math.floor(i / cols)) * gapY,
			phase: (i * Math.PI) / 3
		}));
	});
</script>

<T.PerspectiveCamera
	bind:ref={cameraRef}
	makeDefault
	position={[0, 0, 11]}
	fov={40}
	near={0.1}
	far={100}
/>

<T.AmbientLight intensity={0.3} />
<T.DirectionalLight position={[5, 7, 9]} intensity={2.6} />
<T.DirectionalLight position={[-7, -4, -6]} intensity={0.8} color="#8ab4ff" />
<T.DirectionalLight position={[-4, 6, -3]} intensity={1.1} color="#ffd9a8" />

{#each layout as item, i (item.def.id)}
	<FolioItem
		index={i}
		def={item.def}
		x={item.x}
		y={item.y}
		phase={item.phase}
		active={selected === i}
		hovered={hovered === i}
		dimmed={selected !== null && selected !== i}
		{material}
		{spin}
		{float}
	/>
{/each}
