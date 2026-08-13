<script lang="ts">
  import { onMount } from 'svelte';
  import type { WasmApi } from './wasm/loader';

  let { wasm } = $props<{ wasm: WasmApi }>();

  // ── shape ─
  let profile = $state(0);      // 0 Ellis catenoid, 1 Flamm paraboloid
  let throat = $state(0.3);     // throat/sheet radius ratio q
  let stretch = $state(1.4);    // vertical tube stretch
  let weld = $state(1.0);       // 0 separate sheets … 1 welded throat

  // ── view / style ──
  let colorMode = $state(0);    // 0 classic, 1 spectrum
  let viewMode = $state(2);     // 0 3D, 1 flat chart, 2 both
  let wireframe = $state(false);
  let traveler = $state(true);
  let autoRotate = $state(true);
  let quality = $state(1);      // 0 low, 1 med, 2 high
  let zoom = $state(1.0);

  // ── runtime (not reactive — read per-frame) ──
  let yaw = 0.65;
  let pitch = 0.42;
  let tSec = 0;

  // ── stats ──
  let fps = $state(0);
  let ms = $state(0);
  let nQuads = $state(0);

  let canvas: HTMLCanvasElement;
  let rafId = 0;
  let dragging = false;
  let lx = 0, ly = 0;

  const QUAL: [number, number][] = [[16, 56], [26, 80], [38, 104]];
  const CW = 960, CH = 620;

  function draw(list: Float64Array) {
    const ctx = canvas.getContext('2d')!;
    ctx.fillStyle = '#0b1020';
    ctx.fillRect(0, 0, CW, CH);

    let i = 0, q = 0;
    while (i < list.length) {
      const tag = list[i];
      if (tag === 0) {
        // quad: 13 floats
        const r = list[i + 10] | 0, g = list[i + 11] | 0, b = list[i + 12] | 0;
        const fill = `rgb(${r},${g},${b})`;
        ctx.beginPath();
        ctx.moveTo(list[i + 2], list[i + 3]);
        ctx.lineTo(list[i + 4], list[i + 5]);
        ctx.lineTo(list[i + 6], list[i + 7]);
        ctx.lineTo(list[i + 8], list[i + 9]);
        ctx.closePath();
        if (wireframe) {
          ctx.strokeStyle = 'rgba(34,211,238,0.55)';
          ctx.lineWidth = 0.7;
          ctx.stroke();
        } else {
          ctx.fillStyle = fill;
          ctx.fill();
          // same-color stroke hides antialiasing seams between quads
          ctx.strokeStyle = fill;
          ctx.lineWidth = 0.8;
          ctx.stroke();
        }
        q++;
        i += 13;
      } else if (tag === 1) {
        // line: 10 floats
        ctx.strokeStyle = `rgb(${list[i + 6] | 0},${list[i + 7] | 0},${list[i + 8] | 0})`;
        ctx.lineWidth = list[i + 9];
        ctx.beginPath();
        ctx.moveTo(list[i + 2], list[i + 3]);
        ctx.lineTo(list[i + 4], list[i + 5]);
        ctx.stroke();
        i += 10;
      } else {
        // dot: 8 floats
        ctx.fillStyle = `rgb(${list[i + 5] | 0},${list[i + 6] | 0},${list[i + 7] | 0})`;
        ctx.beginPath();
        ctx.arc(list[i + 2], list[i + 3], Math.max(0.5, list[i + 4]), 0, 6.2832);
        ctx.fill();
        i += 8;
      }
    }
    nQuads = q;
  }

  let last = 0;
  let fpsEma = 60;
  function loop(now: number) {
    rafId = requestAnimationFrame(loop);
    if (last > 0) {
      const dt = Math.min(0.05, (now - last) / 1000);
      fpsEma = fpsEma * 0.92 + (1 / Math.max(dt, 1e-4)) * 0.08;
      fps = Math.round(fpsEma);
      if (autoRotate && !dragging) yaw += dt * 0.45;
      if (traveler) tSec += dt;
    }
    last = now;

    const [rings, segs] = QUAL[quality];
    const t0 = performance.now();
    const list = wasm.wh_render(
      CW, CH, yaw, pitch, zoom,
      profile, throat, stretch, weld,
      colorMode, viewMode, rings, segs,
      traveler ? tSec : -1,
    );
    ms = performance.now() - t0;
    draw(list);
  }

  // ── pointer interaction: drag to orbit, wheel to zoom ──
  function onPointerDown(e: PointerEvent) {
    dragging = true;
    lx = e.clientX; ly = e.clientY;
    canvas.setPointerCapture(e.pointerId);
  }
  function onPointerMove(e: PointerEvent) {
    if (!dragging) return;
    yaw += (e.clientX - lx) * 0.006;
    pitch = Math.min(1.45, Math.max(0.05, pitch + (e.clientY - ly) * 0.006));
    lx = e.clientX; ly = e.clientY;
  }
  function onPointerUp() { dragging = false; }
  function onWheel(e: WheelEvent) {
    e.preventDefault();
    zoom = Math.min(2.2, Math.max(0.5, zoom * (e.deltaY > 0 ? 0.93 : 1.07)));
  }

  onMount(() => {
    rafId = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(rafId);
  });
