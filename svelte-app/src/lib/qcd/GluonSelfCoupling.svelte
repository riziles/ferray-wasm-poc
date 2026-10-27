<script lang="ts">
  import { onMount } from 'svelte';
  import { fit, clear, mulberry32, QUARK_COLOURS, ANTI_COLOURS, QUARK_NAMES } from './util';

  let selfCouple = $state(true);
  let canvas: HTMLCanvasElement;
  let raf = 0;
  let t = 0;
  let frame = 0;

  // the 8 gluon states: colour-antcolour pairs, minus the colourless trace
  const cells: { c: number; a: number; diag: boolean }[] = [];
  for (let i = 0; i < 3; i++)
    for (let j = 0; j < 3; j++) cells.push({ c: i, a: j, diag: i === j });

  let hover = $state<number | null>(null);

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h);
    t += 0.016;
    frame++;

    const cx = w / 2;
    const leftX = cx - Math.min(w * 0.34, 240);
    const rightX = cx + Math.min(w * 0.34, 240);
    const cy = h * 0.55;

    // quark / antiquark
    const drawQuark = (x: number, y: number, col: string, label: string) => {
      ctx.beginPath();
      ctx.arc(x, y, 18, 0, Math.PI * 2);
      ctx.fillStyle = col;
      ctx.shadowColor = col;
      ctx.shadowBlur = 18;
      ctx.fill();
      ctx.shadowBlur = 0;
      ctx.strokeStyle = '#0b1020';
      ctx.lineWidth = 2;
      ctx.stroke();
      ctx.fillStyle = 'rgba(11,16,32,0.85)';
      ctx.font = '10px ui-monospace, monospace';
      ctx.textAlign = 'center';
      ctx.fillText(label, x, y + 3.5);
    };

    drawQuark(leftX, cy, QUARK_COLOURS[0], 'q');
    drawQuark(rightX, cy, ANTI_COLOURS[0], 'q̄');

    ctx.font = '10px ui-monospace, monospace';
    ctx.textAlign = 'center';

    // field lines between the pair
    const lines = selfCouple ? 26 : 14;
    for (let n = 0; n < lines; n++) {
      const u = n / (lines - 1) - 0.5;
      const wob = Math.sin(t * 1.7 + n * 1.3) * (selfCouple ? 2.5 : 6);
      const spread = selfCouple ? 1 : 4.2;
      const midY = cy + u * (selfCouple ? 20 : 105) + wob;
      const cp1x = leftX + (rightX - leftX) * 0.32;
      const cp2x = leftX + (rightX - leftX) * 0.68;
      const sag = u * spread * 26 * Math.sin(t * 0.9 + n * 0.7) * 0.4;
      ctx.beginPath();
      ctx.moveTo(leftX + 18, cy);
      ctx.bezierCurveTo(cp1x, midY + sag, cp2x, midY - sag, rightX - 18, cy);
      const g = selfCouple ? 0.62 : 0.2;
      // self-coupling: bright, dense, coherent; free-field: faint, spread, drifting
      ctx.strokeStyle = selfCouple
        ? `rgba(56,189,248,${g * (0.65 + 0.35 * Math.sin(n * 0.8 + t * 2))})`
        : `rgba(148,163,184,${g * (0.4 + 0.6 * Math.abs(Math.sin(n * 1.7 + t * 0.5)))})`;
      ctx.lineWidth = selfCouple ? 1.5 : 1;
      ctx.stroke();

      // in non-abelian mode: lines can sprout side-branches (gluon-gluon vertices)
      if (selfCouple && n % 2 === 0) {
        const phase = (frame * 0.02 + n * 0.37) % 1;
        const bx = leftX + (rightX - leftX) * (0.25 + 0.5 * phase);
        const by = midY;
        const ang = Math.cos(t * 1.3 + n) * 2.4;
        const L = 30 * Math.sin(phase * Math.PI);
        if (L > 4) {
          ctx.beginPath();
          ctx.moveTo(bx, by);
          ctx.lineTo(bx + Math.cos(ang) * L, by + Math.sin(ang) * L);
          ctx.strokeStyle = `rgba(250,204,21,${0.55 * Math.sin(phase * Math.PI)})`;
          ctx.lineWidth = 1.2;
          ctx.stroke();
        }
      }
    }

    // labels
    ctx.fillStyle = 'rgba(148,163,184,0.85)';
    ctx.font = '11px ui-monospace, monospace';
    ctx.textAlign = 'center';
    ctx.fillText(selfCouple ? 'gluon self-coupling ON' : 'gluon self-coupling OFF (fake QED-like world)', cx, 20);
    ctx.fillStyle = 'rgba(100,116,139,0.9)';
    ctx.font = '10px ui-monospace, monospace';
    ctx.fillText(
      selfCouple ? 'field lines attract each other → they bundle into a tube' : 'field lines spread out → force weakens with distance, quarks escape',
      cx,
      h - 12
    );

    raf = requestAnimationFrame(draw);
  }

  onMount(() => {
    const ro = new ResizeObserver(() => {});
    ro.observe(canvas);
    draw();
    return () => {
      cancelAnimationFrame(raf);
      ro.disconnect();
    };
  });
</script>

<div class="card card-demo p-6 space-y-4">
  <div class="flex flex-wrap gap-3 items-center">
    <label class="flex items-center gap-2 cursor-pointer">
      <input type="checkbox" class="checkbox" bind:checked={selfCouple} />
      <span class="text-xs">gluons interact with gluons</span>
    </label>
    <span class="badge preset-tonal-warning text-xs">3-gluon and 4-gluon vertices exist</span>
  </div>

  <canvas bind:this={canvas} class="w-full rounded border border-surface-700" style="height:300px"
  ></canvas>

  <div>
    <div class="text-xs font-semibold text-surface-300 mb-2">
      The gluon colour matrix — 3 × 3 = 9, minus the one that would be colourless
    </div>
    <div class="grid grid-cols-3 gap-1.5 max-w-sm">
      {#each cells as cell, n}
        <button
          class="rounded px-2 py-2 text-[11px] font-mono border transition-all
            {cell.diag
            ? 'border-error-700/50 bg-error-950/30 text-error-300'
            : 'border-surface-600 bg-surface-800 text-surface-300'}"
          onmouseenter={() => (hover = n)}
          onmouseleave={() => (hover = null)}
          title={cell.diag
            ? 'A colourless gluon would mediate a long-range force and would be a singlet: it is not in the octet. It does not exist.'
            : 'carries colour ' + QUARK_NAMES[cell.c] + ' and anticolour ' + QUARK_NAMES[cell.a]}
        >
          <span style="color: {QUARK_COLOURS[cell.c]}">●</span>
          <span style="color: {ANTI_COLOURS[cell.a]}">●</span>
          <span class="text-surface-500"> {QUARK_NAMES[cell.c].slice(0, 1)}/{QUARK_NAMES[cell.a].slice(0, 1)}̄</span>
        </button>
      {/each}
    </div>
    <p class="text-xs text-surface-500 mt-2">
      The 8 physical gluons are the <em>traceless</em> combinations of these 9 labels — so the grid is
      a bookkeeping device, not a literal list. The red diagonal cells are the excluded trace state.
      A photon carries no charge, so photons ignore photons. A gluon carries colour, so gluons pull on
      gluons, and the QCD field equations have terms that are quadratic and cubic in the field. There
      is no superposition principle for the strong force: two solutions are not a solution.
    </p>
  </div>
</div>
