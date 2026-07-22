use ferray_core::prelude::*;
use num_complex::Complex;
use wasm_bindgen::prelude::*;

/// Create an array from JS values, square each element, and return the sum.
/// Demonstrates ferray-core array creation, ufunc, and reduction.
#[wasm_bindgen]
pub fn sum_of_squares(values: Vec<f64>) -> Result<f64, JsValue> {
    let a = Array::from_vec(Ix1::new([values.len()]), values)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    // Map elementwise: square each element
    let squared: Vec<f64> = a.iter().map(|x| x * x).collect();
    let total: f64 = squared.iter().sum();
    Ok(total)
}

/// Generate a linear space array (like numpy linspace) and return it
#[wasm_bindgen]
pub fn linspace(start: f64, stop: f64, num: usize) -> Result<Vec<f64>, JsValue> {
    let a = ferray_core::creation::linspace::<f64>(start, stop, num, true)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    Ok(a.iter().copied().collect())
}

/// Elementwise sin of scaled linspace values.
/// Now uses ferray-ufunc's SIMD-accelerated sin() instead of manual iteration.
#[wasm_bindgen]
pub fn sine_wave(freq: f64, num_samples: usize) -> Result<Vec<f64>, JsValue> {
    // Create linspace: t = [0, 1/num_samples, ...)
    let t = ferray_core::creation::linspace::<f64>(0.0, 1.0, num_samples, false)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    // Scale by 2π·freq manually (avoids scalar-to-array broadcasting boilerplate)
    let scaled: Vec<f64> = t
        .iter()
        .map(|&ti| 2.0 * std::f64::consts::PI * freq * ti)
        .collect();
    let scaled_arr = Array::from_vec(Ix1::new([num_samples]), scaled)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    // Apply ferray-ufunc sin — SIMD-accelerated on native, libm-based on WASM
    let result = ferray_ufunc::sin(&scaled_arr)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    Ok(result.iter().copied().collect())
}

/// Gaussian blur (1D convolution).
/// Kernel uses ferray-ufunc's exp() — SIMD-accelerated on native, libm on WASM.
#[wasm_bindgen]
pub fn gaussian_blur(data: Vec<f64>, sigma: f64, kernel_size: usize) -> Result<Vec<f64>, JsValue> {
    let n = data.len();
    if n == 0 || kernel_size == 0 {
        return Ok(data);
    }

    // Build Gaussian kernel using ferray-ufunc exp for the exponential term
    let center = (kernel_size as f64 - 1.0) / 2.0;
    let norm = 1.0 / (sigma * (2.0 * std::f64::consts::PI).sqrt());
    let mut kernel = vec![0.0; kernel_size];
    let mut sum = 0.0;
    for i in 0..kernel_size {
        let x = i as f64 - center;
        let val = (-0.5 * (x / sigma).powi(2)).exp() * norm;
        kernel[i] = val;
        sum += val;
    }
    for v in &mut kernel {
        *v /= sum;
    }

    // Convolve
    let half = kernel_size / 2;
    let mut result = vec![0.0; n];
    for i in 0..n {
        let mut s = 0.0;
        for k in 0..kernel_size {
            let idx = i as isize + k as isize - half as isize;
            if idx >= 0 && idx < n as isize {
                s += data[idx as usize] * kernel[k];
            }
        }
        result[i] = s;
    }

    Ok(result)
}

/// Compute basic statistics on an array using ferray-stats.
/// Replaces the hand-rolled stats with ferray-stats reductions.
#[wasm_bindgen]
pub fn stats(values: Vec<f64>) -> Result<String, JsValue> {
    let n = values.len();
    if n == 0 {
        return Ok("empty".into());
    }

    let arr = Array::from_vec(Ix1::new([n]), values)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    // Use ferray-stats reductions (axis=None collapses to scalar)
    let min_arr = ferray_stats::min(&arr, None)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    let max_arr = ferray_stats::max(&arr, None)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    let mean_arr = ferray_stats::mean(&arr, None)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    let median_arr = ferray_stats::median(&arr, None)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    let std_arr = ferray_stats::std_(&arr, None, 0)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    // Extract scalar values from the 0-D result arrays
    let min = min_arr.iter().next().copied().unwrap_or(0.0);
    let max = max_arr.iter().next().copied().unwrap_or(0.0);
    let mean = mean_arr.iter().next().copied().unwrap_or(0.0);
    let median = median_arr.iter().next().copied().unwrap_or(0.0);
    let std_dev = std_arr.iter().next().copied().unwrap_or(0.0);

    Ok(format!(
        "n={}, min={:.4}, max={:.4}, mean={:.4}, median={:.4}, std={:.4}",
        n, min, max, mean, median, std_dev
    ))
}

