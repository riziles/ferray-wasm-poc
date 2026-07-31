//! Tiny Kolmogorov–Arnold Network (KAN) — pure Rust, WASM-friendly.
//!
//! Based on the Kolmogorov–Arnold representation theorem: any multivariate
//! continuous function can be written as a finite composition of univariate
//! functions and addition. KANs (Liu et al., 2024) make this practical by
//! putting a learnable univariate function φ(x) — here a B-spline plus a
//! SiLU residual — on every edge instead of a linear weight.
//!
//! This implementation trains a [2 → hidden → 1] KAN on 2D target functions
//! using full-batch Adam backpropagation, with hand-written gradients
//! (forward-mode autodiff would cost O(n_params) per sample — too slow).
//!
//! Layout of the flat parameter vector, per edge:
//!   [ base_weight, c_0, c_1, …, c_{G+k} ]
//! where G = grid intervals, k = spline order, so each edge carries
//! G + k + 1 B-spline coefficients. φ(x) = base·silu(x) + Σ cᵢ·Bᵢ(x).

use serde_json::json;
use std::cell::RefCell;

// ────────────────────────────────────────────────────────────────────────────
// Deterministic RNG (xorshift64) — no OS entropy needed on WASM
// ────────────────────────────────────────────────────────────────────────────

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed.max(1))
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn uniform(&mut self, lo: f64, hi: f64) -> f64 {
        let u = (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64;
        lo + (hi - lo) * u
    }
}

// ────────────────────────────────────────────────────────────────────────────
// B-spline basis (Cox–de Boor)
// ────────────────────────────────────────────────────────────────────────────

/// Build the extended knot vector for a B-spline on [−1, 1] with `g` intervals
/// and order `k` (degree k−1). Returns g + 2k + 1 knots:
///   [−1 − k·h … −1 − h] ∪ [−1, …, 1] (g+1 uniform knots) ∪ [1 + h … 1 + k·h]
/// The *valid* curve domain is [grid[k−1], grid[g+1+k]] (the extended range
/// minus k knots on each side); edge_phi extrapolates linearly beyond it.
fn make_grid(g: usize, k: usize) -> Vec<f64> {
    let mut grid = vec![0.0; g + 1 + 2 * k];
    let h = 2.0 / g as f64;
    for i in 0..=g {
        grid[k + i] = -1.0 + 2.0 * i as f64 / g as f64;
    }
    for j in 0..k {
        grid[k - 1 - j] = -1.0 - (j + 1) as f64 * h;
        grid[k + g + 1 + j] = 1.0 + (j + 1) as f64 * h;
    }
    grid
}

/// Evaluate all G + k + 1 B-spline basis functions of order `k` at `x`,
/// together with their derivatives.
///
/// Only the `k` basis functions active in `x`'s interval are non-zero, so the
/// Cox–de Boor recursion runs on a length-`d` window instead of the full knot
/// vector (≈7× fewer ops than the naive all-vector recursion). `x` is clamped
/// into the valid domain [grid[k−1], grid[n−k]]; edge_phi handles the linear
/// extrapolation beyond it.
fn spline_basis(x: f64, grid: &[f64], k: usize) -> (Vec<f64>, Vec<f64>) {
    let n = grid.len();
    let nb = n - k; // number of basis functions (G + k + 1)
    let lo = grid[k - 1];
    let hi = grid[n - k];
    let x = x.clamp(lo, hi);

    // interval index: largest idx with grid[idx] <= x < grid[idx+1],
    // restricted so the active window stays inside [0, nb-1].
    let mut idx = 0usize;
    while idx + 1 < n && grid[idx + 1] <= x {
        idx += 1;
    }
    idx = idx.clamp(k - 1, n - k - 1);

    let mut vals = vec![0.0; nb];
    let mut ders = vec![0.0; nb];

    // degree-0 basis: B_{idx,0} = 1 (half-open interval [grid[idx], grid[idx+1]))
    let mut b: Vec<f64> = vec![1.0];
    let mut db: Vec<f64> = vec![0.0];

    for d in 1..k {
        let mut nb2 = vec![0.0; d + 1];
        let mut ndb = vec![0.0; d + 1];
        // active bases at degree d: global idx-d ..= idx (local j = 0..=d)
        for j in 0..=d {
            let gi = idx - d + j;
            let (b1, db1) = if j >= 1 { (b[j - 1], db[j - 1]) } else { (0.0, 0.0) };
            let (b2, db2) = if j <= d - 1 { (b[j], db[j]) } else { (0.0, 0.0) };
            let den1 = grid[gi + d] - grid[gi];
            let den2 = grid[gi + d + 1] - grid[gi + 1];
            let (w1, w1d) = if den1.abs() > 1e-14 {
                ((x - grid[gi]) / den1, 1.0 / den1)
            } else {
                (0.0, 0.0)
            };
            let (w2, w2d) = if den2.abs() > 1e-14 {
                ((grid[gi + d + 1] - x) / den2, -1.0 / den2)
            } else {
                (0.0, 0.0)
            };
            nb2[j] = w1 * b1 + w2 * b2;
            ndb[j] = w1d * b1 + w1 * db1 + w2d * b2 + w2 * db2;
        }
        b = nb2;
        db = ndb;
    }

    // write the k active bases (degree k−1) into the full vector
    let start = idx + 1 - k;
    for j in 0..k {
        vals[start + j] = b[j];
        ders[start + j] = db[j];
    }
    (vals, ders)
}

