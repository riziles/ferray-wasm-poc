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
