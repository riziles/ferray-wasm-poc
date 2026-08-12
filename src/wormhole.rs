//! Interactive 3D wormhole *embedding diagram* renderer.
//!
//! The familiar "two flat sheets joined by a throat" picture is an
//! **embedding diagram**: a surface of revolution whose *intrinsic* geometry
//! equals a spatial (constant-time) slice of a wormhole metric. This module
//! builds that surface — Ellis' catenoid or Flamm's paraboloid — morphs the
//! classic three-panel "construction" (separate holed sheets → glowing rims →
//! welded throat), and renders it with a tiny software 3D pipeline:
//!
//!   mesh → yaw/pitch → perspective → painter sort → shaded draw list
//!
//! The Svelte side only replays the returned draw list on a canvas 2D
//! context, so all of the math lives in Rust. Pure scalar code — no ferray
//! crates needed.

const TAU: f64 = std::f64::consts::TAU;

// ── embedding-profile math ────────────────────────────────────────────────
// Everything is normalized so the outer sheet radius is 1 and the throat
// radius is the ratio `q` = b₀/R ∈ (0, 1).

/// Height ẑ ≥ 0 of the embedding surface at (normalized) radius r ≥ q.
///
/// * profile 0 — Ellis (traversable, catenoid):  ẑ = q·acosh(r/q)
/// * profile 1 — Flamm (Schwarzschild/Einstein–Rosen): ẑ = 2·√(q·(r−q))
fn profile_z(profile: u32, q: f64, r: f64) -> f64 {
    let r = r.max(q);
    match profile {
        1 => 2.0 * (q * (r - q)).sqrt(),
        _ => q * acosh(r / q),
    }
}

/// Inverse: surface radius r at height ẑ ≥ 0.
fn profile_r(profile: u32, q: f64, z: f64) -> f64 {
    let z = z.max(0.0);
    match profile {
        1 => q + z * z / (4.0 * q),
        _ => q * (z / q).cosh(),
    }
}

fn acosh(x: f64) -> f64 {
    (x + (x * x - 1.0).sqrt().max(0.0)).ln()
}

// ── mesh ──────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
struct Quad {
    p: [[f64; 3]; 4],
    /// normalized world height (−1..1) used for the color scheme
    u: f64,
}

#[derive(Clone, Copy)]
struct Seg {
    a: [f64; 3],
    b: [f64; 3],
    col: [f64; 3],
    wpx: f64,
    bias: f64,
}

pub struct ShapeCfg {
    pub profile: u32,
    pub q: f64,       // throat / sheet radius ratio
    pub stretch: f64, // vertical stretch of the tube
    pub weld: f64,    // 0 = two separate holed sheets … 1 = welded throat
    pub rings_half: u32,
    pub segs: u32,
}

fn weld_geometry(c: &ShapeCfg) -> (f64, f64) {
    // inner hole radius grows as the sheets are cut apart
    let r0 = c.q.max(0.35);
    let r_cut = c.q + (1.0 - c.weld) * (r0 - c.q);
    // vertical separation of the two sheets while unwelded
    let gap = (1.0 - c.weld) * 0.55;
    (r_cut, gap)
}

