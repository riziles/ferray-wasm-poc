import init, {
  composite_signal,
  fft_magnitude,
  gaussian_blur,
  linspace,
  radial_2d,
  sine_wave,
  stats,
  sum_of_squares,
  window_fn,
} from './ferray_wasm_poc';

let _initialized = false;
let _initError: string | null = null;
let _initPromise: Promise<void> | null = null;

export type WasmApi = ReturnType<typeof createApi>;

function createApi() {
  return {
    get initialized() { return _initialized; },
    get initError() { return _initError; },
    composite_signal,
    fft_magnitude,
    gaussian_blur,
    linspace,
    radial_2d,
    sine_wave,
    stats,
    sum_of_squares,
    window_fn,
  };
}

export function initWasm(): Promise<WasmApi> {
  if (_initPromise) return _initPromise.then(() => createApi());
  _initPromise = (async () => {
    try {
      await init();
      _initialized = true;
    } catch (e) {
      _initError = String(e);
      console.error('WASM init failed:', e);
    }
  })();
  return _initPromise.then(() => createApi());
}

export function getWasmStatus() {
  return { initialized: _initialized, initError: _initError };
}
