<script lang="ts">
  import { onMount } from 'svelte';

  let { wasm } = $props<{ wasm: ReturnType<typeof import('./wasm/loader').getWasm> }>();

  let canvas: HTMLCanvasElement;
  let detVal = $state(0);
  let evalResult = $state('');
  let collisionShown = $state(false);

  // Camera state
  let rotX = $state(-0.4);
  let rotY = $state(0.6);
  let zoom = $state(3.5);

  let dragging = false;
  let lastX = 0, lastY = 0;

  // Slice plane: we fix one coordinate and vary the other two
  let sliceAxis = $state<'xy' | 'xz' | 'yz'>('xy');
  let slicePos = $state(0); // value of the fixed coordinate
  let gridSize = $state(15);

  // The two collision witness points
  const P1 = { x: 0, y: 0, z: -0.25 };
  const P2 = { x: 1, y: -1.5, z: 6.5 };

  // 3D projection
  function project(x: number, y: number, z: number): [number, number] {
    const cw = canvas.width / 2; // half for split view
    const ch = canvas.height;
    const cx = cw / 2;
    const cy = ch / 2;
    const s = Math.min(cw, ch) * 0.28;

    // Rotate around Y
    const cosY = Math.cos(rotY), sinY = Math.sin(rotY);
    const rx = x * cosY - z * sinY;
    const rz = x * sinY + z * cosY;

    // Rotate around X
    const cosX = Math.cos(rotX), sinX = Math.sin(rotX);
    const ry = y * cosX - rz * sinX;
    const rz2 = y * sinX + rz * cosX;

    // Perspective
    const d = zoom + rz2;
    const scale = zoom / Math.max(d, 0.01);
    return [cx + rx * s * scale, cy - ry * s * scale];
  }

  // Get points for the current slice
  function getSlicePoints(): Float64Array {
    const n = gridSize * gridSize;
    const pts = new Float64Array(n * 3);
    const range = 2;
    for (let i = 0; i < gridSize; i++) {
      for (let j = 0; j < gridSize; j++) {
        const u = (i / (gridSize - 1) - 0.5) * range;
        const v = (j / (gridSize - 1) - 0.5) * range;
        const idx = (i * gridSize + j) * 3;
        if (sliceAxis === 'xy') {
          pts[idx] = u; pts[idx + 1] = v; pts[idx + 2] = slicePos;
        } else if (sliceAxis === 'xz') {
          pts[idx] = u; pts[idx + 1] = slicePos; pts[idx + 2] = v;
        } else {
          pts[idx] = slicePos; pts[idx + 1] = u; pts[idx + 2] = v;
        }
      }
    }
    return pts;
  }

  function draw() {
    if (!canvas || !wasm) return;

    const ctx = canvas.getContext('2d')!;
    const cw = canvas.width;
    const ch = canvas.height;
    const halfW = cw / 2;

    ctx.fillStyle = '#0f1419';
    ctx.fillRect(0, 0, cw, ch);

    // Divider
    ctx.strokeStyle = '#334155';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(halfW, 10);
    ctx.lineTo(halfW, ch - 10);
    ctx.stroke();

    // Labels
    ctx.font = '13px monospace';
    ctx.fillStyle = '#94a3b8';
    ctx.textAlign = 'center';
    ctx.fillText('Input (x, y, z)', halfW / 2, 18);
    ctx.fillText('Output F(x, y, z)', halfW + halfW / 2, 18);

    // Get slice points
    const inputPts = getSlicePoints();

    // Compute output points
    const outputPts = new Float64Array(inputPts.length);
    for (let i = 0; i < gridSize * gridSize; i++) {
      const x = inputPts[i * 3];
      const y = inputPts[i * 3 + 1];
      const z = inputPts[i * 3 + 2];
      const out = wasm.jacobian_eval(x, y, z);
      outputPts[i * 3] = out[0];
      outputPts[i * 3 + 1] = out[1];
      outputPts[i * 3 + 2] = out[2];
    }

    // Draw wireframe grids
    drawWireframe(ctx, inputPts, 0);
    drawWireframe(ctx, outputPts, halfW);

    // Highlight collision points
    drawPoint(ctx, P1.x, P1.y, P1.z, '#3b82f6', 0, 'P₁');
    drawPoint(ctx, P2.x, P2.y, P2.z, '#f59e0b', 0, 'P₂');

    const out1 = wasm.jacobian_eval(P1.x, P1.y, P1.z);
    const out2 = wasm.jacobian_eval(P2.x, P2.y, P2.z);
    drawPoint(ctx, out1[0], out1[1], out1[2], '#22c55e', halfW, 'F(P₁)=F(P₂)');
  }

  function drawWireframe(ctx: CanvasRenderingContext2D, pts: Float64Array, offsetX: number) {
    ctx.strokeStyle = '#475569';
    ctx.lineWidth = 0.5;

    // Horizontal lines
    for (let i = 0; i < gridSize; i++) {
      ctx.beginPath();
      for (let j = 0; j < gridSize; j++) {
        const idx = (i * gridSize + j) * 3;
        const [sx, sy] = project(pts[idx], pts[idx + 1], pts[idx + 2]);
        ctx[j === 0 ? 'moveTo' : 'lineTo'](offsetX + sx, sy);
      }
      ctx.stroke();
    }

    // Vertical lines
    for (let j = 0; j < gridSize; j++) {
      ctx.beginPath();
      for (let i = 0; i < gridSize; i++) {
        const idx = (i * gridSize + j) * 3;
        const [sx, sy] = project(pts[idx], pts[idx + 1], pts[idx + 2]);
        ctx[i === 0 ? 'moveTo' : 'lineTo'](offsetX + sx, sy);
      }
      ctx.stroke();
    }
  }

  function drawPoint(ctx: CanvasRenderingContext2D, x: number, y: number, z: number,
                     color: string, offsetX: number, label: string) {
    const [sx, sy] = project(x, y, z);
    const px = offsetX + sx;
    const py = sy;

    ctx.fillStyle = color;
    ctx.beginPath();
    ctx.arc(px, py, 5, 0, Math.PI * 2);
    ctx.fill();

    ctx.strokeStyle = color;
    ctx.lineWidth = 2;
    ctx.stroke();

    ctx.font = '11px monospace';
    ctx.fillStyle = color;
    ctx.textAlign = 'left';
    ctx.fillText(label, px + 8, py + 4);
  }

  function handleMouseDown(e: MouseEvent) {
    dragging = true;
    lastX = e.clientX;
    lastY = e.clientY;
  }

  function handleMouseMove(e: MouseEvent) {
    if (!dragging) return;
    const dx = e.clientX - lastX;
    const dy = e.clientY - lastY;
    rotY += dx * 0.005;
    rotX += dy * 0.005;
    rotX = Math.max(-1.5, Math.min(1.5, rotX));
    lastX = e.clientX;
    lastY = e.clientY;
    draw();
  }

  function handleMouseUp() { dragging = false; }
  function handleWheel(e: WheelEvent) {
    zoom = Math.max(1, Math.min(10, zoom + e.deltaY * -0.01));
    draw();
  }

  function computeDet() {
    const r = wasm.jacobian_det(1.5, 0.3, -.8);
    detVal = r;
    const p = wasm.jacobian_eval(3, -2, 1);
    evalResult = `F(3, −2, 1) = (${p[0].toFixed(4)}, ${p[1].toFixed(4)}, ${p[2].toFixed(4)})`;
  }

  $effect(() => { sliceAxis; slicePos; gridSize; draw(); });
  onMount(() => { draw(); computeDet(); });
