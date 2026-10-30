<script lang="ts">
  // Section 2.1, the trick the whole proof rests on. Translating the wave by an
  // imaginary vector −iρ(1, ω) multiplies a mode of frequency ξ by
  //
  //     e^(ρ(ξ·ω − |ξ|)) = e^(−ρ|ξ| |ξ̂ − ω|²/2),
  //
  // i.e. a Gaussian in the *direction* of ξ, of angular width (ρN)^(-1/2). It
  // multiplies the sum rather than its square, so the interference — the thing
  // that makes a focus — survives. What shrinks is the cone the beam diverges
  // into.

  import { onMount } from 'svelte';
  import { clear, fit, fmt } from './util';

  let N = $state(40); // frequency
  let gamma = $state(0.24); // cap half-width, radians
  let kappa = $state(120); // ρN: the weight is exp(−κ(1 − cos θ))
  let canvasL: HTMLCanvasElement;
  let canvasR: HTMLCanvasElement;
  let observer: ResizeObserver | null = null;

  const GX = 300;
  const GY = 168;
  const X0 = 0;
  const X1 = 3;
  const YL = 0.8;
  const M = 41;

  let sigL = $state(0);
  let sigR = $state(0);
  let keepR = $state(0);

  function angles(): number[] {
    const out: number[] = [];
    for (let j = 0; j < M; j++) out.push(-gamma + (2 * gamma * j) / (M - 1));
    return out;
  }

  /** |Σ w_j e^{iN(x cos θ_j + y sin θ_j)}| over the box, and the angular moments. */
  function field(weights: number[]) {
    const th = angles();
    const cx = th.map((t) => Math.cos(t));
    const cy = th.map((t) => Math.sin(t));
    const dx = (X1 - X0) / (GX - 1);
    const dy = (2 * YL) / (GY - 1);
    const data = new Float32Array(GX * GY);
    let peak = 0;
    for (let j = 0; j < GY; j++) {
      const y = -YL + j * dy;
      for (let i = 0; i < GX; i++) {
        const x = X0 + i * dx;
        let re = 0;
        let im = 0;
        for (let k = 0; k < th.length; k++) {
          const phase = N * (x * cx[k] + y * cy[k]);
          re += weights[k] * Math.cos(phase);
          im += weights[k] * Math.sin(phase);
        }
        const v = Math.hypot(re, im);
        data[j * GX + i] = v;
        if (v > peak) peak = v;
      }
    }
    // angular moments of the weight distribution
    let s0 = 0;
    let s1 = 0;
    let s2 = 0;
    let s4 = 0;
    for (let k = 0; k < th.length; k++) {
      const w = weights[k];
      s0 += w;
      s1 += w * th[k];
      s2 += w * th[k] * th[k];
      s4 += w * w;
    }
    const mean = s0 > 0 ? s1 / s0 : 0;
    const sigma = s0 > 0 ? Math.sqrt(Math.max(0, s2 / s0 - mean * mean)) : 0;
    return { data, peak: Math.max(peak, 1e-9), sigma, keep: s4 > 0 ? (s0 * s0) / s4 : 0, weights };
  }

  function paint(
    canvas: HTMLCanvasElement,
    f: { data: Float32Array; peak: number; weights: number[] },
    tint: [number, number, number]
  ) {
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h, '#0b1020');
    const off = document.createElement('canvas');
    off.width = GX;
    off.height = GY;
    const octx = off.getContext('2d')!;
    const img = octx.createImageData(GX, GY);
    for (let j = 0; j < GY; j++) {
      for (let i = 0; i < GX; i++) {
        const v = Math.min(1, Math.pow(f.data[j * GX + i] / f.peak, 0.6));
        const o = ((GY - 1 - j) * GX + i) * 4;
        img.data[o] = Math.round(11 + v * tint[0]);
        img.data[o + 1] = Math.round(16 + v * tint[1]);
        img.data[o + 2] = Math.round(32 + v * tint[2]);
        img.data[o + 3] = 255;
      }
    }
    octx.putImageData(img, 0, 0);
    ctx.imageSmoothingEnabled = true;
    ctx.drawImage(off, 0, 0, w, h);

    // The transverse profile at a fixed distance: this is the quantity that
    // actually changes, and it is invisible in the field itself because the
    // interference fringes are what dominate the picture.
    const i0 = Math.round(((2.4 - X0) / (X1 - X0)) * (GX - 1));
    let m = 0;
    for (let j = 0; j < GY; j++) m = Math.max(m, f.data[j * GX + i0]);
    m = Math.max(m, 1e-9);
    ctx.beginPath();
    for (let j = 0; j < GY; j++) {
      const y = h - (j / (GY - 1)) * h;
      const x = 6 + (f.data[j * GX + i0] / m) * 60;
      if (j === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.strokeStyle = 'rgba(248,250,252,0.9)';
    ctx.lineWidth = 1.5;
    ctx.stroke();
    ctx.fillStyle = 'rgba(248,250,252,0.55)';
    ctx.font = '9px ui-monospace, monospace';
    ctx.fillText('|u| at x = 2.4', 6, h - 5);

    // the cap, drawn as a fan whose spokes are as long as each direction's weight
    const cx0 = w - 54;
    const cy0 = 22;
    const th = angles();
    const maxW = Math.max(...f.weights, 1e-9);
    for (let k = 0; k < th.length; k++) {
      const len = 6 + 16 * (f.weights[k] / maxW);
      ctx.beginPath();
      ctx.moveTo(cx0 + Math.cos(th[k]) * 4, cy0 - Math.sin(th[k]) * 4);
      ctx.lineTo(cx0 + Math.cos(th[k]) * (4 + len), cy0 - Math.sin(th[k]) * (4 + len));
      ctx.strokeStyle = k === (th.length - 1) / 2 ? 'rgba(254,240,138,0.95)' : 'rgba(125,211,252,0.55)';
      ctx.lineWidth = 1;
      ctx.stroke();
    }
  }

  function draw() {
    if (!canvasL || !canvasR) return;
    const th = angles();
    const ones = new Array(M).fill(1);
    const filtered = th.map((t) => Math.exp(-kappa * (1 - Math.cos(t))));
    const left = field(ones);
    const right = field(filtered);
    sigL = left.sigma;
    sigR = right.sigma;
    keepR = right.keep;
    paint(canvasL, left, [56, 120, 160]);
    paint(canvasR, right, [250, 190, 90]);
  }

  onMount(() => {
    draw();
    observer = new ResizeObserver(() => draw());
    for (const c of [canvasL, canvasR]) if (c) observer.observe(c);
    return () => observer?.disconnect();
  });

  $effect(() => {
    N;
    gamma;
    kappa;
    draw();
  });
</script>

<div class="rounded-lg border border-surface-700 bg-surface-950/40 overflow-hidden">
  <div class="relative">
    <canvas bind:this={canvasL} class="block w-full h-[190px]"></canvas>
    <div class="absolute top-2 left-2 text-[10px] uppercase tracking-wider text-surface-400">
      the cap, summed — diverges into the whole cap
    </div>
  </div>
  <div class="relative border-t border-surface-800">
    <canvas bind:this={canvasR} class="block w-full h-[190px]"></canvas>
    <div class="absolute top-2 left-2 text-[10px] uppercase tracking-wider text-warning-400">
      after translation by −iρ(1, ω) — collimated
    </div>
  </div>
  <div class="p-3 border-t border-surface-800 space-y-3">
    <div class="grid sm:grid-cols-3 gap-3">
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap">frequency N</span>
        <input type="range" min="16" max="80" step="1" bind:value={N} class="flex-1" />
        <span class="font-mono w-6 text-right">{N}</span>
      </label>
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap">cap γ</span>
        <input type="range" min="0.06" max="0.5" step="0.01" bind:value={gamma} class="flex-1" />
        <span class="font-mono w-10 text-right">{gamma.toFixed(2)}</span>
      </label>
      <label class="flex items-center gap-2 text-xs">
        <span class="text-surface-400 whitespace-nowrap">filter ρN</span>
        <input type="range" min="0" max="1200" step="1" bind:value={kappa} class="flex-1" />
        <span class="font-mono w-8 text-right">{kappa}</span>
      </label>
    </div>
    <div class="flex flex-wrap gap-x-6 gap-y-1 text-xs text-surface-400">
      <span>
        angular width, summed
        <span class="font-mono text-surface-200">{fmt(sigL, 3)}</span>
      </span>
      <span>
        angular width, filtered
        <span class="font-mono text-warning-300">{fmt(sigR, 3)}</span>
      </span>
      <span>
        predicted (ρN)<sup>−1/2</sup>
        <span class="font-mono text-surface-200">{kappa > 0.5 ? fmt(1 / Math.sqrt(kappa), 3) : '—'}</span>
      </span>
      <span>
        directions kept <span class="font-mono text-surface-200">{fmt(keepR, 1)}</span> of {M}
      </span>
    </div>
    <p class="text-xs text-surface-500 leading-relaxed">
      {M} plane waves with frequency N on a cap of half-width γ, drawn as |u| on a 300×168 grid over
      x ∈ [0, 3]. Top: the raw sum, fanning out across the whole cap. Bottom: the same sum after the
      paper's imaginary translation, which weights each mode by e<sup>−ρN(1−cos θ)</sup> — a Gaussian
      in direction with angular width (ρN)<sup>−1/2</sup>, drawn as the little fan in the corner.
      The fringes are untouched: the filter multiplies the sum, it does not square first. Push ρN far
      enough and the profile flattens rather than narrowing — a perfectly collimated beam is a single
      plane wave, which has no transverse structure at all. The filter trades transverse concentration
      for angular concentration, and the paper needs only the second. This is the object Section 2
      then turns into a conserved current.
    </p>
  </div>
</div>