// ────────────────────────────────────────────────────────────────────────────
// Network structures
// ────────────────────────────────────────────────────────────────────────────

/// One edge's position in the flat parameter vector.
struct EdgeInfo {
    o: usize,    // output unit index
    i: usize,    // input unit index
    off: usize,  // flat offset of this edge's parameters
    ncoeff: usize, // number of spline coefficients (G + k + 1)
}

struct LayerInfo {
    in_dim: usize,
    out_dim: usize,
    grid: Vec<f64>,
    edge_start: usize,
    edge_count: usize,
}

/// Values captured during forward that backprop needs.
struct EdgeCache {
    basis: Vec<f64>, // B-spline basis values at the input
    silu: f64,       // SiLU of the input
    dphi: f64,       // dφ/dx
}

pub struct KanTrainer {
    layer_dims: Vec<usize>,
    layers: Vec<LayerInfo>,
    edges: Vec<EdgeInfo>,
    k: usize,
    params: Vec<f64>,
    // Adam state
    m: Vec<f64>,
    v: Vec<f64>,
    t: usize,
    // training data
    target: usize,
    samples: Vec<f64>, // flat [x0,y0, x1,y1, …], normalized to [−1,1]²
    targets: Vec<f64>, // min–max normalized to [0, 1]
    tmin: f64,
    tmax: f64,
    epoch: usize,
}

// ────────────────────────────────────────────────────────────────────────────
// Target functions (x, y ∈ [−3, 3])
// ────────────────────────────────────────────────────────────────────────────

pub fn target_fn(id: usize, x: f64, y: f64) -> f64 {
    match id {
        0 => ((std::f64::consts::PI * x / 2.0).sin().exp()) * ((y * y / 8.0).exp()), // wavy exp
        1 => (x * x - y * y) / 18.0,                                                 // saddle
        2 => x * y / 9.0,                                                            // product
        3 => (x / 2.0).sin() * (y / 2.0).cos(),                                       // ripples
        4 => (-(x * x + y * y) / 6.0).exp(),                                          // gaussian
        _ => ((x * x + y * y) / 3.0).sin() * 0.5,                                     // rings
    }
}

/// Uniform grid of samples on [−3, 3]² with min–max normalized targets.
fn build_data(target: usize, samples_per_axis: usize) -> (Vec<f64>, Vec<f64>, f64, f64) {
    let n = samples_per_axis * samples_per_axis;
    let step = (samples_per_axis.max(2) - 1) as f64;
    let mut samples = Vec::with_capacity(2 * n);
    let mut raw = Vec::with_capacity(n);
    for iy in 0..samples_per_axis {
        for ix in 0..samples_per_axis {
            let x = -3.0 + 6.0 * ix as f64 / step;
            let y = -3.0 + 6.0 * iy as f64 / step;
            samples.push(x / 3.0);
            samples.push(y / 3.0);
            raw.push(target_fn(target, x, y));
        }
    }
    let tmin = raw.iter().cloned().fold(f64::INFINITY, f64::min);
    let tmax = raw.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let span = (tmax - tmin).max(1e-12);
    let targets = raw.iter().map(|t| (t - tmin) / span).collect();
    (samples, targets, tmin, tmax)
}

// ────────────────────────────────────────────────────────────────────────────
// Edge function φ(x) = base·silu(x) + Σ cᵢ Bᵢ(x)
// ────────────────────────────────────────────────────────────────────────────

