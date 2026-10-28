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
 * Where the head sits: bottom-left, in front of the folio, looking at you.
 * The spine starts here and dives back into the ring.
 */
export const HEAD_ANCHOR = new THREE.Vector3(-5.15, -1.85, 0.55);

/**
 * The body spine: a spiral that laps the folio 1.35 times, sitting *behind* it
 * (z ≈ −2.8) so the eight objects are never occluded and the body only rides in
 * the margins. The radius grows with t both to thicken the animal towards the
 * hips and to keep the second lap outside the first instead of stacking on it.
 *
 * The head is not part of the spiral: it is prepended as a short neck, so the
 * head's placement stays independent of the ring's geometry.
 */
export function bodyCurve(): THREE.CatmullRomCurve3 {
	const steps = 44;
	const theta0 = -2.35; // bottom-left
	const ring: THREE.Vector3[] = [];

	for (let i = 0; i <= steps; i++) {
		const t = i / steps;
		const theta = theta0 + t * Math.PI * 2 * 1.35;
		const rx = (7.7 + 0.9 * t) * (1 + 0.03 * Math.sin(3 * theta + 0.6));
		const ry = (3.95 + 0.5 * t) * (1 + 0.04 * Math.sin(2 * theta + 1.2));
		ring.push(new THREE.Vector3(Math.cos(theta) * rx, Math.sin(theta) * ry, -2.8));
	}

	const neck = HEAD_ANCHOR.clone().lerp(ring[0], 0.45);

	return new THREE.CatmullRomCurve3(
		[HEAD_ANCHOR.clone(), neck, ...ring],
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