fn build_mesh(c: &ShapeCfg) -> (Vec<Quad>, Vec<Seg>) {
    let (r_cut, gap) = weld_geometry(c);
    let z_top = profile_z(c.profile, c.q, 1.0) * c.stretch;
    let extent = (z_top + gap).max(1e-6);

    let mut quads = Vec::new();
    let mut segs = Vec::new();
    let n = c.rings_half.max(2) as usize;
    let m = c.segs.max(3) as usize;

    for half in 0..2u32 {
        let sgn = if half == 0 { 1.0 } else { -1.0 };

        // ring radii, clustered toward the inner edge where curvature peaks
        let rs: Vec<f64> = (0..=n)
            .map(|i| {
                let s = i as f64 / n as f64;
                r_cut + (1.0 - r_cut) * s.powf(1.4)
            })
            .collect();

        let rings: Vec<Vec<[f64; 3]>> = rs
            .iter()
            .map(|&r| {
                let z = sgn * (profile_z(c.profile, c.q, r) * c.stretch + gap);
                (0..m)
                    .map(|j| {
                        let th = TAU * j as f64 / m as f64;
                        [r * th.cos(), r * th.sin(), z]
                    })
                    .collect()
            })
            .collect();

        for i in 0..n {
            for j in 0..m {
                let j2 = (j + 1) % m;
                let zm = (rings[i][j][2] + rings[i + 1][j2][2]) / 2.0;
                quads.push(Quad {
                    p: [rings[i][j], rings[i][j2], rings[i + 1][j2], rings[i + 1][j]],
                    u: (zm / extent).clamp(-1.0, 1.0),
                });
            }
        }

        // inner rim (glowing while the sheets are cut apart) + outer outline
        for j in 0..m {
            let j2 = (j + 1) % m;
            if c.weld < 0.999 {
                segs.push(Seg {
                    a: rings[0][j],
                    b: rings[0][j2],
                    col: [1.0, 0.86, 0.35],
                    wpx: 2.5,
                    bias: -0.02,
                });
            }
            segs.push(Seg {
                a: rings[n][j],
                b: rings[n][j2],
                col: [0.30, 0.36, 0.46],
                wpx: 1.2,
                bias: -0.02,
            });
        }
    }
    (quads, segs)
}

// ── colors ────────────────────────────────────────────────────────────────