/// Combine precomputed basis values/derivatives (shared by all edges that
/// read the same input unit) with an edge's coefficients.
fn edge_phi_from_parts(
    base: f64,
    coeffs: &[f64],
    vals: &[f64],
    ders: &[f64],
    x: f64,
    lo: f64,
    hi: f64,
) -> (f64, f64, f64) {
    let mut s = 0.0;
    let mut ds = 0.0;
    for (i, &c) in coeffs.iter().enumerate() {
        s += c * vals[i];
        ds += c * ders[i];
    }
    // Linear extrapolation beyond the valid domain (keeps φ C¹)
    if x < lo {
        s += (x - lo) * ds;
    } else if x > hi {
        s += (x - hi) * ds;
    }
    let sig = 1.0 / (1.0 + (-x).exp());
    let silu = x * sig;
    let dsilu = sig * (1.0 + x * (1.0 - sig));
    (base * silu + s, base * dsilu + ds, silu)
}

fn edge_phi(params: &[f64], e: &EdgeInfo, grid: &[f64], k: usize, x: f64) -> (f64, f64, Vec<f64>, f64) {
    let base = params[e.off];
    let coeffs = &params[e.off + 1..e.off + 1 + e.ncoeff];
    let n = grid.len();
    let (vals, ders) = spline_basis(x, grid, k);
    let (phi, dphi, silu) =
        edge_phi_from_parts(base, coeffs, &vals, &ders, x, grid[k - 1], grid[n - k]);
    (phi, dphi, vals, silu)
}

// ────────────────────────────────────────────────────────────────────────────
// Forward / backward
// ────────────────────────────────────────────────────────────────────────────

impl KanTrainer {
    /// Returns (prediction, per-layer edge caches, per-layer activations).
    /// Basis functions are computed once per input unit and shared by every
    /// edge reading that unit (layer-0 edges all read x or y).
    fn forward(&self, x: &[f64]) -> (Vec<f64>, Vec<Vec<EdgeCache>>, Vec<Vec<f64>>) {
        let mut acts: Vec<Vec<f64>> = vec![x.to_vec()];
        let mut caches_all = Vec::with_capacity(self.layers.len());
        for l in 0..self.layers.len() {
            let li = &self.layers[l];
            let n = li.grid.len();
            let (lo, hi) = (li.grid[self.k - 1], li.grid[n - self.k]);
            let mut out = vec![0.0; li.out_dim];
            let mut caches = Vec::with_capacity(li.edge_count);
            let per_input: Vec<(Vec<f64>, Vec<f64>)> = (0..li.in_dim)
                .map(|i| spline_basis(acts[l][i], &li.grid, self.k))
                .collect();
            for e in &self.edges[li.edge_start..li.edge_start + li.edge_count] {
                let (vals, ders) = &per_input[e.i];
                let (phi, dphi, silu) = edge_phi_from_parts(
                    self.params[e.off],
                    &self.params[e.off + 1..e.off + 1 + e.ncoeff],
                    vals,
                    ders,
                    acts[l][e.i],
                    lo,
                    hi,
                );
                caches.push(EdgeCache {
                    basis: vals.clone(),
                    silu,
                    dphi,
                });
                out[e.o] += phi;
            }
            caches_all.push(caches);
            acts.push(out);
        }
        (acts.last().unwrap().clone(), caches_all, acts)
    }

    /// Accumulate gradients of L = ½Σ(ŷ − y)² for one sample.
    fn backward(&self, pred: &[f64], caches_all: &[Vec<EdgeCache>], y: f64) -> Vec<f64> {
        let mut grads = vec![0.0; self.params.len()];
        // error signal at the output: ∂L/∂ŷ = ŷ − y
        let mut err: Vec<f64> = pred.iter().map(|p| p - y).collect();

        for l in (0..self.layers.len()).rev() {
            let li = &self.layers[l];
            let mut err_in = vec![0.0; li.in_dim];
            for (idx, e) in self.edges[li.edge_start..li.edge_start + li.edge_count]
                .iter()
                .enumerate()
            {
                let c = &caches_all[l][idx];
                let en = err[e.o];
                grads[e.off] += en * c.silu;
                for (ci, &b) in c.basis.iter().enumerate() {
                    grads[e.off + 1 + ci] += en * b;
                }
                err_in[e.i] += en * c.dphi;
            }
            err = err_in;
        }
        grads
    }

