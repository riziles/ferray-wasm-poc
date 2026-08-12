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
    /// radius of the quad's inner ring (distance from the axis)
    r_in: f64,
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
    pub show_collars: bool, // tint the collar neighborhoods of the holes
    pub charts_mode: u32,   // 0 = one chart, 1 = two charts with overlap
    pub show_seam: bool,    // dashed ring on the glued boundary circle
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
                    r_in: rs[i],
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

// ── collar neighborhoods & chart colors ───────────────────────────────────
// The connected-sum construction glues the two *collars*: the innermost
// annulus [r_cut, r_cut + COLLAR_FRAC·(1 − r_cut)] just inside each boundary
// circle. The didactic toggles below let you see those cuffs, the identified
// circle itself (the seam), and the same finished space as two charts whose
// patches overlap exactly on the glued collar band.

const COLLAR_FRAC: f64 = 0.10;

fn collar_radius(r_cut: f64) -> f64 {
    r_cut + COLLAR_FRAC * (1.0 - r_cut)
}

/// 1 at the boundary circle, fading to 0 at the collar's outer edge.
fn collar_fade(r: f64, r_cut: f64) -> f64 {
    ((collar_radius(r_cut) - r) / (COLLAR_FRAC * (1.0 - r_cut)).max(1e-9)).clamp(0.0, 1.0)
}

/// Chart U₁ / U₂ tints for the "two charts" view: the welded surface is one
/// space; U₁ covers the upper half, U₂ the lower half, and the two charts
/// overlap exactly on the glued collar band (checkerboarded there).
const CHART_TOP: [f64; 3] = [0.35, 0.85, 0.95];
const CHART_BOT: [f64; 3] = [0.95, 0.55, 0.65];

/// Collar "cuff" tints — the material that actually gets glued: bright
/// yellow above, violet below, matching the prepared rims of the classic
/// two-chart construction figure.
const COLLAR_TOP: [f64; 3] = [1.0, 0.88, 0.30];
const COLLAR_BOT: [f64; 3] = [0.75, 0.62, 1.0];

/// Neutral body color of the sheets in panel 2 (kept light enough to read
/// as solid planes against the dark background).
const PLANE_BODY: [f64; 3] = [0.24, 0.28, 0.42];