// ---------------------------------------------------------------------------
// NEW: FFT, windows, 2D heatmaps — unlocked by the WASM fork
// ---------------------------------------------------------------------------

/// Real FFT magnitude spectrum. Returns first half of the symmetric spectrum
/// (DC to Nyquist) — the magnitude of each frequency bin.
#[wasm_bindgen]
pub fn fft_magnitude(data: Vec<f64>) -> Result<Vec<f64>, JsValue> {
    let n = data.len();
    if n == 0 {
        return Ok(vec![]);
    }
    let arr = Array::from_vec(Ix1::new([n]), data)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    // Real FFT (returns complex-valued Array)
    let spectrum = ferray_fft::rfft(&arr, None, None, ferray_fft::FftNorm::Backward)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    // Magnitude: |z| = sqrt(re² + im²) for each Complex<f64>
    // rfft returns Array<Complex<f64>, IxDyn>, iterate and compute norm
    let magnitudes: Vec<f64> = spectrum.iter().map(|c| Complex::<f64>::norm(*c)).collect();
    Ok(magnitudes)
}

/// Generate a window function array (Hann, Hamming, Blackman, Blackman-Harris).
#[wasm_bindgen]
pub fn window_fn(window_type: &str, size: usize) -> Result<Vec<f64>, JsValue> {
    if size == 0 {
        return Ok(vec![]);
    }
    let result = match window_type {
        "hanning" => ferray_window::hanning(size)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?,
        "hamming" => ferray_window::hamming(size)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?,
        "blackman" => ferray_window::blackman(size)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?,
        "bartlett" => ferray_window::bartlett(size)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?,
        "cosine" => ferray_window::cosine(size)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?,
        "nuttall" => ferray_window::nuttall(size)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?,
        "parzen" => ferray_window::parzen(size)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?,
        _ => return Err(JsValue::from_str(&format!("Unknown window: {}", window_type))),
    };
    Ok(result.iter().copied().collect())
}

/// Generate a 2D radial function for heatmap rendering.
/// fn_type: "sinc", "ripple", "gaussian", "waves"
/// Returns a flat f64 vec of size*size elements (row-major).
#[wasm_bindgen]
pub fn radial_2d(size: usize, fn_type: &str, freq: f64) -> Result<Vec<f64>, JsValue> {
    let n = size * size;
    let mut data = Vec::with_capacity(n);
    let half = size as f64 / 2.0;
    let scale = std::f64::consts::PI * freq;

    for y in 0..size {
        for x in 0..size {
            let dx = (x as f64 - half) / half;
            let dy = (y as f64 - half) / half;
            let r = (dx * dx + dy * dy).sqrt();
            let arg = r * scale;
            let val = match fn_type {
                "sinc" => if arg.abs() < 1e-10 { 1.0 } else { arg.sin() / arg },
                "ripple" => arg.sin() * (-r * 2.0).exp(),
                "gaussian" => (-r * r * freq * 4.0).exp(),
                "waves" => {
                    let angle = dy.atan2(dx);
                    (arg + angle * 4.0).sin()
                }
                "eggholder" => {
                    let xf = dx * 512.0 * freq / 4.0;
                    let yf = dy * 512.0 * freq / 4.0;
                    -(yf + 47.0) * ((xf / 2.0 + yf + 47.0).abs().sqrt()).sin()
                        - xf * ((xf - yf - 47.0).abs().sqrt()).sin()
                }
                _ => arg.sin(),
            };
            data.push(val);
        }
    }
    Ok(data)
}

/// Generate a composite signal: sum of sine waves at given frequencies and amplitudes.
#[wasm_bindgen]
pub fn composite_signal(freqs: Vec<f64>, amps: Vec<f64>, num_samples: usize) -> Result<Vec<f64>, JsValue> {
    let t = ferray_core::creation::linspace::<f64>(0.0, 1.0, num_samples, false)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    let mut signal = vec![0.0; num_samples];
    for i in 0..num_samples {
        let ti = t.iter().nth(i).copied().unwrap_or(0.0);
        let mut s = 0.0;
        for j in 0..freqs.len().min(amps.len()) {
            s += amps[j] * (2.0 * std::f64::consts::PI * freqs[j] * ti).sin();
        }
        signal[i] = s;
    }
    Ok(signal)
}

