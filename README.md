# Ferray + WASM POC

A signal processing demo using [Ferray](https://github.com/forecast-bio/ferray) (the Rust NumPy replacement) compiled to WebAssembly and deployed to GitHub Pages.

**Live site:** [riziles.github.io/ferray-wasm-poc](https://riziles.github.io/ferray-wasm-poc)

## What it does

Four interactive demos running entirely in an 88 KB WASM binary:

- **Sum of Squares** — Creates a `ferray::Array<f64, Ix1>`, maps x², and sums
- **Sine Wave Generator** — Uses `ferray::linspace` with adjustable frequency and sample count
- **Gaussian Blur** — 1D convolution with a Gaussian kernel (adjustable σ)
- **Statistics** — Min, max, mean, median, std on sine wave data

## Architecture

```
JavaScript (browser)
    ↓
wasm-bindgen glue
    ↓
Rust → ferray-core (N-dimensional arrays, linspace, iterators)
    ↓
GitHub Actions builds & deploys to GitHub Pages
```

## WASM limitations

Only `ferray-core` compiles to WASM out of the box. The rest of the 18-crate workspace is blocked by a single dependency chain:

```
ferray-ufunc → core-math → core-math-sys → bindgen → libclang
```

`bindgen` generates C FFI bindings at build time and requires `libclang`, which can't resolve for the `wasm32-unknown-unknown` target. This transitively blocks `ferray-stats`, `ferray-fft`, and `ferray-linalg`.

The fix is ~2 hours of work: add a `wasm` feature to `ferray-ufunc` that swaps `core-math` for the pure-Rust `libm` crate in `cr_math.rs`.

Full details on the [live site](https://riziles.github.io/ferray-wasm-poc).

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