fn mix(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

/// The screenshot palette: green top sheet, yellow upper throat,
/// periwinkle lower throat, soft-red bottom sheet.
fn classic(u: f64) -> [f64; 3] {
    let yellow = [0.98, 0.87, 0.35];
    let green = [0.55, 0.93, 0.65];
    let indigo = [0.51, 0.55, 0.96];
    let red = [0.98, 0.64, 0.64];
    if u >= 0.0 {
        mix(yellow, green, u / 0.55)
    } else {
        mix(indigo, red, -u / 0.55)
    }
}

/// Cool cosine palette keyed to height (for the "spectrum" mode).
fn spectrum(u: f64) -> [f64; 3] {
    let t = u * 0.9;
    [
        0.45 + 0.40 * (TAU * t).cos(),
        0.45 + 0.40 * (TAU * t + 2.1).cos(),
        0.55 + 0.40 * (TAU * t + 4.2).cos(),
    ]
}

// ── render ────────────────────────────────────────────────────────────────

/// Render one frame.
///
/// Returns a flat draw list of tagged records, sorted far→near (painter's
/// algorithm), for the JS side to replay:
///
/// * quad: `[0, depth, x1,y1, x2,y2, x3,y3, x4,y4, r,g,b]` (13 floats)
/// * line: `[1, depth, x1,y1, x2,y2, r,g,b, width]`        (10 floats)
/// * dot:  `[2, depth, x, y, radius, r,g,b]`               (8 floats)
///
/// `traveler_t` < 0 disables the falling-traveler particle.
pub fn render(
    w: u32,
    h: u32,
    yaw: f64,
    pitch: f64,
    zoom: f64,
    c: &ShapeCfg,
    color_mode: u32,
    traveler_t: f64,
) -> Vec<f64> {
    let (quads, segs) = build_mesh(c);

    let (cyaw, syaw) = (yaw.cos(), yaw.sin());
    let (cp, sp) = (pitch.cos(), pitch.sin());
    let (cam_d, f) = (3.6f64, 3.0f64);
    let s = 0.50 * (w.min(h)) as f64 * zoom;
    let cx = w as f64 / 2.0;
    let cyc = h as f64 / 2.0;

    // world → view: yaw about z, then tilt so `pitch` is the camera's
    // elevation above the sheet plane (like the textbook figures),
    // then perspective project.
    let view = |p: &[f64; 3]| -> [f64; 3] {
        let x1 = p[0] * cyaw - p[1] * syaw;
        let y1 = p[0] * syaw + p[1] * cyaw;
        [x1, y1 * sp + p[2] * cp, p[2] * sp - y1 * cp]
    };
    let proj = |v: &[f64; 3]| -> (f64, f64, f64) {
        let dv = cam_d - v[2];
        let k = f / (f + dv);
        (cx + v[0] * k * s, cyc - v[1] * k * s, dv)
    };

    // light fixed in view space: from upper-left, toward the camera
    let l = norm3([-0.40, 0.55, 0.72]);

    let mut recs: Vec<(f64, Vec<f64>)> = Vec::with_capacity(quads.len() + segs.len() + 16);

    for q in &quads {
        let v0 = view(&q.p[0]);
        let v1 = view(&q.p[1]);
        let v2 = view(&q.p[2]);
        let v3 = view(&q.p[3]);
        let (x0, y0, d0) = proj(&v0);
        let (x1, y1, d1) = proj(&v1);
        let (x2, y2, _) = proj(&v2);
        let (x3, y3, d3) = proj(&v3);
        let depth = (d0 + d1 + d3) / 3.0;

        // two-sided Lambert shading from the view-space normal
        let n = norm3(cross3(sub3(v1, v0), sub3(v3, v0)));
        let shade = 0.42 + 0.58 * dot3(n, l).abs();
        let base = if color_mode == 0 { classic(q.u) } else { spectrum(q.u) };
        let col = [base[0] * shade, base[1] * shade, base[2] * shade];

        recs.push((
            depth,
            vec![0.0, depth, x0, y0, x1, y1, x2, y2, x3, y3, col[0] * 255.0, col[1] * 255.0, col[2] * 255.0],
        ));
    }

    for sg in &segs {
        let va = view(&sg.a);
        let vb = view(&sg.b);
        let (xa, ya, da) = proj(&va);
        let (xb, yb, db) = proj(&vb);
        recs.push((
            (da + db) / 2.0 + sg.bias,
            vec![1.0, (da + db) / 2.0 + sg.bias, xa, ya, xb, yb, sg.col[0] * 255.0, sg.col[1] * 255.0, sg.col[2] * 255.0, sg.wpx],
        ));
    }

    // ── traveler: a particle sliding along the surface through the throat ──
    // (only makes sense once the manifold is actually welded together)
    if traveler_t >= 0.0 && c.weld > 0.999 {
        let (r_cut, gap) = weld_geometry(c);
        let amp = (1.0 - r_cut * 0.35).min(0.97); // base-height amplitude
        let bg = [14.0, 16.0, 32.0];
        for k in (0..=20).rev() {
            let tt = traveler_t - k as f64 * 0.035;
            let zb = amp * profile_z(c.profile, c.q, 0.95) * (tt * 0.8).cos();
            let sgn = if zb >= 0.0 { 1.0 } else { -1.0 };
            let r = profile_r(c.profile, c.q, zb.abs()).min(0.985);
            let th = tt * 1.9;
            let p = [r * th.cos(), r * th.sin(), sgn * (zb.abs() * c.stretch + gap)];
            let v = view(&p);
            let (x, y, d) = proj(&v);
            let kpersp = f / (f + d);
            let fade = 1.0 - k as f64 / 22.0;
            let glow = mix([bg[0] / 255.0, bg[1] / 255.0, bg[2] / 255.0], [1.0, 0.72, 0.20], fade);
            let core = mix([bg[0] / 255.0, bg[1] / 255.0, bg[2] / 255.0], [1.0, 0.98, 0.90], fade);
            let rad = 0.038 * kpersp * s * (1.0 + 0.5 * (1.0 - fade));
            recs.push((d - 0.001, vec![2.0, d - 0.001, x, y, rad, glow[0] * 255.0, glow[1] * 255.0, glow[2] * 255.0]));
            if k == 0 {
                recs.push((d - 0.002, vec![2.0, d - 0.002, x, y, rad * 0.5, core[0] * 255.0, core[1] * 255.0, core[2] * 255.0]));
            }
        }
    }

    // painter's algorithm: far first
    recs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut out = Vec::with_capacity(recs.len() * 12);
    for (_, rec) in recs {
        out.extend(rec);
    }
    out
}

fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn norm3(a: [f64; 3]) -> [f64; 3] {
    let m = dot3(a, a).sqrt().max(1e-12);
    [a[0] / m, a[1] / m, a[2] / m]
}

// ── tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(weld: f64) -> ShapeCfg {
        ShapeCfg { profile: 0, q: 0.3, stretch: 1.4, weld, rings_half: 8, segs: 24 }
    }

    #[test]
    fn profile_roundtrip() {
        for q in [0.1, 0.3, 0.5] {
            for prof in [0u32, 1] {
                // both profiles start at the throat: ẑ(q) = 0
                assert!(profile_z(prof, q, q) < 1e-12);
                for r in [q, q + 0.1, 0.7, 1.0] {
                    let z = profile_z(prof, q, r);
                    let r2 = profile_r(prof, q, z);
                    assert!((r2 - r).abs() < 1e-9, "roundtrip prof={} q={} r={}", prof, q, r);
                }
                // monotone in r
                assert!(profile_z(prof, q, 1.0) > profile_z(prof, q, q + 0.1));
            }
        }
    }

    #[test]
    fn weld_zero_separates_sheets() {
        let (quads, _) = build_mesh(&cfg(0.0));
        let (mut top_min, mut bot_max) = (f64::INFINITY, f64::NEG_INFINITY);
        for q in &quads {
            for p in &q.p {
                if p[2] >= 0.0 { top_min = top_min.min(p[2]); } else { bot_max = bot_max.max(p[2]); }
            }
        }
        assert!(top_min > 0.0 && bot_max < 0.0 && top_min - bot_max > 0.5, "gap");
    }

    #[test]
    fn weld_one_is_continuous() {
        let (quads, _) = build_mesh(&cfg(1.0));
        let (mut top_min, mut bot_max) = (f64::INFINITY, f64::NEG_INFINITY);
        for q in &quads {
            for p in &q.p {
                // corners at exactly z=0 belong to both halves (the weld seam)
                if p[2] >= 0.0 { top_min = top_min.min(p[2]); }
                if p[2] <= 0.0 { bot_max = bot_max.max(p[2]); }
            }
        }
        assert!((top_min - bot_max).abs() < 1e-9, "halves must meet at the throat");
    }

    #[test]
    fn drawlist_sorted_bounded_and_parseable() {
        let dl = render(800, 600, 0.7, 0.5, 1.0, &cfg(1.0), 0, -1.0);
        assert!(!dl.is_empty());
        let mut i = 0usize;
        let mut last = f64::INFINITY;
        let mut nquads = 0usize;
        while i < dl.len() {
            let tag = dl[i] as u32;
            let n = match tag { 0 => 13, 1 => 10, _ => 8 };
            let d = dl[i + 1];
            assert!(d <= last + 1e-9, "painter order violated at {}", i);
            last = d;
            if tag == 0 {
                nquads += 1;
                for v in &dl[i + 2..i + 10] {
                    assert!(*v > -4000.0 && *v < 4800.0, "coord out of bounds");
                }
            }
            i += n;
        }
        assert_eq!(i, dl.len(), "draw list must parse exactly");
        assert_eq!(nquads, 2 * 8 * 24, "two halves × rings × segs");
    }

    #[test]
    fn traveler_stays_on_surface() {
        // the traveler's radius must remain within the sheet for all times
        for t in (0..200).map(|i| i as f64 * 0.13) {
            let zb = 0.97 * profile_z(0, 0.3, 0.95) * (t * 0.8).cos();
            let r = profile_r(0, 0.3, zb.abs()).min(0.985);
            assert!((0.29..=0.99).contains(&r));
        }
    }
}
