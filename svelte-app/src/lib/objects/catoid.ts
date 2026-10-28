import * as THREE from 'three';

/**
 * The catoid: one ridiculously elongated cat, wrapped around the folio.
 *
 * Nothing here is a primitive. The body is a tube with a *varying* radius swept
 * along a spiral, which Three.js' `TubeGeometry` cannot do (its radius is a
 * constant), so `taperedTube` builds the rings itself from the curve's Frenet
 * frames. The tail reuses the same builder but rewrites its buffers in place
 * every frame, because its spine is animated.
 */

/** Caption metadata — the catoid is not one of the eight folio shapes. */
export const CATOID_DEF = {
	id: 'catoid',
	name: 'Catoid',
	tagline: 'One cat, 1.55 laps of the folio, no known maximum length.',
	formula: 'r(t) = 6.6 + 0.5t  ·  z = 2.4 sin(2.5θ)  ·  θ = 3.1π t',
	color: '#8b5cf6',
	spin: 0
} as const;

/** Index used in `userData.shapeIndex` for picking. */
export const CATOID_INDEX = -2;

/** Piecewise-linear radius profile: neck → chest → belly → hips. */
const BODY_PROFILE: [number, number][] = [
	[0.0, 0.16],
	[0.04, 0.24],
	[0.12, 0.32],
	[0.3, 0.29],
	[0.5, 0.27],
	[0.68, 0.25],
	[0.84, 0.22],
	[1.0, 0.17]
];

function profileAt(profile: [number, number][], t: number): number {
	const x = THREE.MathUtils.clamp(t, 0, 1);
	for (let i = 1; i < profile.length; i++) {
		const [t1, r1] = profile[i];
		if (x <= t1) {
			const [t0, r0] = profile[i - 1];
			const k = t1 === t0 ? 0 : (x - t0) / (t1 - t0);
			return r0 + (r1 - r0) * k;
		}
	}
	return profile[profile.length - 1][1];
}

export function bodyRadius(t: number): number {
	return profileAt(BODY_PROFILE, t);
}

/**
 * The body spine, authored by hand rather than by formula: the body laps the
 * folio once, sitting *behind* it so the eight objects are never occluded, and
 * only rides in the margins. The neck then rises out of that back plane at the
 * head end and brings the head forward, below the bottom row.
 *
 * Order is head → neck → shoulder → around → hips (where the tail takes over).
 */
const SPINE: [number, number, number][] = [
	[2.2, -2.9, 2.2], // head anchor, in front of the folio
	[1.7, -3.0, 0.7], // neck, crossing the plane
	[0.3, -3.5, -1.3], // shoulders, behind
	[-1.9, -3.75, -2.6],
	[-6.2, -2.4, -2.9],
	[-7.7, 1.2, -3.0],
	[-5.2, 3.5, -2.9],
	[-1.0, 4.15, -2.7],
	[3.4, 3.6, -2.8],
	[7.2, 1.0, -3.0],
	[7.5, -2.0, -2.9],
	[5.0, -3.4, -2.6],
	[3.2, -3.6, -2.2] // hips
];

export function bodyCurve(): THREE.CatmullRomCurve3 {
	return new THREE.CatmullRomCurve3(
		SPINE.map(([x, y, z]) => new THREE.Vector3(x, y, z)),
		false,
		'catmullrom',
		0.5
	);
}

/**
 * Sweep a tube along `curve` with a radius that varies with the parameter.
 * Rings are oriented with the curve's Frenet frames; the seam is duplicated so
 * the surface stays closed.
 */
export function taperedTube(
	curve: THREE.Curve<THREE.Vector3>,
	radiusAt: (t: number) => number,
	tubularSegments: number,
	radialSegments: number
): THREE.BufferGeometry {
	const geometry = new THREE.BufferGeometry();

	const vertexCount = (tubularSegments + 1) * (radialSegments + 1);
	const positions = new Float32Array(vertexCount * 3);
	const normals = new Float32Array(vertexCount * 3);
	const uvs = new Float32Array(vertexCount * 2);
	const indices: number[] = [];

	const frames = curve.computeFrenetFrames(tubularSegments, false);
	const P = new THREE.Vector3();

	for (let i = 0; i <= tubularSegments; i++) {
		const t = i / tubularSegments;
		curve.getPointAt(t, P);
		const N = frames.normals[i];
		const B = frames.binormals[i];
		const r = radiusAt(t);

		for (let j = 0; j <= radialSegments; j++) {
			const v = (j / radialSegments) * Math.PI * 2;
			const sin = Math.sin(v);
			const cos = -Math.cos(v);

			const nx = cos * N.x + sin * B.x;
			const ny = cos * N.y + sin * B.y;
			const nz = cos * N.z + sin * B.z;

			const k = i * (radialSegments + 1) + j;
			positions[k * 3] = P.x + r * nx;
			positions[k * 3 + 1] = P.y + r * ny;
			positions[k * 3 + 2] = P.z + r * nz;
			normals[k * 3] = nx;
			normals[k * 3 + 1] = ny;
			normals[k * 3 + 2] = nz;
			uvs[k * 2] = t;
			uvs[k * 2 + 1] = j / radialSegments;
		}
	}

	for (let i = 1; i <= tubularSegments; i++) {
		for (let j = 1; j <= radialSegments; j++) {
			const a = (radialSegments + 1) * (i - 1) + (j - 1);
			const b = (radialSegments + 1) * i + (j - 1);
			const c = (radialSegments + 1) * i + j;
			const d = (radialSegments + 1) * (i - 1) + j;
			indices.push(a, b, d, b, c, d);
		}
	}

	geometry.setIndex(indices);
	geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
	geometry.setAttribute('normal', new THREE.BufferAttribute(normals, 3));
	geometry.setAttribute('uv', new THREE.BufferAttribute(uvs, 2));
	geometry.computeBoundingSphere();

	return geometry;
}

/**
 * Rewrite an existing tapered tube in place (same segment counts) — used by the
 * tail, whose spine moves every frame.
 */
export function updateTaperedTube(
	geometry: THREE.BufferGeometry,
	curve: THREE.Curve<THREE.Vector3>,
	radiusAt: (t: number) => number,
	tubularSegments: number,
	radialSegments: number
): void {
	const positions = geometry.getAttribute('position') as THREE.BufferAttribute;
	const normals = geometry.getAttribute('normal') as THREE.BufferAttribute;
	const pos = positions.array as Float32Array;
	const nor = normals.array as Float32Array;

	const frames = curve.computeFrenetFrames(tubularSegments, false);
	const P = new THREE.Vector3();

	for (let i = 0; i <= tubularSegments; i++) {
		const t = i / tubularSegments;
		curve.getPointAt(t, P);
		const N = frames.normals[i];
		const B = frames.binormals[i];
		const r = radiusAt(t);

		for (let j = 0; j <= radialSegments; j++) {
			const v = (j / radialSegments) * Math.PI * 2;
			const sin = Math.sin(v);
			const cos = -Math.cos(v);

			const nx = cos * N.x + sin * B.x;
			const ny = cos * N.y + sin * B.y;
			const nz = cos * N.z + sin * B.z;

			const k = (i * (radialSegments + 1) + j) * 3;
			pos[k] = P.x + r * nx;
			pos[k + 1] = P.y + r * ny;
			pos[k + 2] = P.z + r * nz;
			nor[k] = nx;
			nor[k + 1] = ny;
			nor[k + 2] = nz;
		}
	}

	positions.needsUpdate = true;
	normals.needsUpdate = true;
}
