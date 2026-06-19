use ferray_core::prelude::*;
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

/// Elementwise sin of an array of f64 values
#[wasm_bindgen]
pub fn sine_wave(freq: f64, num_samples: usize) -> Result<Vec<f64>, JsValue> {
    let t = ferray_core::creation::linspace::<f64>(0.0, 1.0, num_samples, false)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    // sin(2π * freq * t) for each t
    let result: Vec<f64> = t.iter().map(|&ti| (2.0 * std::f64::consts::PI * freq * ti).sin()).collect();
    Ok(result)
}

/// Gaussian blur (1D convolution) — a more interesting signal processing demo
#[wasm_bindgen]
pub fn gaussian_blur(data: Vec<f64>, sigma: f64, kernel_size: usize) -> Result<Vec<f64>, JsValue> {
    let n = data.len();
    if n == 0 || kernel_size == 0 {
        return Ok(data);
    }

    // Build normalized Gaussian kernel using ferray array
    let center = (kernel_size as f64 - 1.0) / 2.0;
    let mut kernel = vec![0.0; kernel_size];
    let mut sum = 0.0;
    for i in 0..kernel_size {
        let x = i as f64 - center;
        let val = (-0.5 * (x / sigma).powi(2)).exp() / (sigma * (2.0 * std::f64::consts::PI).sqrt());
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

/// Compute basic statistics on an array
#[wasm_bindgen]
pub fn stats(values: Vec<f64>) -> Result<String, JsValue> {
    let n = values.len();
    if n == 0 {
        return Ok("empty".into());
    }
    let sum: f64 = values.iter().sum();
    let mean = sum / n as f64;
    let variance: f64 = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
    let std_dev = variance.sqrt();

    let mut sorted = values.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let min = sorted[0];
    let max = sorted[n - 1];
    let median = sorted[n / 2];

    Ok(format!(
        "n={}, min={:.4}, max={:.4}, mean={:.4}, median={:.4}, std={:.4}",
        n, min, max, mean, median, std_dev
    ))
}
