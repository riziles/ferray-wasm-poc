# Ferray × WASM — Svelte + Skeleton

A signal processing demo using [Ferray](https://github.com/dollspace-gay/ferray) (the Rust NumPy replacement) compiled to WebAssembly. Built with **Svelte 5** + **Skeleton v4** (Cerberus theme) + **Tailwind CSS 4**.

**Live site:** [riziles.github.io/ferray-wasm-poc](https://riziles.github.io/ferray-wasm-poc)

## Demos

Seven interactive demos running entirely in a ~524 KB WASM binary:

| Demo | Crates used | Description |
|---|---|---|
| 📐 Sum of Squares | `ferray-core` | Interactive `Float64Array` → WASM → Σ x² |
| 🌊 Sine Wave + Live Stats | `ferray-core`, `ferray-ufunc`, `ferray-stats` | Adjustable wave with live stat badges |
| 🔵 Gaussian Blur | `ferray-core`, `ferray-ufunc` | 1D convolution, adjustable σ |
| 📈 FFT Spectrum Analyzer | `ferray-core`, `ferray-fft`, `ferray-window` | Composite signal → windowed FFT → dual-canvas spectrum |
| 🪟 Window Function Gallery | `ferray-window` | 7 window functions overlaid, adjustable points |
| 🔥 2D Function Heatmap | `ferray-core`, `ferray-ufunc` | 200×200 canvas `ImageData`, 5 function types |
| 🧬 Crate Matrix | — | Which ferray workspace crates compile on WASM |

## Architecture

```
JavaScript (Svelte 5 + Skeleton v4)
    ↓  wasm-bindgen glue
Rust → ferray-core + ufunc + stats + fft + window
    ↓  GitHub Actions
GitHub Pages (pnpm + Vite build)
```

## Building locally

```bash
# Rust + WASM
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --no-typescript --out-dir svelte-app/src/lib/wasm \
  target/wasm32-unknown-unknown/release/ferray_wasm_poc.wasm

# Frontend
cd svelte-app
pnpm install
pnpm run dev    # dev server
pnpm run build  # production build
```

## WASM crate compatibility

| Crate | WASM? | Notes |
|---|---|---|
| `ferray-core` | ✅ | N-dimensional arrays |
| `ferray-ufunc` | ✅ | Trig, exp/log via libm on WASM |
| `ferray-stats` | ✅ | min/max/mean/median/std |
| `ferray-fft` | ✅ | Real FFT |
| `ferray-window` | ✅ | Hanning, Hamming, Blackman, etc. |
| `ferray-linalg` | ✅ | Matrix ops (available, not yet demoed) |
| `ferray-random` | ✅ | Distributions (available, not yet demoed) |
| `ferray-io` | ❌ | Blocked by zstd-sys |
| `ferray-python` | ❌ | Blocked by pyo3 |

The WASM fix uses target-conditional `libm` in [riziles/ferray-riz-dev](https://github.com/riziles/ferray-riz-dev): on `wasm32`, `core-math` is swapped for pure-Rust `libm` (~1-2 ULP, same as NumPy). No feature flags, no build.rs — just `cargo build --target wasm32-unknown-unknown`.

## Built by

🤖 This entire project — Rust code, WASM pipeline, Svelte component architecture, CI/CD — was built by a [pi](https://github.com/earendil-works/pi-coding-agent) coding agent.
