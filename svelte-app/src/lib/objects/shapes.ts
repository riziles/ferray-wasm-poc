import * as THREE from 'three';

/**
 * The folio's object catalogue. Everything here is plain Three.js geometry —
 * the page itself only adds materials, lights and interaction.
 */

export type MaterialKind = 'satin' | 'chrome' | 'normals' | 'wire';

export interface ShapeDef {
	/** Stable id, also the key for the geometry cache. */
	id: string;
	name: string;
	/** One-line "what it is", shown under the title. */
	tagline: string;
	/** Short formula / parameter summary for the info card. */
	formula: string;
	color: string;
	/** Per-object spin multiplier, so the folio never marches in lockstep. */
	spin: number;
	/** Crisp faceted shading — right for polyhedra, wrong for smooth surfaces. */
	flat?: boolean;
	/** Shaded geometry. */
	build: () => THREE.BufferGeometry;
	/** Optional coarse geometry for the wireframe preset: at full resolution the
	 *  lines merge into a solid silhouette. Defaults to `build`. */
	wire?: () => THREE.BufferGeometry;
}

/**
 * Build a `BufferGeometry` from `f(u, v)` with `u, v ∈ [0, 1]`.
 *
 * This is the same idea as Three.js' `ParametricGeometry` addon, but it lives
 * here so the folio has no extra dependency and the UV seam handling is
 * explicit: `slices + 1` rings are generated, the last one duplicating `u = 0`,
 * which keeps the surface watertight.
 */
function parametric(
	fn: (u: number, v: number, out: THREE.Vector3) => void,
	slices = 96,
	stacks = 48
): THREE.BufferGeometry {
	const positions: number[] = [];
	const uvs: number[] = [];
	const indices: number[] = [];
	const p = new THREE.Vector3();

	for (let i = 0; i <= slices; i++) {
		const u = i / slices;
		for (let j = 0; j <= stacks; j++) {
			const v = j / stacks;
			fn(u, v, p);
			positions.push(p.x, p.y, p.z);
			uvs.push(u, v);
		}
	}

	const stride = stacks + 1;
	for (let i = 0; i < slices; i++) {
		for (let j = 0; j < stacks; j++) {
			const a = i * stride + j;
			const b = a + stride;
			indices.push(a, b, a + 1, b, b + 1, a + 1);
		}
	}

	const geometry = new THREE.BufferGeometry();
	geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
	geometry.setAttribute('uv', new THREE.Float32BufferAttribute(uvs, 2));
	geometry.setIndex(indices);
	geometry.computeVertexNormals();
	return geometry;
}

/** Centre a geometry on the origin and scale it so it fills the same cell. */
function fit(geometry: THREE.BufferGeometry, radius = 1.08): THREE.BufferGeometry {
	geometry.computeBoundingSphere();
	const sphere = geometry.boundingSphere;
	if (!sphere || sphere.radius === 0) return geometry;

	geometry.translate(-sphere.center.x, -sphere.center.y, -sphere.center.z);
	const s = radius / sphere.radius;
	geometry.scale(s, s, s);
	geometry.computeBoundingSphere();
	return geometry;
}

/** Gielis superformula in polar form. */
function superformula(
	theta: number,
	m: number,
	n1: number,
	n2: number,
	n3: number,
	a = 1,
	b = 1
): number {
	const t1 = Math.abs(Math.cos((m * theta) / 4) / a) ** n2;
	const t2 = Math.abs(Math.sin((m * theta) / 4) / b) ** n3;
	return (t1 + t2) ** (-1 / n1);
}

const TAU = Math.PI * 2;

// ---- surfaces ------------------------------------------------------------
// Each surface is a pure f(u, v) so the shaded and wireframe variants can be
// sampled at different resolutions from the same source of truth.

/** Klein bottle as the figure-8 immersion; tilted so the twist is visible. */
const klein = (u: number, v: number, out: THREE.Vector3) => {
	const U = u * TAU;
	const V = v * TAU;
	const a = 2;
	const r = a + Math.cos(U / 2) * Math.sin(V) - Math.sin(U / 2) * Math.sin(2 * V);
	out.set(
		r * Math.cos(U),
		r * Math.sin(U),
		Math.sin(U / 2) * Math.sin(V) + Math.cos(U / 2) * Math.sin(2 * V)
	);
};