    fn adam_step(&mut self, grads: &[f64], lr: f64, n: f64) {
        self.t += 1;
        let b1: f64 = 0.9;
        let b2: f64 = 0.999;
        let eps = 1e-8;
        let bc1 = 1.0 - b1.powi(self.t as i32);
        let bc2 = 1.0 - b2.powi(self.t as i32);
        for i in 0..self.params.len() {
            let g = grads[i] / n;
            self.m[i] = b1 * self.m[i] + (1.0 - b1) * g;
            self.v[i] = b2 * self.v[i] + (1.0 - b2) * g * g;
            let mhat = self.m[i] / bc1;
            let vhat = self.v[i] / bc2;
            self.params[i] -= lr * mhat / (vhat.sqrt() + eps);
        }
    }

    /// Run `epochs` full-batch Adam epochs; returns RMSE (normalized scale)
    /// measured before each update.
    pub fn train(&mut self, epochs: usize, lr: f64) -> Vec<f64> {
        let n = self.targets.len();
        let mut losses = Vec::with_capacity(epochs);
        for _ in 0..epochs {
            let mut grads = vec![0.0; self.params.len()];
            let mut se = 0.0;
            for s in 0..n {
                let x = [self.samples[2 * s], self.samples[2 * s + 1]];
                let (pred, caches, _) = self.forward(&x);
                let e = pred[0] - self.targets[s];
                se += e * e;
                let g = self.backward(&pred, &caches, self.targets[s]);
                for i in 0..grads.len() {
                    grads[i] += g[i];
                }
            }
            self.adam_step(&grads, lr, n as f64);
            self.epoch += 1;
            losses.push((se / n as f64).sqrt());
        }
        losses
    }

    /// Evaluate the target (normalized) and the network on a size×size grid.
    /// Returns interleaved [truth, learned] pairs, row-major.
    ///
    /// Exploits the grid structure: layer 0 reads only the coordinate, so its
    /// basis functions are computed once per distinct x/y value instead of
    /// once per pixel.
    pub fn eval_grid(&self, size: usize) -> Vec<f64> {
        let span = (self.tmax - self.tmin).max(1e-12);
        let step = (size.max(2) - 1) as f64;
        let coords: Vec<f64> = (0..size).map(|i| -1.0 + 2.0 * i as f64 / step).collect();

        let l0 = &self.layers[0];
        let n0 = l0.grid.len();
        let (lo0, hi0) = (l0.grid[self.k - 1], l0.grid[n0 - self.k]);
        let basis_x: Vec<(Vec<f64>, Vec<f64>)> =
            coords.iter().map(|&x| spline_basis(x, &l0.grid, self.k)).collect();
        let basis_y = basis_x.clone(); // unit 1 reads y, same coordinate set

        let mut out = Vec::with_capacity(size * size * 2);
        for iy in 0..size {
            for ix in 0..size {
                let x = -3.0 + 6.0 * ix as f64 / step;
                let y = -3.0 + 6.0 * iy as f64 / step;
                let truth = (target_fn(self.target, x, y) - self.tmin) / span;

                // layer 0: hidden[j] = φ_{0j}(xs) + φ_{1j}(ys)
                let mut h = vec![0.0; self.layer_dims[1]];
                for e in &self.edges[l0.edge_start..l0.edge_start + l0.edge_count] {
                    let (vals, ders) = if e.i == 0 { &basis_x[ix] } else { &basis_y[iy] };
                    let xin = if e.i == 0 { coords[ix] } else { coords[iy] };
                    let (phi, _, _) = edge_phi_from_parts(
                        self.params[e.off],
                        &self.params[e.off + 1..e.off + 1 + e.ncoeff],
                        vals,
                        ders,
                        xin,
                        lo0,
                        hi0,
                    );
                    h[e.o] += phi;
                }

                // layer 1: pred = Σ_j φ_{j0}(h[j])
                let l1 = &self.layers[1];
                let n1 = l1.grid.len();
                let (lo1, hi1) = (l1.grid[self.k - 1], l1.grid[n1 - self.k]);
                let per_input: Vec<(Vec<f64>, Vec<f64>)> = (0..l1.in_dim)
                    .map(|j| spline_basis(h[j], &l1.grid, self.k))
                    .collect();
                let mut pred = 0.0;
                for e in &self.edges[l1.edge_start..l1.edge_start + l1.edge_count] {
                    let (vals, ders) = &per_input[e.i];
                    let (phi, _, _) = edge_phi_from_parts(
                        self.params[e.off],
                        &self.params[e.off + 1..e.off + 1 + e.ncoeff],
                        vals,
                        ders,
                        h[e.i],
                        lo1,
                        hi1,
                    );
                    pred += phi;
                }

                out.push(truth);
                out.push(pred);
            }
        }
        out
    }

