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
//! A second view mode maps the same surface *isometrically* onto a flat
//! annulus ("collar chart"): θ is preserved and height is replaced by arc
//! length s(r) along the meridian, so the whole manifold unrolls into one
//! ring — inner rim = bottom sheet edge, middle circle = throat, outer rim =
//! top sheet edge. Colors and the traveler are mapped through the same
//! transform.
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

/// Arc length along the meridian from the throat (r = q) out to radius r.
/// Closed forms of ∫ √(1 + (dẑ/dr)²) dr:
///
/// * Ellis:  s(r) = √(r² − q²)
/// * Flamm:  s(r) = √(r(r−q)) + q·ln((√r + √(r−q)) / √q)
fn arc_len(profile: u32, q: f64, r: f64) -> f64 {
    let r = r.max(q);
    match profile {
        1 => {
            let a = (r * (r - q)).sqrt();
            let b = q * ((r.sqrt() + (r - q).sqrt()) / q.sqrt()).ln();
            a + b
        }
        _ => (r * r - q * q).sqrt().max(0.0),
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

#[derive(Clone, Copy)]
struct Dot {
    p: [f64; 3],
    /// world-space radius
    rad: f64,
    col: [f64; 3],
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

/// ring radii, clustered toward the inner edge where curvature peaks
fn ring_radii(c: &ShapeCfg, r_cut: f64) -> Vec<f64> {
    let n = c.rings_half.max(2) as usize;
    (0..=n)
        .map(|i| {
            let s = i as f64 / n as f64;
            r_cut + (1.0 - r_cut) * s.powf(1.4)
        })
        .collect()
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
        let rs = ring_radii(c, r_cut);

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

// ── flat annulus ("collar chart") ─────────────────────────────────────────
// θ is preserved; height is replaced by meridian arc length s(r), so every
// meridian is unrolled isometrically onto a radial line. The full manifold
// becomes one annulus: inner rim = bottom sheet edge, throat circle in the
// middle, outer rim = top sheet edge. While unwelded the annulus splits into
// two concentric rings with a gap (same story as the 3D weld morph).

struct ChartGeo {
    s1: f64,     // meridian arc length, throat → rim
    gap_c: f64,  // radial gap while unwelded
    rho_in: f64, // inner radius (bottom rim)
    rho_th: f64, // throat circle
    r_out: f64,  // outer radius (top rim)
}

fn chart_geo(c: &ShapeCfg) -> ChartGeo {
    let s1 = arc_len(c.profile, c.q, 1.0);
    let gap_c = (1.0 - c.weld) * 0.18 * s1;
    let rho_in = 0.55 * s1;
    let rho_th = rho_in + s1;
    let r_out = rho_in + 2.0 * s1 + 2.0 * gap_c;
    ChartGeo { s1, gap_c, rho_in, rho_th, r_out }
}

/// flat radius (normalized to the outer rim = 1) for surface radius r
fn chart_rho(c: &ShapeCfg, g: &ChartGeo, r: f64, top: bool) -> f64 {
    let s = arc_len(c.profile, c.q, r);
    let rho = if top {
        g.rho_th + g.gap_c + s
    } else {
        g.rho_th - g.gap_c - s
    };
    rho / g.r_out
}

fn build_chart(c: &ShapeCfg, reversed: bool) -> (Vec<Quad>, Vec<Seg>) {
    let (r_cut, gap) = weld_geometry(c);
    let g = chart_geo(c);
    let z_top = profile_z(c.profile, c.q, 1.0) * c.stretch;
    let extent = (z_top + gap).max(1e-6);

    let mut quads = Vec::new();
    let mut segs = Vec::new();
    let n = c.rings_half.max(2) as usize;
    let m = c.segs.max(3) as usize;

    for half in 0..2u32 {
        let top = half == 0;
        let sgn = if top { 1.0 } else { -1.0 };
        let rs = ring_radii(c, r_cut);

        let rings: Vec<Vec<[f64; 3]>> = rs
            .iter()
            .map(|&r| {
                // reversed chart: top sheet hugs the inner rim,
                // bottom sheet forms the outer rim
                let rho = chart_rho(c, &g, r, top != reversed);
                (0..m)
                    .map(|j| {
                        let th = TAU * j as f64 / m as f64;
                        [rho * th.cos(), rho * th.sin(), 0.0]
                    })
                    .collect()
            })
            .collect();

        for i in 0..n {
            for j in 0..m {
                let j2 = (j + 1) % m;
                let rm = (rs[i] + rs[i + 1]) / 2.0;
                let u = (sgn * (profile_z(c.profile, c.q, rm) * c.stretch + gap) / extent)
                    .clamp(-1.0, 1.0);
                quads.push(Quad {
                    p: [rings[i][j], rings[i][j2], rings[i + 1][j2], rings[i + 1][j]],
                    u,
                });
            }
        }

        // outlines: outer/inner rims solid dark; inner edges glow while cut
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

    // throat circle: dashed pale ring when welded
    if c.weld > 0.999 {
        let rho = g.rho_th / g.r_out;
        for j in (0..m).step_by(2) {
            let t0 = TAU * j as f64 / m as f64;
            let t1 = TAU * (j + 1) as f64 / m as f64;
            segs.push(Seg {
                a: [rho * t0.cos(), rho * t0.sin(), 0.0],
                b: [rho * t1.cos(), rho * t1.sin(), 0.0],
                col: [0.92, 0.95, 1.0],
                wpx: 1.4,
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

struct Trig {
    cyaw: f64,
    syaw: f64,
    cp: f64,
    sp: f64,
}

#[allow(clippy::too_many_arguments)]
fn emit(
    out: &mut Vec<(f64, Vec<f64>)>,
    quads: &[Quad],
    segs: &[Seg],
    dots: &[Dot],
    shaded: bool,
    t: &Trig,
    light: [f64; 3],
    cx: f64,
    cyc: f64,
    s: f64,
    color_mode: u32,
    offx: f64,
    offy: f64,
) {
    let view = |p: &[f64; 3]| -> [f64; 3] {
        let x1 = p[0] * t.cyaw - p[1] * t.syaw;
        let y1 = p[0] * t.syaw + p[1] * t.cyaw;
        let v = [x1, y1 * t.sp + p[2] * t.cp, p[2] * t.sp - y1 * t.cp];
        // view-space offset: stays "below/above" on screen at any yaw/pitch
        [v[0] + offx, v[1] + offy, v[2]]
    };
    let (f, cam_d) = (3.0f64, 3.6f64);
    let proj = |v: &[f64; 3]| -> (f64, f64, f64) {
        let dv = cam_d - v[2];
        let k = f / (f + dv);
        (cx + v[0] * k * s, cyc - v[1] * k * s, dv)
    };

    for q in quads {
        let v0 = view(&q.p[0]);
        let v1 = view(&q.p[1]);
        let v2 = view(&q.p[2]);
        let v3 = view(&q.p[3]);
        let (x0, y0, d0) = proj(&v0);
        let (x1, y1, d1) = proj(&v1);
        let (x2, y2, _) = proj(&v2);
        let (x3, y3, d3) = proj(&v3);
        let depth = (d0 + d1 + d3) / 3.0;

        let shade = if shaded {
            let n = norm3(cross3(sub3(v1, v0), sub3(v3, v0)));
            0.42 + 0.58 * dot3(n, light).abs()
        } else {
            1.0
        };
        let base = if color_mode == 0 { classic(q.u) } else { spectrum(q.u) };
        let col = [base[0] * shade, base[1] * shade, base[2] * shade];

        out.push((
            depth,
            vec![0.0, depth, x0, y0, x1, y1, x2, y2, x3, y3, col[0] * 255.0, col[1] * 255.0, col[2] * 255.0],
        ));
    }

    for sg in segs {
        let va = view(&sg.a);
        let vb = view(&sg.b);
        let (xa, ya, da) = proj(&va);
        let (xb, yb, db) = proj(&vb);
        let depth = (da + db) / 2.0 + sg.bias;
        out.push((
            depth,
            vec![1.0, depth, xa, ya, xb, yb, sg.col[0] * 255.0, sg.col[1] * 255.0, sg.col[2] * 255.0, sg.wpx],
        ));
    }

    for dt in dots {
        let v = view(&dt.p);
        let (x, y, d) = proj(&v);
        let k = f / (f + d);
        let depth = d + dt.bias;
        out.push((
            depth,
            vec![2.0, depth, x, y, dt.rad * k * s, dt.col[0] * 255.0, dt.col[1] * 255.0, dt.col[2] * 255.0],
        ));
    }
}

/// Traveler path on the surface as (radius r, angle θ, world z) samples,
/// head first. Empty when disabled or the manifold is unwelded.
fn traveler_path(c: &ShapeCfg, traveler_t: f64) -> Vec<(f64, f64, f64)> {
    if traveler_t < 0.0 || c.weld <= 0.999 {
        return vec![];
    }
    let (r_cut, gap) = weld_geometry(c);
    let amp = (1.0 - r_cut * 0.35).min(0.97);
    let mut pts = Vec::with_capacity(21);
    for k in 0..=20 {
        let tt = traveler_t - k as f64 * 0.035;
        let zb = amp * profile_z(c.profile, c.q, 0.95) * (tt * 0.8).cos();
        let sgn = if zb >= 0.0 { 1.0 } else { -1.0 };
        let r = profile_r(c.profile, c.q, zb.abs()).min(0.985);
        let th = tt * 1.9;
        let wz = sgn * (zb.abs() * c.stretch + gap);
        pts.push((r, th, wz));
    }
    pts
}

fn traveler_dots(path: &[(f64, f64, f64)], map: impl Fn(f64, f64, f64) -> [f64; 3]) -> Vec<Dot> {
    let bg = [14.0 / 255.0, 16.0 / 255.0, 32.0 / 255.0];
    let mut dots = Vec::with_capacity(path.len() + 1);
    for (k, &(r, th, wz)) in path.iter().enumerate() {
        let fade = 1.0 - k as f64 / 22.0;
        let col = if k == 0 {
            mix(bg, [1.0, 0.98, 0.90], fade)
        } else {
            mix(bg, [1.0, 0.72, 0.20], fade)
        };
        let rad = if k == 0 { 0.020 } else { 0.038 * (1.0 + 0.5 * (1.0 - fade)) };
        dots.push(Dot { p: map(r, th, wz), rad, col, bias: -0.001 });
    }
    dots
}

/// Render one frame.
///
/// `view_mode`: 0 = 3D embedding only, 1 = flat annulus charts only
/// (normal mapping + radially reversed, stacked), 2 = 3D + charts side by side.
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
    view_mode: u32,
    traveler_t: f64,
) -> Vec<f64> {
    let t = Trig {
        cyaw: yaw.cos(),
        syaw: yaw.sin(),
        cp: pitch.cos(),
        sp: pitch.sin(),
    };
    // light fixed in view space: from upper-left, toward the camera
    let light = norm3([-0.40, 0.55, 0.72]);

    let mut recs: Vec<(f64, Vec<f64>)> = Vec::new();

    let s_full = 0.50 * (w.min(h)) as f64 * zoom;
    let cx = w as f64 / 2.0;
    let cyc = h as f64 / 2.0;

    let path = traveler_path(c, traveler_t);

    if view_mode == 0 || view_mode == 2 {
        let (quads, segs) = build_mesh(c);
        let dots = traveler_dots(&path, |r, th, wz| [r * th.cos(), r * th.sin(), wz]);
        let (s, ccx) = if view_mode == 2 { (s_full * 0.52, w as f64 * 0.27) } else { (s_full, cx) };
        emit(&mut recs, &quads, &segs, &dots, true, &t, light, ccx, cyc, s, color_mode, 0.0, 0.0);
    }

    if view_mode == 1 || view_mode == 2 {
        // two flat charts stacked vertically: normal mapping on top,
        // radially reversed (bottom sheet = outer rim) below it
        let (s_ch, ccx) = if view_mode == 2 {
            (s_full * 0.50, w as f64 * 0.73)
        } else {
            (s_full * 0.60, cx)
        };
        let d = 1.18; // chart outer radius is 1, so this leaves a gap
        for reversed in [false, true] {
            let (quads, segs) = build_chart(c, reversed);
            let offy = if reversed { -d } else { d };
            let dots = traveler_dots(&path, |r, th, wz| {
                let g = chart_geo(c);
                let rho = chart_rho(c, &g, r, (wz >= 0.0) != reversed);
                [rho * th.cos(), rho * th.sin(), 0.0]
            });
            emit(&mut recs, &quads, &segs, &dots, false, &t, light, ccx, cyc, s_ch, color_mode, 0.0, offy);
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
    fn arc_length_matches_numeric() {
        // integrate √(1+(dẑ/dr)²) with the substitution r = q+(1−q)u² to
        // remove the throat singularity, and compare with the closed forms
        for prof in [0u32, 1] {
            for q in [0.15, 0.3, 0.45] {
                // after the substitution r = q+(1−q)u² the integrand
                // √(1+(dẑ/dr)²)·dr/du is nonsingular:
                //   Ellis: 2r√(1−q)/√(r+q)   Flamm: 2√(r(1−q))
                let f = |u: f64| {
                    let r = q + (1.0 - q) * u * u;
                    match prof {
                        1 => 2.0 * (r * (1.0 - q)).sqrt(),
                        _ => 2.0 * r * (1.0 - q).sqrt() / (r + q).sqrt(),
                    }
                };
                let n = 2000usize;
                let hstep = 1.0 / n as f64;
                let mut sum = f(0.0) + f(1.0);
                for i in 1..n {
                    sum += if i % 2 == 1 { 4.0 } else { 2.0 } * f(i as f64 * hstep);
                }
                let num = sum * hstep / 3.0;
                let closed = arc_len(prof, q, 1.0);
                assert!((num - closed).abs() < 1e-6, "prof={} q={} num={} closed={}", prof, q, num, closed);
            }
        }
        // and s(q) = 0
        for prof in [0u32, 1] {
            assert!(arc_len(prof, 0.3, 0.3) < 1e-12);
        }
    }

    #[test]
    fn chart_radii_monotone_and_bounded() {
        for prof in [0u32, 1] {
            let c = cfg(1.0);
            let c = ShapeCfg { profile: prof, ..c };
            let g = chart_geo(&c);
            // top half: rho grows from throat (≈ middle) to 1 at the rim
            let mut prev = 0.0;
            for r in [0.3, 0.5, 0.7, 0.9, 1.0] {
                let rho = chart_rho(&c, &g, r, true);
                assert!(rho > prev);
                prev = rho;
            }
            assert!((prev - 1.0).abs() < 1e-9, "top rim normalizes to 1");
            // bottom half: rho shrinks from throat to rho_in/r_out at the rim
            let mut prev = 2.0;
            for r in [0.3, 0.5, 0.7, 0.9, 1.0] {
                let rho = chart_rho(&c, &g, r, false);
                assert!(rho < prev);
                prev = rho;
            }
            assert!(prev > 0.0, "inner rim must stay positive");
            // throat sits strictly between the rims
            let rt = g.rho_th / g.r_out;
            assert!(rt > prev && rt < 1.0);
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

    fn check_drawlist(dl: &[f64]) {
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
        assert!(nquads > 300);
    }

    #[test]
    fn drawlist_sorted_bounded_and_parseable_all_modes() {
        for mode in [0u32, 1, 2] {
            let dl = render(800, 600, 0.7, 0.5, 1.0, &cfg(1.0), 0, mode, -1.0);
            check_drawlist(&dl);
        }
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

    #[test]
    fn traveler_mapped_to_chart_annulus() {
        let c = cfg(1.0);
        let g = chart_geo(&c);
        let path = traveler_path(&c, 3.7);
        assert!(!path.is_empty());
        for &(r, _th, wz) in &path {
            let rho = chart_rho(&c, &g, r, wz >= 0.0);
            assert!((g.rho_in / g.r_out..=1.0).contains(&rho), "traveler inside annulus");
        }
    }
}