/** Möbius strip: a band whose cross-section rotates half a turn per lap. */
const mobius = (u: number, v: number, out: THREE.Vector3) => {
	const U = u * TAU;
	const s = (v - 0.5) * 2; // v ∈ [−1, 1]
	const r = 1 + 0.24 * s * Math.cos(U / 2);
	out.set(r * Math.cos(U), r * Math.sin(U), 0.24 * s * Math.sin(U / 2));
};

/** Spherical harmonic: radius wobbles with polar angle. */
const harmonic = (u: number, v: number, out: THREE.Vector3) => {
	const [m0, m1, m2, m3] = [6, 2, 4, 2];
	const phi = v * Math.PI; // polar angle
	const theta = u * TAU; // azimuth
	const r = Math.sin(m0 * phi) ** m1 + Math.cos(m2 * phi) ** m3;
	const s = r * Math.sin(phi);
	out.set(s * Math.cos(theta), r * Math.cos(phi), s * Math.sin(theta));
};

/** Gielis 3D supershape: two radial functions multiplied together. */
const supershape = (u: number, v: number, out: THREE.Vector3) => {
	const theta = u * TAU; // longitude
	const phi = (v - 0.5) * Math.PI; // latitude
	const [m, n1, n2, n3] = [7, 0.2, 1.7, 1.7];
	const r1 = superformula(theta, m, n1, n2, n3);
	const r2 = superformula(phi, m, n1, n2, n3);
	const c = r2 * Math.cos(phi);
	out.set(r1 * c * Math.cos(theta), r1 * c * Math.sin(theta), r2 * Math.sin(phi));
};

/** A helix swept around the Y axis, used as a `TubeGeometry` spine. */
class HelixCurve extends THREE.Curve<THREE.Vector3> {
	constructor(
		private readonly turns = 3.25,
		private readonly height = 2.4,
		private readonly radius = 0.6
	) {
		super();
	}

	override getPoint(t: number, target = new THREE.Vector3()): THREE.Vector3 {
		const a = t * Math.PI * 2 * this.turns;
		return target.set(
			this.radius * Math.cos(a),
			(t - 0.5) * this.height,
			this.radius * Math.sin(a)
		);
	}
}

const helix = (tubular: number, radius: number, radial: number) =>
	new THREE.TubeGeometry(new HelixCurve(3.25, 2.5, 0.72), tubular, radius, radial, false)
		.rotateX(0.35)
		.rotateZ(0.18);

// ---- the folio -----------------------------------------------------------