    /// Evaluate one edge's φ(x) over x ∈ [−4, 4], returned as [x, y] pairs.
    pub fn edge_spline(&self, layer: usize, from: usize, to: usize, points: usize) -> Vec<f64> {
        let (start, count) = match self.layers.get(layer) {
            Some(li) => (li.edge_start, li.edge_count),
            None => return vec![],
        };
        let Some(e) = self.edges[start..start + count].iter().find(|e| e.i == from && e.o == to)
        else {
            return vec![];
        };
        let grid = &self.layers[layer].grid;
        let step = (points.max(2) - 1) as f64;
        let mut out = Vec::with_capacity(points * 2);
        for p in 0..points {
            let x = -4.0 + 8.0 * p as f64 / step;
            let (phi, _, _, _) = edge_phi(&self.params, e, grid, self.k, x);
            out.push(x);
            out.push(phi);
        }
        out
    }

    pub fn stats_json(&self) -> String {
        json!({
            "epoch": self.epoch,
            "layers": self.layer_dims,
            "g": self.layers[0].grid.len() - 2 * self.k - 1,
            "k": self.k,
            "n_params": self.params.len(),
            "n_edges": self.edges.len(),
            "samples": self.targets.len(),
            "target": self.target,
            "tmin": self.tmin,
            "tmax": self.tmax,
        })
        .to_string()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Construction
// ────────────────────────────────────────────────────────────────────────────

impl KanTrainer {
    pub fn new(
        layer_dims: Vec<usize>,
        g: usize,
        k: usize,
        target: usize,
        samples_per_axis: usize,
        seed: u64,
    ) -> Result<Self, String> {
        if layer_dims.len() < 2 {
            return Err("need at least 2 layers".into());
        }
        if layer_dims.iter().any(|&d| d == 0) {
            return Err("layer dims must be positive".into());
        }
        if g < 2 || k < 2 {
            return Err("grid ≥ 2 and order ≥ 2 required".into());
        }
        if target > 5 {
            return Err("unknown target".into());
        }
        if samples_per_axis < 3 {
            return Err("samples_per_axis ≥ 3 required".into());
        }

        let mut rng = Rng::new(seed);
        let ncoeff = g + k + 1;
        let mut edges = Vec::new();
        let mut layers = Vec::new();
        let mut params = Vec::new();
        let mut off = 0usize;

        for l in 0..layer_dims.len() - 1 {
            let in_dim = layer_dims[l];
            let out_dim = layer_dims[l + 1];
            let edge_start = edges.len();
            let mut edge_count = 0;
            for o in 0..out_dim {
                for i in 0..in_dim {
                    edges.push(EdgeInfo { o, i, off, ncoeff });
                    params.push(1.0); // base weight (φ starts ≈ silu)
                    for _ in 0..ncoeff {
                        params.push(rng.uniform(-0.5, 0.5));
                    }
                    off += 1 + ncoeff;
                    edge_count += 1;
                }
            }
            layers.push(LayerInfo {
                in_dim,
                out_dim,
                grid: make_grid(g, k),
                edge_start,
                edge_count,
            });
        }

        let (samples, targets, tmin, tmax) = build_data(target, samples_per_axis);
        let n = params.len();
        Ok(KanTrainer {
            layer_dims,
            layers,
            edges,
            k,
            params,
            m: vec![0.0; n],
            v: vec![0.0; n],
            t: 0,
            target,
            samples,
            targets,
            tmin,
            tmax,
            epoch: 0,
        })
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Global trainer state (single-threaded WASM main thread)
// ────────────────────────────────────────────────────────────────────────────

thread_local! {
    static TRAINER: RefCell<Option<KanTrainer>> = const { RefCell::new(None) };
}

fn with_trainer<R>(f: impl FnOnce(&mut KanTrainer) -> R) -> Result<R, JsValue> {
    TRAINER.with(|cell| {
        let mut guard = cell.borrow_mut();
        match guard.as_mut() {
            Some(t) => Ok(f(t)),
            None => Err(JsValue::from_str(
                "KAN not initialized — call kan_reset first",
            )),
        }
    })
}

use wasm_bindgen::JsValue;

/// (Re)create the KAN with fresh random weights and regenerate training data.
pub fn reset(
    hidden: usize,
    grid: usize,
    order: usize,
    target: usize,
    samples_per_axis: usize,
    seed: u32,
) -> Result<(), JsValue> {
    let trainer = KanTrainer::new(
        vec![2, hidden.max(1), 1],
        grid.clamp(2, 32),
        order.clamp(2, 6),
        target,
        samples_per_axis.clamp(3, 64),
        seed as u64,
    )
    .map_err(|e| JsValue::from_str(&e))?;
    TRAINER.with(|cell| *cell.borrow_mut() = Some(trainer));
    Ok(())
}

pub fn train(epochs: usize, lr: f64) -> Result<Vec<f64>, JsValue> {
    with_trainer(|t| t.train(epochs.clamp(1, 500), lr.max(1e-5)))
}

pub fn eval_grid(size: usize) -> Result<Vec<f64>, JsValue> {
    with_trainer(|t| t.eval_grid(size.clamp(2, 512)))
}

pub fn edge_spline(layer: usize, from: usize, to: usize, points: usize) -> Result<Vec<f64>, JsValue> {
    with_trainer(|t| t.edge_spline(layer, from, to, points.clamp(4, 512)))
}

pub fn stats() -> Result<String, JsValue> {
    with_trainer(|t| t.stats_json())
}

// ────────────────────────────────────────────────────────────────────────────
// Native tests — verify the math before shipping to WASM
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// B-spline basis must form a partition of unity: Σ Bᵢ(x) = 1
    /// inside the valid knot span [t_{k−1}, t_{n−k}] (outside it, the
    /// linear extrapolation in edge_phi takes over).
    #[test]
    fn basis_partition_of_unity() {
        for &(g, k) in &[(4usize, 3usize), (8, 4), (12, 2)] {
            let grid = make_grid(g, k);
            let n = grid.len();
            let (lo, hi) = (grid[k - 1], grid[n - k]);
            // include the exact knot positions in the sweep
            let mut xs: Vec<f64> = Vec::new();
            for p in 0..400 {
                xs.push(lo + (hi - lo) * p as f64 / 399.0);
            }
            for &knot in &grid[k - 1..=n - k] {
                xs.push(knot);
            }
            for x in xs {
                let (vals, _) = spline_basis(x, &grid, k);
                let sum: f64 = vals.iter().sum();
                assert!((sum - 1.0).abs() < 1e-9, "g={g} k={k} x={x} sum={sum}");
                assert!(vals.len() == g + k + 1, "basis count mismatch");
            }
        }
    }

    /// Numerical check of the analytic derivative of the full edge function
    /// (including the linear extrapolation past the valid domain).
    #[test]
    fn spline_derivative_matches_finite_difference() {
        let g = 6;
        let k = 3;
        let grid = make_grid(g, k);
        let mut rng = Rng::new(42);
        let coeffs: Vec<f64> = (0..g + k + 1).map(|_| rng.uniform(-1.0, 1.0)).collect();
        let e = EdgeInfo { o: 0, i: 0, off: 0, ncoeff: coeffs.len() };
        let params = [1.0f64].into_iter().chain(coeffs.clone()).collect::<Vec<_>>();
        let n = grid.len();
        let (lo, hi) = (grid[k - 1], grid[n - k]);
        let h = 1e-6;
        for p in 0..200 {
            let x = -3.5 + 7.0 * p as f64 / 199.0;
            let (_, ds, _, _) = edge_phi(&params, &e, &grid, k, x);
            let (v2, _, _, _) = edge_phi(&params, &e, &grid, k, x + h);
            let (v1, _, _, _) = edge_phi(&params, &e, &grid, k, x - h);
            let fd = (v2 - v1) / (2.0 * h);
            assert!(
                (ds - fd).abs() < 1e-4,
                "x={x} (lo={lo} hi={hi}) ds={ds} fd={fd}"
            );
        }
    }

    /// A 2→5→1 KAN must actually learn 2D target functions.
    #[test]
    fn kan_learns_targets() {
        for target in 0..6 {
            let mut t = KanTrainer::new(vec![2, 5, 1], 6, 3, target, 24, 7 + target as u64).unwrap();
            let mut last = f64::INFINITY;
            for _ in 0..12 {
                let losses = t.train(50, 0.02);
                last = *losses.last().unwrap();
            }
            assert!(
                last < 0.15,
                "target {target} failed to converge: RMSE={last:.4}"
            );
        }
    }
}
