type WasmModule = Awaited<typeof import('./ferray_wasm_poc')>;

export interface WasmApi {
  readonly initialized: boolean;
  readonly initError: string | null;
  composite_signal: WasmModule['composite_signal'];
  fft_magnitude: WasmModule['fft_magnitude'];
  gaussian_blur: WasmModule['gaussian_blur'];
  jacobian_det: WasmModule['jacobian_det'];
  jacobian_eval: WasmModule['jacobian_eval'];
  jacobian_eval_batch: WasmModule['jacobian_eval_batch'];
  jacobian_autodiff: WasmModule['jacobian_autodiff'];
  jacobian_verify: WasmModule['jacobian_verify'];
  linspace: WasmModule['linspace'];
  radial_2d: WasmModule['radial_2d'];
  sine_wave: WasmModule['sine_wave'];
  stats: WasmModule['stats'];
  sum_of_squares: WasmModule['sum_of_squares'];
  window_fn: WasmModule['window_fn'];
}

let _initialized = false;
let _initError: string | null = null;
let _wasmApi: WasmApi | null = null;
let _initPromise: Promise<WasmApi> | null = null;

function createApi(mod: WasmModule): WasmApi {
  return {
    get initialized() { return _initialized; },
    get initError() { return _initError; },
    composite_signal: mod.composite_signal,
    fft_magnitude: mod.fft_magnitude,
    gaussian_blur: mod.gaussian_blur,
    jacobian_det: mod.jacobian_det,
    jacobian_eval: mod.jacobian_eval,
    jacobian_eval_batch: mod.jacobian_eval_batch,
    jacobian_autodiff: mod.jacobian_autodiff,
    jacobian_verify: mod.jacobian_verify,
    linspace: mod.linspace,
    radial_2d: mod.radial_2d,
    sine_wave: mod.sine_wave,
    stats: mod.stats,
    sum_of_squares: mod.sum_of_squares,
    window_fn: mod.window_fn,
  };
}

export function initWasm(): Promise<WasmApi> {
  if (_wasmApi) return Promise.resolve(_wasmApi);
  if (_initPromise) return _initPromise;

  _initPromise = import('./ferray_wasm_poc').then(async (mod) => {
    await mod.default();
    _initialized = true;
    _wasmApi = createApi(mod);
    return _wasmApi;
  }).catch((e) => {
    _initError = String(e);
    console.error('WASM init failed:', e);
    throw e;
  });

  return _initPromise;
}

export function getWasmStatus() {
  return { initialized: _initialized, initError: _initError };
}