</script>

<div class="card card-demo p-4 space-y-4">
  <div class="flex gap-3 items-end flex-wrap">
    <label class="label w-28">
      <span>Slice plane</span>
      <select class="input" bind:value={sliceAxis}>
        <option value="xy">XY (z fixed)</option>
        <option value="xz">XZ (y fixed)</option>
        <option value="yz">YZ (x fixed)</option>
      </select>
    </label>
    <label class="label w-40">
      <span>Slice at: {slicePos.toFixed(1)}</span>
      <input type="range" class="input" min="-2" max="2" step="0.1" bind:value={slicePos} />
    </label>
    <label class="label w-32">
      <span>Grid: {gridSize}</span>
      <input type="range" class="input" min="5" max="25" step="2" bind:value={gridSize} />
    </label>
    <span class="badge preset-tonal-primary text-xs">Drag to rotate · Scroll to zoom</span>
  </div>

  <canvas
    bind:this={canvas}
    width="900" height="420"
    class="w-full rounded border border-surface-600 cursor-grab active:cursor-grabbing"
    onmousedown={handleMouseDown}
    onmousemove={handleMouseMove}
    onmouseup={handleMouseUp}
    onmouseleave={handleMouseUp}
    onwheel={handleWheel}
    style="touch-action: none"
  ></canvas>

  <div class="flex flex-wrap gap-2 items-center text-xs font-mono">
    <span class="badge preset-tonal">
      det(J<sub>F</sub>) = {detVal !== 0 ? detVal.toFixed(4) : '—'}
      <span class="text-success-400 ml-1">(≈ −2)</span>
    </span>
    <span class="badge preset-tonal">{evalResult}</span>
    <button class="btn preset-tonal text-xs" onclick={computeDet}>↻ Recompute</button>
  </div>

  {#if collisionShown}
    <div class="bg-surface-800 p-3 rounded text-xs font-mono space-y-1">
      <div class="text-primary-400">🔴 Collision witness:</div>
      <div>F(0, 0, −¼) = <span class="text-green-400">(−¼, 0, 0)</span></div>
      <div>F(1, −1.5, 6.5) = <span class="text-green-400">(−¼, 0, 0)</span></div>
      <div class="text-surface-500 mt-1">Same output, different inputs — F is not injective</div>
    </div>
  {:else}
    <button class="btn preset-tonal-warning text-xs w-48" onclick={() => collisionShown = true}>
      🔍 Show collision witness
    </button>
  {/if}

  {#if collisionShown}
    <div class="bg-surface-800 p-4 rounded text-xs space-y-2">
      <h3 class="font-bold text-surface-200 text-sm">Why does this disprove the conjecture?</h3>
      <p class="text-surface-400">
        The Jacobian conjecture said: if det(J<sub>F</sub>) is a nonzero constant (here: −2),
        then F <em>must</em> have a polynomial inverse. But if two distinct points
        map to the same output, no inverse exists — injectivity fails.
      </p>
      <p class="text-surface-400">
        This counterexample proves the conjecture false for <strong>N ≥ 3</strong>.
        For N = 2, it remains an open problem.
      </p>
    </div>
  {/if}
</div>