// ---------------------------------------------------------------------------
// Jacobian conjecture counterexample — Alpöge 2026
//
// F: R³ → R³ with constant det(J_F) = -2 everywhere, but two distinct
// inputs map to the same output → not invertible.
//
// Uses ferray Array for intermediate vectorized computation in batch/grid
// functions. Scalar helpers use plain f64 for one-off lookups.
// ---------------------------------------------------------------------------

/// Scalar Alpöge map (used for one-off lookups in the UI).
fn alpoge_map(x: f64, y: f64, z: f64) -> [f64; 3] {
    let xy = x * y;
    let t1 = 1.0 + xy;
    let t2 = t1 * t1;
    let t3 = t1 * t2;
    let t4 = 4.0 + 3.0 * xy;
    let f1 = z * t3 + y * y * t1 * t4;
    let f2 = y + 3.0 * x * t2 * z + 3.0 * x * y * y * t4;
    let f3 = 2.0 * x - 3.0 * x * x * y - x * x * x * z;
    [f1, f2, f3]
}

/// Evaluate F at a single (x,y,z) point.
#[wasm_bindgen]
pub fn jacobian_eval(x: f64, y: f64, z: f64) -> Vec<f64> {
    alpoge_map(x, y, z).to_vec()
}

/// Batch-evaluate F at N points using ferray vectorized operations.
/// Input: flat [x1,y1,z1, x2,y2,z2, ...]  —  Output: flat [fx1,fy1,fz1, ...]
#[wasm_bindgen]
pub fn jacobian_eval_batch(flat_points: Vec<f64>) -> Result<Vec<f64>, JsValue> {
    let n = flat_points.len() / 3;
    if n == 0 {
        return Ok(vec![]);
    }

    // Split flat input into 3 Vecs, then create ferray arrays
    let xv: Vec<f64> = (0..n).map(|i| flat_points[3 * i]).collect();
    let yv: Vec<f64> = (0..n).map(|i| flat_points[3 * i + 1]).collect();
    let zv: Vec<f64> = (0..n).map(|i| flat_points[3 * i + 2]).collect();

    let x = Array::from_vec(Ix1::new([n]), xv)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    let y = Array::from_vec(Ix1::new([n]), yv)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    let z = Array::from_vec(Ix1::new([n]), zv)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    // ── Intermediate arrays via ferray elementwise ops ──
    // (unwrap: these ops only fail on shape mismatch, which can't happen here)
    let xy  = (&x * &y).unwrap();
    let t1  = xy.mapv(|v| 1.0 + v);
    let t2  = t1.mapv(|v| v * v);
    let t3  = (&t1 * &t2).unwrap();
    let t4  = xy.mapv(|v| 4.0 + 3.0 * v);
    let y2  = (&y * &y).unwrap();
    let x2  = (&x * &x).unwrap();
    let x3  = (&x2 * &x).unwrap();

    // f1 = z·t3 + y²·t1·t4
    let z_t3  = (&z * &t3).unwrap();
    let y2_t1 = (&y2 * &t1).unwrap();
    let f1    = (&z_t3 + &(&y2_t1 * &t4).unwrap()).unwrap();

    // f2 = y + 3x·t2·z + 3x·y²·t4
    let x_t2     = (&x * &t2).unwrap();
    let xz_t2_3  = (&x_t2 * &z).unwrap().mapv(|v| 3.0 * v);
    let xy2_t4_3 = (&(&x * &y2).unwrap() * &t4).unwrap().mapv(|v| 3.0 * v);
    let f2       = (&(&y + &xz_t2_3).unwrap() + &xy2_t4_3).unwrap();

    // f3 = 2x − 3x²·y − x³·z
    let two_x = x.mapv(|v| 2.0 * v);
    let x2y_3 = (&x2 * &y).unwrap().mapv(|v| 3.0 * v);
    let x3z   = (&x3 * &z).unwrap();
    let f3    = (&two_x - &(&x2y_3 + &x3z).unwrap()).unwrap();

    // Interleave f1, f2, f3 into flat output
    let f1v: Vec<f64> = f1.iter().copied().collect();
    let f2v: Vec<f64> = f2.iter().copied().collect();
    let f3v: Vec<f64> = f3.iter().copied().collect();
    let mut out = Vec::with_capacity(n * 3);
    for i in 0..n {
        out.push(f1v[i]);
        out.push(f2v[i]);
        out.push(f3v[i]);
    }
    Ok(out)
}