/// Rainbow hue at h ∈ [0, 1) — the identification color mapping: each hue
/// around the boundary circle of one sheet is declared "the same point" as
/// the matching hue on the other sheet.
fn hue_rgb(h: f64) -> [f64; 3] {
    let h = h - h.floor();
    let x = 1.0 - ((h * 6.0) % 2.0 - 1.0).abs();
    let (r, g, b) = match (h * 6.0) as u32 {
        0 => (1.0, x, 0.0),
        1 => (x, 1.0, 0.0),
        2 => (0.0, 1.0, x),
        3 => (0.0, x, 1.0),
        4 => (x, 0.0, 1.0),
        _ => (1.0, 0.0, x),
    };
    [r, g, b]
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
///
/// `view` selects which illustration to draw:
/// * 0 — panel 1: the two flat sheets, cut apart (two charts, before gluing)
/// * 1 — panel 2: the same sheets with the collar identification color map
/// * 2 — panel 3: the smooth 3D embedding (one chart)
pub fn render(
    w: u32,
    h: u32,
    yaw: f64,
    pitch: f64,
    zoom: f64,
    c: &ShapeCfg,
    color_mode: u32,
    traveler_t: f64,
    view: u32,
) -> Vec<f64> {
    if view <= 1 {
        return render_planes(w, h, yaw, pitch, zoom, c, view, traveler_t);
    }
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

    let (r_cut, _) = weld_geometry(c);
    let per_half = (c.rings_half.max(2) * c.segs.max(3)) as usize;
    let m = c.segs.max(3) as usize;

    for (idx, q) in quads.iter().enumerate() {
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
        let in_collar = q.r_in < collar_radius(r_cut);

        // optional didactic recolorings (they change surface colors only)
        let mut col = base;
        if c.charts_mode == 1 {
            let within = idx % per_half;
            let (ring, seg) = (within / m, within % m);
            if in_collar {
                // the overlap strip: both charts cover the glued collar band
                col = if (ring + seg) % 2 == 0 {
                    mix(base, CHART_TOP, 0.45)
                } else {
                    mix(base, CHART_BOT, 0.45)
                };
            } else if q.u >= 0.0 {
                col = mix(base, CHART_TOP, 0.45); // chart U₁ covers the upper half
            } else {
                col = mix(base, CHART_BOT, 0.45); // chart U₂ covers the lower half
            }
        } else if c.show_collars && in_collar {
            let t = collar_fade(q.r_in, r_cut);
            let tint = if q.u >= 0.0 { COLLAR_TOP } else { COLLAR_BOT };
            col = mix(base, tint, 0.85 * t);
        }
        let lit = [col[0] * shade, col[1] * shade, col[2] * shade];

        recs.push((
            depth,
            vec![0.0, depth, x0, y0, x1, y1, x2, y2, x3, y3, lit[0] * 255.0, lit[1] * 255.0, lit[2] * 255.0],
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

    // ── glued seam: the identified boundary circle (dashed ring) ──
    // The throat is where the two collar neighborhoods were glued; the seam
    // marks that identified circle S¹ so the construction stays visible in
    // the finished manifold instead of being swept under the rug.
    if c.show_seam && c.weld > 0.999 {
        const DASHES: u32 = 24;
        let dash = TAU / DASHES as f64 / 2.0;
        for k in 0..DASHES {
            let a0 = k as f64 * 2.0 * dash;
            let a1 = a0 + dash;
            let va = view(&[r_cut * a0.cos(), r_cut * a0.sin(), 0.0]);
            let vb = view(&[r_cut * a1.cos(), r_cut * a1.sin(), 0.0]);
            let (xa, ya, da) = proj(&va);
            let (xb, yb, db) = proj(&vb);
            let d = (da + db) / 2.0 - 0.0005;
            recs.push((d, vec![1.0, d, xa, ya, xb, yb, 0.85 * 255.0, 0.97 * 255.0, 1.0 * 255.0, 2.2]));
        }
    }

    // ── traveler: a particle sliding along the surface through the throat ──
    // (only makes sense once the manifold is actually welded together)
    if traveler_t >= 0.0 && c.weld > 0.999 {
        let (r_cut, gap) = weld_geometry(c);
        let amp = (1.0 - r_cut * 0.35).min(0.97); // base-height amplitude
        for k in (0..=10).rev() {
            let tt = traveler_t - k as f64 * 0.05;
            let zb = amp * profile_z(c.profile, c.q, 0.95) * (tt * 0.8).cos();
            let sgn = if zb >= 0.0 { 1.0 } else { -1.0 };
            let r = profile_r(c.profile, c.q, zb.abs()).min(0.985);
            let th = tt * 1.9;
            let p = [r * th.cos(), r * th.sin(), sgn * (zb.abs() * c.stretch + gap)];
            let v = view(&p);
            let (x, y, d) = proj(&v);
            let kpersp = f / (f + d);
            let fade = 1.0 - k as f64 / 11.0;
            // short comet trail: fades to a warm dim orange, never to the
            // background (which read as dark smudges on the surface)
            let glow = mix([0.45, 0.32, 0.12], [1.0, 0.72, 0.20], fade);
            let core = mix([0.55, 0.45, 0.25], [1.0, 0.98, 0.90], fade);
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

// ── panels 1 & 2: the two-chart pictures ─────────────────────────────────
// Two flat parallel planes (annuli) with the boundary circles that get
// glued. Panel 1 shows them plain, with glowing rims. Panel 2 paints the
// collar neighborhoods with an orientation-reversing hue mapping — the
// colors run clockwise on the top rim and counter-clockwise on the bottom,
// so matching colors are the points that get identified. Inside the collar
// the traveler appears on BOTH sheets (the two charts overlap there) with
// a dashed connector; outside it, only on the sheet it's actually on.

fn render_planes(
    w: u32,
    h: u32,
    yaw: f64,
    pitch: f64,
    zoom: f64,
    c: &ShapeCfg,
    view: u32,
    traveler_t: f64,
) -> Vec<f64> {
    let q = c.q.max(0.05);
    let gap = 0.85; // fixed separation: this is an illustration, not the weld morph
    let n = 6usize; // flat annuli need very few rings
    let m = c.segs.max(3) as usize;

    let (cyaw, syaw) = (yaw.cos(), yaw.sin());
    let (cp, sp) = (pitch.cos(), pitch.sin());
    let (cam_d, f) = (3.6f64, 3.0f64);
    let s = 0.50 * (w.min(h)) as f64 * zoom * 0.9;
    let cx = w as f64 / 2.0;
    let cyc = h as f64 / 2.0;
    let view3 = |p: &[f64; 3]| -> [f64; 3] {
        let x1 = p[0] * cyaw - p[1] * syaw;
        let y1 = p[0] * syaw + p[1] * cyaw;
        [x1, y1 * sp + p[2] * cp, p[2] * sp - y1 * cp]
    };
    let proj = |v: &[f64; 3]| -> (f64, f64, f64) {
        let dv = cam_d - v[2];
        let k = f / (f + dv);
        (cx + v[0] * k * s, cyc - v[1] * k * s, dv)
    };
    let l = norm3([-0.40, 0.55, 0.72]);

    let mut recs: Vec<(f64, Vec<f64>)> = Vec::new();

    // uniform ring radii from the rim q out to 1
    let rs: Vec<f64> = (0..=n).map(|i| q + (1.0 - q) * (i as f64 / n as f64)).collect();

    for half in 0..2u32 {
        let sgn = if half == 0 { 1.0 } else { -1.0 };
        let z0 = sgn * gap;
        let rings: Vec<Vec<[f64; 3]>> = rs
            .iter()
            .map(|&r| {
                (0..m)
                    .map(|j| {
                        let th = TAU * j as f64 / m as f64;
                        [r * th.cos(), r * th.sin(), z0]
                    })
                    .collect()
            })
            .collect();

        for i in 0..n {
            for j in 0..m {
                let j2 = (j + 1) % m;
                let th_mid = TAU * (j as f64 + 0.5) / m as f64;
                let r_mid = 0.5 * (rs[i] + rs[i + 1]);
                let mut col = if half == 0 { [0.55, 0.93, 0.65] } else { [0.98, 0.64, 0.64] };
                if view == 1 {
                    if r_mid < collar_radius(q) {
                        // the collar: identification gradient, fading outward from the rim
                        let t = collar_fade(r_mid, q);
                        let h = if half == 0 { th_mid / TAU } else { 1.0 - th_mid / TAU };
                        col = mix(PLANE_BODY, hue_rgb(h), 0.35 + 0.65 * t);
                    } else {
                        col = PLANE_BODY; // neutral body
                    }
                }
                let nrm = [0.0, 0.0, sgn as f64];
                let shade = 0.55 + 0.45 * dot3(nrm, l).abs();
                let lit = [col[0] * shade, col[1] * shade, col[2] * shade];

                let pts = [rings[i][j], rings[i][j2], rings[i + 1][j2], rings[i + 1][j]];
                let v0 = view3(&pts[0]);
                let v1 = view3(&pts[1]);
                let v2 = view3(&pts[2]);
                let v3 = view3(&pts[3]);
                let (x0, y0, d0) = proj(&v0);
                let (x1, y1, d1) = proj(&v1);
                let (x2, y2, _) = proj(&v2);
                let (x3, y3, d3) = proj(&v3);
                let depth = (d0 + d1 + d3) / 3.0;
                recs.push((
                    depth,
                    vec![0.0, depth, x0, y0, x1, y1, x2, y2, x3, y3, lit[0] * 255.0, lit[1] * 255.0, lit[2] * 255.0],
                ));
            }
        }

        // the boundary circle (the rim that gets glued)
        for j in 0..m {
            let j2 = (j + 1) % m;
            let th_mid = TAU * (j as f64 + 0.5) / m as f64;
            let col = if view == 0 {
                [1.0, 0.86, 0.35]
            } else if half == 0 {
                hue_rgb(th_mid / TAU)
            } else {
                hue_rgb(1.0 - th_mid / TAU)
            };
            let va = view3(&rings[0][j]);
            let vb = view3(&rings[0][j2]);
            let (xa, ya, da) = proj(&va);
            let (xb, yb, db) = proj(&vb);
            let d = (da + db) / 2.0 - 0.02;
            recs.push((d, vec![1.0, d, xa, ya, xb, yb, col[0] * 255.0, col[1] * 255.0, col[2] * 255.0, 2.4]));
        }

        // faint concentric rings so the sheets read as flat planes (panel 1 only)
        if view == 0 {
            for r in [0.6, 0.82] {
                for j in 0..m {
                    let j2 = (j + 1) % m;
                    let pa = [r * (TAU * j as f64 / m as f64).cos(), r * (TAU * j as f64 / m as f64).sin(), z0];
                    let pb = [r * (TAU * j2 as f64 / m as f64).cos(), r * (TAU * j2 as f64 / m as f64).sin(), z0];
                    let va = view3(&pa);
                    let vb = view3(&pb);
                    let (xa, ya, da) = proj(&va);
                    let (xb, yb, db) = proj(&vb);
                    let d = (da + db) / 2.0 - 0.015;
                    recs.push((d, vec![1.0, d, xa, ya, xb, yb, 0.25 * 255.0, 0.30 * 255.0, 0.42 * 255.0, 1.0]));
                }
            }
        }
    }

    // ── traveler on the two-chart pictures ──
    // Same world-space trajectory as panel 3. Outside the collar the traveler
    // lives on exactly one sheet (charts overlap only in the collar band);
    // inside it appears on both at the identified point, with a slight
    // dilation that vanishes at the glued circle (the charts agree there).
    // The copy and the dashed connector fade in with collar depth.
    //
    // Crucial continuity detail: the traveler's worldline is smooth. It
    // orbits at angle θ on the top sheet; the orientation-reversing glue
    // θ ↦ −θ means its continuation on the bottom sheet orbits at −θ. Each
    // sheet's dot therefore stays put through the crossing — the two dots
    // never swap places, which is what made the old version look like it
    // was jumping.
    if traveler_t >= 0.0 {
        let amp = (1.0 - q * 0.35).min(0.97);
        let zb = amp * profile_z(c.profile, q, 0.95) * (traveler_t * 0.8).cos();
        let r = profile_r(c.profile, q, zb.abs()).min(0.985);
        let th = traveler_t * 1.9; // the traveler's own (top-chart) angle
        // presence: 0 at the collar's outer edge → 1 at the boundary circle
        let p = ((collar_radius(q) - r) / (collar_radius(q) - q).max(1e-9)).clamp(0.0, 1.0);
        // dilation of the copy's reading; fades to 0 at the glued circle so
        // the two charts agree exactly there and nothing jumps at the crossing
        let wob = 0.15 * (traveler_t * 0.7).sin() * (1.0 - p);
        let r2 = (r * (0.94 + 0.06 * p)).max(q * 1.01);

        let (top_ang, bot_ang, top_pres, bot_pres) = if zb >= 0.0 {
            (th, -th + wob, 1.0, p)
        } else {
            (-th, th + wob, p, 1.0)
        };
        let dots: [([f64; 3], f64, f64); 2] = [
            // top sheet dot (hue in the top chart's convention)
            ([r * top_ang.cos(), r * top_ang.sin(), gap], top_ang / TAU, top_pres),
            // bottom sheet dot (hue in the bottom chart's convention, so
            // matching colors really mean the identified point)
            ([r2 * bot_ang.cos(), r2 * bot_ang.sin(), -gap], 1.0 - bot_ang / TAU, bot_pres),
        ];

        for (pnt, h, pres) in &dots {
            let v = view3(pnt);
            let (x, y, d) = proj(&v);
            let kpersp = f / (f + d);
            let rad = 0.045 * kpersp * s * pres;
            if rad > 0.05 {
                let col = if view == 1 { hue_rgb(*h) } else { [1.0, 0.72, 0.20] };
                recs.push((d - 0.002, vec![2.0, d - 0.002, x, y, rad, col[0] * 255.0, col[1] * 255.0, col[2] * 255.0]));
                recs.push((d - 0.003, vec![2.0, d - 0.003, x, y, rad * 0.45, 255.0, 250.0, 230.0]));
            }
        }

        // dashed connector: the two appearances are the same traveler; it
        // fades in/out with the overlap (brightness and width both ramp)
        if p > 0.0 {
            let pa = dots[0].0;
            let pb = dots[1].0;
            let mixc = |a: f64, b: f64| a + (b - a) * p;
            for k in 0..10 {
                let t0 = k as f64 / 10.0;
                let t1 = (k as f64 + 0.45) / 10.0;
                let lerp = |t: f64| -> [f64; 3] {
                    [pa[0] + (pb[0] - pa[0]) * t, pa[1] + (pb[1] - pa[1]) * t, pa[2] + (pb[2] - pa[2]) * t]
                };
                let va = view3(&lerp(t0));
                let vb = view3(&lerp(t1));
                let (xa, ya, da) = proj(&va);
                let (xb, yb, db) = proj(&vb);
                let d = (da + db) / 2.0 - 0.01;
                recs.push((d, vec![1.0, d, xa, ya, xb, yb, mixc(0.24, 1.0) * 255.0, mixc(0.28, 1.0) * 255.0, mixc(0.42, 1.0) * 255.0, 1.6 * p]));
            }
        }
    }

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
        ShapeCfg {
            profile: 0,
            q: 0.3,
            stretch: 1.4,
            weld,
            rings_half: 8,
            segs: 24,
            show_collars: false,
            charts_mode: 0,
            show_seam: false,
        }
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
        let dl = render(800, 600, 0.7, 0.5, 1.0, &cfg(1.0), 0, -1.0, 2);
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

    #[test]
    fn collar_band_is_annulus_near_the_hole() {
        let r_cut = 0.3;
        assert!(collar_radius(r_cut) > r_cut, "collar is a band, not a line");
        assert!(collar_fade(r_cut, r_cut) > 0.999, "strongest at the rim");
        assert!(collar_fade(1.0, r_cut) < 1e-9, "nothing at the outer edge");
        assert!(collar_fade((r_cut + collar_radius(r_cut)) / 2.0, r_cut) > 0.4);
    }

    #[test]
    fn didactic_modes_change_the_draw_list() {
        let plain = render(640, 480, 0.7, 0.5, 1.0, &cfg(1.0), 0, -1.0, 2);
        let mut c2 = cfg(1.0);
        c2.charts_mode = 1;
        let charts = render(640, 480, 0.7, 0.5, 1.0, &c2, 0, -1.0, 2);
        let mut c3 = cfg(1.0);
        c3.show_collars = true;
        let collars = render(640, 480, 0.7, 0.5, 1.0, &c3, 0, -1.0, 2);
        assert_ne!(plain, charts, "two-charts view must recolor the surface");
        assert_ne!(plain, collars, "collar highlight must recolor the surface");
    }

    #[test]
    fn seam_ring_appears_only_when_welded() {
        let count_lines = |dl: &[f64]| {
            let mut n = 0usize;
            let mut i = 0usize;
            while i < dl.len() {
                if dl[i] == 1.0 {
                    n += 1;
                }
                i += match dl[i] as u32 { 0 => 13, 1 => 10, _ => 8 };
            }
            n
        };
        let without = render(640, 480, 0.7, 0.5, 1.0, &cfg(1.0), 0, -1.0, 2);
        let mut c = cfg(1.0);
        c.show_seam = true;
        let with = render(640, 480, 0.7, 0.5, 1.0, &c, 0, -1.0, 2);
        let mut c0 = cfg(0.0);
        c0.show_seam = true;
        let unwelded = render(640, 480, 0.7, 0.5, 1.0, &c0, 0, -1.0, 2);
        // two outline rings of m segs each, plus the 24 dashed seam arcs
        assert_eq!(count_lines(&without), 2 * 24);
        assert_eq!(count_lines(&with), 2 * 24 + 24);
        // unwelded: glowing inner rims (2·m) + outlines (2·m), no seam dashes
        assert_eq!(count_lines(&unwelded), 4 * 24, "no seam while the sheets are cut apart");
    }

    #[test]
    fn hue_mapping_is_periodic() {
        assert_eq!(hue_rgb(0.0), [1.0, 0.0, 0.0], "red at h=0");
        assert_eq!(hue_rgb(1.0), [1.0, 0.0, 0.0], "hue wraps");
        assert_eq!(hue_rgb(1.0 / 6.0), [1.0, 1.0, 0.0], "yellow at h=1/6");
        assert_eq!(hue_rgb(0.5), [0.0, 1.0, 1.0], "cyan at h=1/2");
        // the identification θ ↦ −θ sends the top rim's color at θ to the
        // bottom rim's color at 1 − θ: the ORDER of the colors is reversed,
        // which is the point of the mapping (orientation-reversing glue)
        assert_eq!(hue_rgb(0.25), [0.5, 1.0, 0.0]);
        assert_eq!(hue_rgb(0.75), [0.5, 0.0, 1.0]);
        assert_ne!(hue_rgb(0.25), hue_rgb(0.75), "opposite angles get reversed colors");
    }

    #[test]
    fn planes_views_traveler_enters_collar() {
        let count = |dl: &[f64], tag: f64| {
            let mut n = 0usize;
            let mut i = 0usize;
            while i < dl.len() {
                if dl[i] == tag {
                    n += 1;
                }
                i += match dl[i] as u32 { 0 => 13, 1 => 10, _ => 8 };
            }
            n
        };
        // t=0: high above the sheets (r ≈ 0.76 > collar) — one chart sees it.
        // t=π/1.6: at the throat (r = q ≤ collar) — both charts see it.
        // (each dot emits two tag-2 records: glow + core)
        let t_in = std::f64::consts::PI / 1.6;
        for view in [0u32, 1] {
            let out = render(480, 360, 0.7, 0.5, 1.0, &cfg(1.0), 0, 0.0, view);
            let inn = render(480, 360, 0.7, 0.5, 1.0, &cfg(1.0), 0, t_in, view);
            assert_eq!(count(&out, 2.0), 2, "far from the hole: one chart sees the traveler");
            assert_eq!(count(&inn, 2.0), 4, "inside the collar: both charts see it");
            // dashed connector only while in the collar (view 0 adds two grid
            // rings on each sheet: 2·2·m, so 6·m lines; view 1: 2·m rims)
            let base_lines = if view == 0 { 6 * 24 } else { 2 * 24 };
            assert_eq!(count(&out, 1.0), base_lines, "no connector outside the collar");
            assert_eq!(count(&inn, 1.0), base_lines + 10, "dashed connector while in the collar");
        }
        // the plane views never include the 3D tube: exactly two flat annuli
        let dl = render(480, 360, 0.7, 0.5, 1.0, &cfg(1.0), 0, -1.0, 1);
        assert_eq!(count(&dl, 0.0), 2 * 6 * 24, "two flat annuli × 6 rings × 24 segs");
        let mut last = f64::INFINITY;
        let mut i = 0usize;
        while i < dl.len() {
            let d = dl[i + 1];
            assert!(d <= last + 1e-9, "painter order violated");
            last = d;
            i += match dl[i] as u32 { 0 => 13, 1 => 10, _ => 8 };
        }
        assert_eq!(i, dl.len(), "plane draw list must parse exactly");
    }
}
