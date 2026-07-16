# Ferray + WASM POC

A signal processing demo using [Ferray](https://github.com/dollspace-gay/ferray) (the Rust NumPy replacement) compiled to WebAssembly and deployed to GitHub Pages.

**Live site:** [riziles.github.io/ferray-wasm-poc](https://riziles.github.io/ferray-wasm-poc)

## What it does

Four interactive demos running entirely in a ~234 KB WASM binary:

- **Sum of Squares** — Creates a `ferray::Array<f64, Ix1>`, maps x², and sums
- **Sine Wave Generator** — Uses `ferray::linspace` + `ferray-ufunc::sin` (SIMD on native, libm on WASM)
- **Gaussian Blur** — 1D convolution with a Gaussian kernel (adjustable σ)
- **Statistics** — Uses `ferray-stats::min/max/mean/median/std` — real NumPy-equivalent reductions in the browser

## Architecture

```
JavaScript (browser)
    ↓
wasm-bindgen glue
    ↓
Rust → ferray-core (arrays) + ferray-ufunc (sin/exp) + ferray-stats (reductions)
    ↓
GitHub Actions builds & deploys to GitHub Pages
```

## WASM status — SOLVED ✅

The upstream blocker (`ferray-ufunc → core-math → bindgen → libclang`) has been fixed in [riziles/ferray-riz-dev](https://github.com/riziles/ferray-riz-dev/pull/1) by swapping `core-math` for the pure-Rust `libm` crate on `wasm32` targets using target-conditional dependencies (no feature flags needed).

**WASM compatibility after the fix:**

| Crate | Status | Notes |
|---|---|---|
| `ferray-core` | ✅ | N-dimensional arrays, creation, indexing |
| `ferray-ufunc` | ✅ | sin/cos/exp/log/arcsin/… via libm on WASM |
| `ferray-stats` | ✅ | min/max/mean/median/std/var/quantile/… |
| `ferray-fft` | ✅ | Always worked (no ufunc dep) |
| `ferray-linalg` | ✅ | faer sans rayon; GEMM wasm32 fallback |
| `ferray-random` | ✅ | getrandom + wasm_js |
| `ferray-polynomial` | ✅ | Unblocked by linalg/ufunc fixes |
| `ferray-window` | ✅ | Always worked |
| `ferray-io` | ❌ | Blocked by zstd-sys (needs C compiler) |
| `ferray-python` | ❌ | Blocked by pyo3 (no WASM target) |

### How it works

The fix uses Cargo's target-conditional dependencies:

```toml
# ferray-ufunc/Cargo.toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
core-math = { workspace = true }

[target.'cfg(target_arch = "wasm32")'.dependencies]
libm = { workspace = true }
```

And in `cr_math.rs`, the `CrMath` trait has separate impls for native (CORE-MATH, <0.5 ULP) and WASM (libm, ~1-2 ULP — same accuracy NumPy ships with). No feature flags, no build.rs — just `cargo build --target wasm32-unknown-unknown`.

## Building locally

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --no-typescript --out-dir web/pkg \
  target/wasm32-unknown-unknown/release/ferray_wasm_poc.wasm
# Serve web/ with any HTTP server
```

## Built by

🤖 This entire project — Rust code, WASM build pipeline, GitHub Actions deploy, and the page itself — was built by a [pi](https://github.com/earendil-works/pi-coding-agent) coding agent running DeepSeek V4 Pro.
