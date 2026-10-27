<script lang="ts">
  import { onMount } from 'svelte';
  import { fit, clear, clamp, QUARK_COLOURS, ANTI_COLOURS, mulberry32 } from './util';

  // "How many quarks are in a proton?" has no answer: it depends on the resolution.
  let logQ = $state(Math.log10(1)); // 0.3 .. 1000 GeV
  let canvas: HTMLCanvasElement;
  let raf = 0;
  let t = 0;

  const rnd = mulberry32(424242);

  interface Parton {
    x: number;
    y: number;
    angle: number;
    len: number;
    kind: 'valence' | 'gluon' | 'sea';
    colour: number;
    radius: number;
    vx: number;
    vy: number;
  }

  const partons: Parton[] = [];
  // three valence quarks, always there
  for (let i = 0; i < 3; i++) {
    partons.push({
      x: 0.5 + 0.22 * Math.cos((i * 2 * Math.PI) / 3 + 0.4),
      y: 0.5 + 0.22 * Math.sin((i * 2 * Math.PI) / 3 + 0.4),
      angle: rnd() * 6.28,
      len: 0.1 + rnd() * 0.1,
      kind: 'valence',
      colour: i,
      radius: 0.05,
      vx: (rnd() - 0.5) * 0.0006,
      vy: (rnd() - 0.5) * 0.0006
    });
  }
  // a fixed pool of sea + gluon partons, revealed progressively as you zoom in
  for (let i = 0; i < 160; i++) {
    const a = rnd() * 6.28;
    const r = 0.12 + 0.34 * Math.sqrt(rnd());
    partons.push({
      x: 0.5 + r * Math.cos(a),
      y: 0.5 + r * Math.sin(a),
      angle: rnd() * 6.28,
      len: 0.06 + rnd() * 0.12,
      kind: rnd() < 0.55 ? 'gluon' : 'sea',
      colour: Math.floor(rnd() * 3),
      radius: 0.012 + rnd() * 0.016,
      vx: (rnd() - 0.5) * 0.0016,
      vy: (rnd() - 0.5) * 0.0016
    });
  }

  // resolved fraction: grows like ln(Q/Λ)², the classic hadron-multiplicity behaviour
  const Q = $derived(Math.pow(10, logQ));
  const resolvedCount = $derived(
    Math.round(clamp(3 + 1.35 * Math.pow(Math.log(Q / 0.25), 2), 3, partons.length))
  );

  function draw() {
    if (!canvas) return;
    const { ctx, w, h } = fit(canvas);
    clear(ctx, w, h);
    t += 0.016;

    const R = Math.min(w, h) * 0.34;
    const cx = w / 2;
    const cy = h / 2;

    // the proton's edge — fuzzy, because there is no edge
    const g = ctx.createRadialGradient(cx, cy, R * 0.4, cx, cy, R);
    g.addColorStop(0, 'rgba(56,189,248,0.10)');
    g.addColorStop(0.85, 'rgba(56,189,248,0.06)');
    g.addColorStop(1, 'rgba(56,189,248,0)');
    ctx.beginPath();
    ctx.arc(cx, cy, R, 0, Math.PI * 2);
    ctx.fillStyle = g;
    ctx.fill();
    ctx.setLineDash([3, 6]);
    ctx.strokeStyle = 'rgba(56,189,248,0.2)';
    ctx.stroke();
    ctx.setLineDash([]);

    const visible = partons.slice(0, resolvedCount);

    // gluon links (a gluon is drawn as a spring between two partons)
    ctx.lineWidth = 1;
    visible.forEach((p, i) => {
      if (p.kind !== 'gluon') return;
      const q = visible[(i * 7 + 3) % visible.length];
      const x1 = cx + p.x * R;
      const y1 = cy + p.y * R;
      const x2 = cx + q.x * R;
      const y2 = cy + q.y * R;
      const dx = x2 - x1;
      const dy = y2 - y1;
      const L = Math.hypot(dx, dy);
      if (L < 4) return;
      const steps = Math.max(3, Math.round(L / 6));
      ctx.beginPath();
      ctx.moveTo(x1, y1);
      for (let s = 1; s <= steps; s++) {
        const u = s / steps;
        const nx = -dy / L;
        const ny = dx / L;
        const amp = 3 * Math.sin(u * Math.PI * steps) * Math.sin(t * 2 + i);
        ctx.lineTo(x1 + dx * u + nx * amp, y1 + dy * u + ny * amp);
      }
      ctx.strokeStyle = `rgba(250,204,21,${0.18 + 0.1 * Math.sin(t + i)})`;
      ctx.stroke();
    });

    // partons
    for (const p of visible) {
      p.x += p.vx;
      p.y += p.vy;
      if (p.x < 0.2 || p.x > 0.8) p.vx *= -1;
      if (p.y < 0.2 || p.y > 0.8) p.vy *= -1;
      const x = cx + p.x * R;
      const y = cy + p.y * R;

      if (p.kind === 'valence') {
        ctx.beginPath();
        ctx.arc(x, y, R * p.radius, 0, Math.PI * 2);
        ctx.fillStyle = QUARK_COLOURS[p.colour];
        ctx.shadowColor = QUARK_COLOURS[p.colour];
        ctx.shadowBlur = 16;
        ctx.fill();
        ctx.shadowBlur = 0;
      } else if (p.kind === 'sea') {
        ctx.beginPath();
        ctx.arc(x, y, R * p.radius, 0, Math.PI * 2);
        ctx.fillStyle = ANTI_COLOURS[p.colour];
        ctx.globalAlpha = 0.75;
        ctx.fill();
        ctx.globalAlpha = 1;
        ctx.beginPath();
        ctx.arc(x + R * p.radius * 2.2, y, R * p.radius, 0, Math.PI * 2);
        ctx.fillStyle = QUARK_COLOURS[p.colour];
        ctx.globalAlpha = 0.75;
        ctx.fill();
        ctx.globalAlpha = 1;
      } else {
        ctx.beginPath();
        ctx.arc(x, y, R * p.radius * 1.3, 0, Math.PI * 2);
        ctx.strokeStyle = 'rgba(250,204,21,0.8)';
        ctx.lineWidth = 1.2;
        ctx.stroke();
      }
    }

    // readout (top, so it never sits on top of the swarm)
    ctx.fillStyle = 'rgba(226,232,240,0.92)';
    ctx.font = '13px ui-monospace, monospace';
    ctx.textAlign = 'center';
    const gluons = visible.filter((p) => p.kind === 'gluon').length;
    const sea = visible.filter((p) => p.kind === 'sea').length * 2;
    ctx.fillText(`resolved partons: ${visible.length}  (3 valence · ${gluons} gluons · ${sea} sea quarks)`, w / 2, 20);
    ctx.fillStyle = 'rgba(100,116,139,0.9)';
    ctx.font = '11px ui-monospace, monospace';
    ctx.fillText(`resolution Q = ${Q < 10 ? Q.toFixed(2) : Q.toFixed(0)} GeV`, w / 2, 38);

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
  <label class="label">
    <span class="text-xs">zoom in (higher resolution ⇒ higher Q ⇒ more stuff)</span>
    <input type="range" class="input" min={Math.log10(0.3)} max={Math.log10(1000)} step="0.01" bind:value={logQ} />
  </label>

  <canvas bind:this={canvas} class="w-full rounded border border-surface-700" style="height:360px"
  ></canvas>

  <p class="text-xs text-surface-500">
    Ask “how many quarks are in a proton?” at low resolution and you get <strong>three</strong>. Ask
    again with a harder probe and you get three plus a swarm of gluons and quark–antiquark pairs that
    were not there before. There is no final answer, because “number of partons” is not a property of
    the proton — it is a property of your apparatus. The resolvable parton count grows roughly like
    ln² of the resolution: every factor of ten in Q roughly doubles the swarm, and it never stops. The
    proton is a divergent series wearing a particle's name.
  </p>
</div>