</script>

<div class="card card-demo p-6 space-y-6">

  <!-- controls -->
  <div class="flex gap-3 items-end flex-wrap">
    <label class="label w-56">
      <span>Embedding profile</span>
      <select class="input" bind:value={profile}>
        <option value={0}>Ellis catenoid (traversable)</option>
        <option value={1}>Flamm paraboloid (Schwarzschild)</option>
      </select>
    </label>
    <label class="label w-48">
      <span>View</span>
      <select class="input" bind:value={viewMode}>
        <option value={0}>3D embedding</option>
        <option value={1}>flat annulus chart</option>
        <option value={2}>both side by side</option>
      </select>
    </label>
    <label class="label w-40">
      <span>Throat ratio b₀/R: {throat.toFixed(2)}</span>
      <input type="range" class="input" min="0.08" max="0.55" step="0.01" bind:value={throat} />
    </label>
    <label class="label w-40">
      <span>Tube stretch: {stretch.toFixed(2)}×</span>
      <input type="range" class="input" min="0.4" max="2.8" step="0.05" bind:value={stretch} />
    </label>
    <label class="label w-44">
      <span>Construction (weld): {(weld * 100).toFixed(0)}%</span>
      <input type="range" class="input" min="0" max="1" step="0.01" bind:value={weld} />
    </label>
    <label class="label w-40">
      <span>Zoom: {zoom.toFixed(2)}×</span>
      <input type="range" class="input" min="0.5" max="2.2" step="0.05" bind:value={zoom} />
    </label>
    <label class="label w-36">
      <span>Mesh quality</span>
      <select class="input" bind:value={quality}>
        <option value={0}>low</option>
        <option value={1}>medium</option>
        <option value={2}>high</option>
      </select>
    </label>
  </div>

  <div class="flex gap-2 items-center flex-wrap">
    <button class="btn preset-tonal-surface" onclick={() => { colorMode = colorMode === 0 ? 1 : 0; }}>
      {colorMode === 0 ? '🎨 classic palette' : '🌈 spectrum palette'}
    </button>
    <button class="btn preset-tonal-surface" onclick={() => { wireframe = !wireframe; }}>
      {wireframe ? '▦ wireframe on' : '◼ shaded'}
    </button>
    <button class="btn preset-tonal-surface" onclick={() => { traveler = !traveler; }}>
      {traveler ? '☄ traveler on' : '☄ traveler off'}
    </button>
    <button class="btn preset-tonal-surface" onclick={() => { autoRotate = !autoRotate; }}>
      {autoRotate ? '🔄 auto-rotate on' : '🔄 auto-rotate off'}
    </button>
    <button class="btn preset-tonal-surface" onclick={() => { yaw = 0.65; pitch = 0.42; zoom = 1.0; }}>↺ reset view</button>
    <span class="badge preset-tonal-primary">{fps} fps</span>
    <span class="badge preset-tonal-warning">{ms.toFixed(1)} ms/frame (WASM)</span>
    <span class="badge preset-tonal-surface">{nQuads} quads</span>
  </div>

  <div class="space-y-2">
    <p class="text-sm text-surface-300 font-medium">
      Drag to orbit · scroll to zoom. The <strong>weld</strong> slider replays the textbook
      construction: two flat sheets with holes cut out → rims identified → one continuous
      manifold. The <strong>traveler</strong> rides the surface straight through the throat —
      and the <strong>flat annulus chart</strong> shows the same surface unrolled isometrically:
      θ preserved, height replaced by arc length, so the whole manifold becomes one ring
      (inner rim = bottom universe, middle circle = throat, outer rim = top universe).
    </p>
    <canvas
      bind:this={canvas}
      width={CW}
      height={CH}
      class="rounded border border-surface-700 bg-surface-950/40 w-full cursor-grab active:cursor-grabbing"
      style="touch-action:none;"
      onpointerdown={onPointerDown}
      onpointermove={onPointerMove}
      onpointerup={onPointerUp}
      onpointercancel={onPointerUp}
      onwheel={onWheel}
    ></canvas>
  </div>

</div>