export const SHAPES: ShapeDef[] = [
	{
		id: 'torus-knot',
		name: 'Torus knot (2, 3)',
		tagline: 'A knot drawn on a doughnut and inflated into a tube.',
		formula: 'p = 2, q = 3  ·  220 × 28 tube segments',
		color: '#ff8a3d',
		spin: 0.9,
		build: () => fit(new THREE.TorusKnotGeometry(0.72, 0.26, 220, 28, 2, 3)),
		wire: () => new THREE.TorusKnotGeometry(0.72, 0.26, 64, 8, 2, 3)
	},
	{
		id: 'icosahedron',
		name: 'Icosahedron',
		tagline: '20 triangles, 12 vertices, five meeting at every corner.',
		formula: 'V − E + F = 12 − 30 + 20 = 2',
		color: '#5eead4',
		spin: 0.6,
		flat: true,
		build: () => fit(new THREE.IcosahedronGeometry(1.08, 1)),
		wire: () => fit(new THREE.IcosahedronGeometry(1.08, 0))
	},
	{
		id: 'klein',
		name: 'Klein bottle',
		tagline: 'A closed surface with no inside — here as the figure-8 immersion.',
		formula: '(a + cos(u/2) sin v − sin(u/2) sin 2v) · (cos u, sin u)',
		color: '#a78bfa',
		spin: 0.5,
		build: () => fit(parametric(klein, 120, 48).rotateY(0.9).rotateX(-0.35)),
		wire: () => fit(parametric(klein, 30, 8).rotateY(0.9).rotateX(-0.35))
	},
	{
		id: 'mobius',
		name: 'Möbius strip',
		tagline: 'One side, one edge. Cut it lengthways and it stays in one piece.',
		formula: '((1 + ½v cos ½u) cos u, (1 + ½v cos ½u) sin u, ½v sin ½u)',
		color: '#f472b6',
		spin: 0.75,
		build: () => fit(parametric(mobius, 220, 8).rotateX(0.55).rotateZ(-0.5)),
		wire: () => fit(parametric(mobius, 54, 3).rotateX(0.55).rotateZ(-0.5))
	},
	{
		id: 'harmonic',
		name: 'Spherical harmonic',
		tagline: 'A sphere whose radius wobbles — count the lobes to read the parameters.',
		formula: 'r(θ) = sinᵐ⁰θ ^ ᵐ¹ + cosᵐ²θ ^ ᵐ³',
		color: '#60a5fa',
		spin: 0.45,
		build: () => fit(parametric(harmonic, 96, 96)),
		wire: () => fit(parametric(harmonic, 30, 18))
	},
	{
		id: 'torus',
		name: 'Torus',
		tagline: 'The one surface of revolution that is flat everywhere.',
		formula: 'R = 0.7, r = 0.28  ·  Gaussian curvature K = 0',
		color: '#facc15',
		spin: 0.65,
		build: () => fit(new THREE.TorusGeometry(0.7, 0.28, 48, 128)),
		wire: () => new THREE.TorusGeometry(0.7, 0.28, 8, 28)
	},
	{
		id: 'helix',
		name: 'Helical tube',
		tagline: 'A curve with constant curvature and constant torsion, given thickness.',
		formula: 'x = 0.72 cos a, y = t − ½, z = 0.72 sin a,  a = 3.25 · 2πt',
		color: '#4ade80',
		spin: 0.85,
		build: () => fit(helix(512, 0.16, 20)),
		wire: () => fit(helix(140, 0.05, 6))
	},
	{
		id: 'supershape',
		name: 'Superformula star',
		tagline: 'Two radial functions multiplied: a handful of exponents, an entire zoo.',
		formula: 'm = 7, n₁ = 0.2, n₂ = n₃ = 1.7',
		color: '#fb7185',
		spin: 0.7,
		build: () => fit(parametric(supershape, 128, 64)),
		wire: () => fit(parametric(supershape, 30, 14))
	}
];

/**
 * Geometries are built once per shape (and per preset) and shared across
 * remounts. Threlte meshes are created with `dispose={false}` so this cache
 * stays valid.
 */
const geometryCache = new Map<string, THREE.BufferGeometry>();

export function geometryFor(def: ShapeDef, kind: MaterialKind = 'satin'): THREE.BufferGeometry {
	const wire = kind === 'wire';
	const key = `${def.id}:${wire ? 'wire' : 'shaded'}`;
	let geometry = geometryCache.get(key);
	if (!geometry) {
		geometry = wire && def.wire ? def.wire() : def.build();
		geometryCache.set(key, geometry);
	}
	return geometry;
}

/**
 * Materials are cached the same way, one per (shape, preset) pair. They are
 * handed to the mesh as a plain `material` property rather than as conditional
 * child components, so switching presets is a single assignment.
 */
const materialCache = new Map<string, THREE.Material>();

export function materialFor(kind: MaterialKind, def: ShapeDef): THREE.Material {
	const key = `${kind}:${def.id}`;
	const cached = materialCache.get(key);
	if (cached) return cached;

	const flatShading = def.flat ?? false;
	let material: THREE.Material;

	if (kind === 'wire') {
		material = new THREE.MeshBasicMaterial({
			color: def.color,
			wireframe: true,
			side: THREE.DoubleSide
		});
	} else if (kind === 'normals') {
		material = new THREE.MeshNormalMaterial({ side: THREE.DoubleSide, flatShading });
	} else {
		const chrome = kind === 'chrome';
		material = new THREE.MeshStandardMaterial({
			color: def.color,
			metalness: chrome ? 1 : 0.32,
			roughness: chrome ? 0.07 : 0.32,
			envMapIntensity: chrome ? 1.8 : 1,
			flatShading,
			side: THREE.DoubleSide
		});
	}

	materialCache.set(key, material);
	return material;
}