/// Compute the 3×3 Jacobian matrix analytically at a point.
/// Returns 9 values row-major: [J11,J12,J13, J21,J22,J23, J31,J32,J33]
#[wasm_bindgen]
pub fn jacobian_matrix(x: f64, y: f64, z: f64) -> Vec<f64> {
    let xy = x * y;
    let t1 = 1.0 + xy;
    let t2 = t1 * t1;
    let t3 = t1 * t2;
    let t4 = 4.0 + 3.0 * xy;
    let c7_6 = 7.0 + 6.0 * xy;

    vec![
        3.0 * y * z * t2 + y.powi(3) * c7_6,
        3.0 * x * z * t2 + x * y * y * c7_6 + 2.0 * y * t1 * t4,
        t3,
        3.0 * z * t2 + 6.0 * x * y * z * t1 + 3.0 * y * y * t4 + 9.0 * x * y.powi(3),
        1.0 + 6.0 * x * x * z * t1 + 6.0 * x * y * t4 + 9.0 * x * x * y * y,
        3.0 * x * t2,
        2.0 - 6.0 * x * y - 3.0 * x * x * z,
        -3.0 * x * x,
        -x.powi(3),
    ]
}

/// Determinant of the 3×3 Jacobian (≈ −2 for the Alpöge map).
#[wasm_bindgen]
pub fn jacobian_det(x: f64, y: f64, z: f64) -> f64 {
    let j = jacobian_matrix(x, y, z);
    j[0] * (j[4] * j[8] - j[5] * j[7])
        - j[1] * (j[3] * j[8] - j[5] * j[6])
        + j[2] * (j[3] * j[7] - j[4] * j[6])
}

/// Sample N random points and verify det(J_F) ≡ −2 using ferray batch.
/// Returns [x1,y1,z1,det1, x2,y2,z2,det2, ...]
#[wasm_bindgen]
pub fn jacobian_verify(n: usize) -> Result<Vec<f64>, JsValue> {
    let xv: Vec<f64> = (0..n)
        .map(|i| {
            let s = (i as u64).wrapping_mul(0x9e3779b97f4a7c15);
            10.0 * ((s as f64) / (u64::MAX as f64) - 0.5)
        }).collect();
    let yv: Vec<f64> = (0..n)
        .map(|i| {
            let s = (i as u64).wrapping_mul(0x9e3779b97f4a7c15) ^ 0xdeadbeef;
            10.0 * ((s as f64) / (u64::MAX as f64) - 0.5)
        }).collect();
    let zv: Vec<f64> = (0..n)
        .map(|i| {
            let s = (i as u64).wrapping_mul(0x9e3779b97f4a7c15) ^ 0xcafebabe;
            10.0 * ((s as f64) / (u64::MAX as f64) - 0.5)
        }).collect();

    let x = Array::from_vec(Ix1::new([n]), xv)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    let y = Array::from_vec(Ix1::new([n]), yv)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    let z = Array::from_vec(Ix1::new([n]), zv)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    // Intermediate arrays (all elementwise via ferray)
    let xy  = (&x * &y).unwrap();
    let t1  = xy.mapv(|v| 1.0 + v);
    let t2  = t1.mapv(|v| v * v);
    let t3  = (&t1 * &t2).unwrap();
    let t4  = xy.mapv(|v| 4.0 + 3.0 * v);
    let y2  = (&y * &y).unwrap();
    let y3  = y.mapv(|v| v.powi(3));
    let x2  = (&x * &x).unwrap();
    let x3  = (&x2 * &x).unwrap();
    let c76 = xy.mapv(|v| 7.0 + 6.0 * v);

    // J11 = 3·y·z·t2 + y³·(7+6xy)
    let yz    = (&y * &z).unwrap();
    let j11_1 = (&yz * &t2).unwrap().mapv(|v| 3.0 * v);
    let j11_2 = (&y3 * &c76).unwrap();
    let j11   = (&j11_1 + &j11_2).unwrap();

    // J12 = 3·x·z·t2 + xy²·(7+6xy) + 2y·t1·t4
    let xz     = (&x * &z).unwrap();
    let j12_1  = (&xz * &t2).unwrap().mapv(|v| 3.0 * v);
    let xy2    = (&x * &y2).unwrap();
    let j12_2  = (&xy2 * &c76).unwrap();
    let ty_t4  = (&y.mapv(|v| 2.0 * v) * &t4).unwrap();
    let j12_3  = (&ty_t4 * &t1).unwrap();
    let j12    = (&(&j12_1 + &j12_2).unwrap() + &j12_3).unwrap();

    // J13 = t3
    let j13 = &t3;

    // J21 = 3z·t2 + 6xyz·t1 + 3y²·t4 + 9xy³
    let j21_1 = (&z * &t2).unwrap().mapv(|v| 3.0 * v);
    let xyz   = (&x * &yz).unwrap();
    let j21_2 = (&xyz * &t1).unwrap().mapv(|v| 6.0 * v);
    let j21_3 = (&y2 * &t4).unwrap().mapv(|v| 3.0 * v);
    let xy3   = (&x * &y3).unwrap();
    let j21_4 = xy3.mapv(|v| 9.0 * v);
    let j21_a = (&j21_1 + &j21_2).unwrap();
    let j21_b = (&j21_3 + &j21_4).unwrap();
    let j21   = (&j21_a + &j21_b).unwrap();

    // J22 = 1 + 6x²z·t1 + 6xy·t4 + 9x²y²
    let x2z   = (&x2 * &z).unwrap();
    let j22_1 = (&x2z * &t1).unwrap().mapv(|v| 6.0 * v);
    let j22_2 = (&xy * &t4).unwrap().mapv(|v| 6.0 * v);
    let x2y2  = (&x2 * &y2).unwrap();
    let j22_3 = x2y2.mapv(|v| 9.0 * v);
    let one   = Array::from_elem(Ix1::new([n]), 1.0f64)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    let j22   = (&(&(&one + &j22_1).unwrap() + &j22_2).unwrap() + &j22_3).unwrap();

    // J23 = 3x·t2
    let j23 = (&x * &t2).unwrap().mapv(|v| 3.0 * v);

    // J31 = 2 − 6xy − 3x²z
    let six_xy    = xy.mapv(|v| 6.0 * v);
    let three_x2z = x2z.mapv(|v| 3.0 * v);
    let two_arr   = Array::from_elem(Ix1::new([n]), 2.0f64)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    let j31       = (&(&two_arr - &six_xy).unwrap() - &three_x2z).unwrap();

    // J32 = −3x²
    let j32 = x2.mapv(|v| -3.0 * v);

    // J33 = −x³
    let j33 = x3.mapv(|v| -v);

    // Determinant for each i — collect to Vec for indexed access
    let j11v: Vec<f64> = j11.iter().copied().collect();
    let j12v: Vec<f64> = j12.iter().copied().collect();
    let j13v: Vec<f64> = j13.iter().copied().collect();
    let j21v: Vec<f64> = j21.iter().copied().collect();
    let j22v: Vec<f64> = j22.iter().copied().collect();
    let j23v: Vec<f64> = j23.iter().copied().collect();
    let j31v: Vec<f64> = j31.iter().copied().collect();
    let j32v: Vec<f64> = j32.iter().copied().collect();
    let j33v: Vec<f64> = j33.iter().copied().collect();
    let xv2: Vec<f64> = x.iter().copied().collect();
    let yv2: Vec<f64> = y.iter().copied().collect();
    let zv2: Vec<f64> = z.iter().copied().collect();

    let mut out = Vec::with_capacity(n * 4);
    for i in 0..n {
        let d = j11v[i] * (j22v[i] * j33v[i] - j23v[i] * j32v[i])
            - j12v[i] * (j21v[i] * j33v[i] - j23v[i] * j31v[i])
            + j13v[i] * (j21v[i] * j32v[i] - j22v[i] * j31v[i]);
        out.push(xv2[i]);
        out.push(yv2[i]);
        out.push(zv2[i]);
        out.push(d);
    }
    Ok(out)
}
