<script lang="ts">
  import ColourCharge from '$lib/qcd/ColourCharge.svelte';
  import GluonSelfCoupling from '$lib/qcd/GluonSelfCoupling.svelte';
  import FluxTube from '$lib/qcd/FluxTube.svelte';
  import RunningCoupling from '$lib/qcd/RunningCoupling.svelte';
  import VacuumStructure from '$lib/qcd/VacuumStructure.svelte';
  import ProtonMass from '$lib/qcd/ProtonMass.svelte';
  import PartonZoom from '$lib/qcd/PartonZoom.svelte';
  import DiagramExplosion from '$lib/qcd/DiagramExplosion.svelte';
  import LieMeter from '$lib/qcd/LieMeter.svelte';

  let scrollProgress = $state(0);
  let descend = $state(0);

  function onScroll() {
    const doc = document.documentElement;
    const max = doc.scrollHeight - window.innerHeight;
    scrollProgress = max > 0 ? doc.scrollTop / max : 0;
    descend = Math.round(scrollProgress * 100);
  }

  import { onMount } from 'svelte';
  onMount(() => {
    window.addEventListener('scroll', onScroll, { passive: true });
    onScroll();
    return () => window.removeEventListener('scroll', onScroll);
  });

  const acts = [
    { n: 1, title: 'Colour is a charge that isn’t a colour', lie: 78 },
    { n: 2, title: 'The gluon field pulls on itself', lie: 64 },
    { n: 3, title: 'You can never see a quark', lie: 58 },
    { n: 4, title: 'The force gets weaker when you squeeze', lie: 52 },
    { n: 5, title: 'The vacuum is a material', lie: 41 },
    { n: 6, title: 'You are 99% strong force', lie: 33 },
    { n: 7, title: 'A proton has no particle count', lie: 24 },
    { n: 8, title: 'The maths does not converge', lie: 12 },
    { n: 9, title: 'The theory is not known to exist', lie: 4 }
  ];
</script>

<svelte:window on:scroll={onScroll} />

<div class="fixed top-0 left-0 right-0 h-0.5 z-20 bg-surface-800">
  <div class="h-full bg-gradient-to-r from-primary-500 via-warning-500 to-error-500" style="width: {descend}%"></div>
</div>

<div class="container mx-auto px-4 py-8 max-w-4xl space-y-16 pb-24">
  <!-- ───────────────────────── hero ───────────────────────── -->
  <header class="space-y-6">
    <div class="flex flex-wrap items-center gap-2">
      <span class="badge preset-filled-primary-500">interactive explainer</span>
      <span class="badge preset-tonal-warning">no WASM required</span>
      <span class="badge preset-tonal">pure TypeScript</span>
    </div>
    <h1 class="h1 leading-tight">
      🌈 Quantum Chromodynamics
      <span class="block text-surface-400 text-2xl md:text-3xl mt-2">
        eight gluons, no free quarks, and a theory that does not quite exist
      </span>
    </h1>
    <p class="text-lg text-surface-300 max-w-3xl">
      QCD is the theory of the strong force: the thing that holds quarks into protons, protons into
      nuclei, and nuclei into you. It is also the least intuitive theory humanity has ever built and
      kept. This page walks from the version you may have heard of (“quarks are held together by
      gluons, like little springs”) to the version that is actually true, which is much worse.
    </p>

    <!-- the honesty contract -->
    <div class="rounded-lg border border-warning-700/50 bg-warning-950/20 p-5 space-y-3">
      <h2 class="font-bold text-warning-300 flex items-center gap-2">
        <span>⚠️</span> Everything you are about to look at is a cartoon
      </h2>
      <p class="text-sm text-surface-300">
        There is no honest picture of quantum chromodynamics. Not a hard one, not an advanced one —
        <em>none</em>. Colour charge is not a colour, quarks are not balls, gluons are not springs,
        flux tubes are not tubes, and the vacuum is not empty. Every visual below is a projection of a
        complex-valued non-abelian gauge field configuration onto two dimensions and your visual cortex,
        and it loses almost everything on the way.
      </p>
      <p class="text-sm text-surface-400">
        So each section carries a <strong>picture honesty</strong> score. It starts around 78% — “this
        is a legitimate mnemonic” — and ends around 4% — “the diagram is a note that a number exists”.
        The physics gets <em>more</em> accurate as the pictures get <em>worse</em>. That is not a
        failure of this page. That is QCD.
      </p>
    </div>

    <!-- contents -->
    <nav class="rounded-lg border border-surface-700 bg-surface-800/40 p-4">
      <div class="text-xs uppercase tracking-wider text-surface-500 mb-2">Descent</div>
      <ol class="grid sm:grid-cols-2 gap-x-6 gap-y-1 text-sm">
        {#each acts as act}
          <li class="flex items-start gap-2 text-surface-400">
            <span class="font-mono text-surface-600 w-5 shrink-0">{act.n}.</span>
            <span class="flex-1">{act.title}</span>
            <span class="font-mono text-[10px] text-surface-600 shrink-0">{act.lie}%</span>
          </li>
        {/each}
      </ol>
    </nav>
  </header>

  <!-- ───────────────────────── act 1 ───────────────────────── -->
  <section class="space-y-4">
    <div class="flex items-center justify-between flex-wrap gap-2">
      <h2 class="h2">1. Colour is a charge that isn’t a colour</h2>
      <LieMeter honesty={78} />
    </div>
    <p class="text-surface-300">
      Quarks come in three varieties that physicists called red, green and blue, because they needed a
      word and colour was available. They are not colours. They are the three components of a
      <span class="term" data-tip="A vector of three complex numbers that transforms under the group SU(3).">complex 3-vector</span>,
      and the only rule that matters is: <strong>anything you can isolate must be colour-neutral</strong>.
      Three quarks, one of each colour: a baryon (proton, neutron). A quark and an antiquark with
      opposite colours: a meson (pion). Nothing else has ever been caught alone.
    </p>
    <p class="text-surface-400 text-sm">
      Now the part that is usually skipped. “Red” is a <em>basis choice</em>. You can rotate colour at
      every point in space independently — apply a different SU(3) matrix per point — and the physics
      is unchanged. This is a local gauge symmetry, which is a fancy way of saying that the description
      has more degrees of freedom than the world. The gluon field exists precisely to soak up those
      redundant choices. It is not “the thing that pushes quarks”; it is the price of being allowed to
      describe the same quark in infinitely many ways.
    </p>
    <ColourCharge />
    <p class="text-sm text-surface-400 border-l-2 border-surface-600 pl-4">
      <strong>What the picture lied about:</strong> the quarks are drawn as circles at fixed positions,
      in a shaped region called “a proton”. Real quarks are excitations of a field with no position
      until you measure one, and the proton has no surface. What survived: the invariant number
      |ε<sub>ijk</sub>q<sup>i</sup>q<sup>j</sup>q<sup>k</sup>|² = 1, and the fact that rotating the
      basis changes nothing observable.
    </p>
  </section>

  <!-- ───────────────────────── act 2 ───────────────────────── -->
  <section class="space-y-4">
    <div class="flex items-center justify-between flex-wrap gap-2">
      <h2 class="h2">2. The gluon field pulls on itself</h2>
      <LieMeter honesty={64} />
    </div>
    <p class="text-surface-300">
      Electromagnetism is easy in one specific way: the photon carries no electric charge, so at
      leading order photons pass through photons. You can superpose two light beams and get two light
      beams.
    </p>
    <p class="text-surface-300">
      Gluons carry colour. There are eight of them (3 × 3 colour–anticolour labels, minus the one
      combination that would be colourless), and because they carry the charge of their own force, they
      couple to each other. The field equations acquire terms quadratic and cubic in the field —
      “3-gluon” and “4-gluon” vertices. Two gluon field configurations are <em>not</em> a gluon field
      configuration in any useful sense. There is no superposition principle for the strong force. The
      equations are nonlinear in the way fluid dynamics is nonlinear, and they produce the same kind of
      behaviour: turbulence, confinement, and structure that nobody can write down in closed form.
    </p>
    <GluonSelfCoupling />
    <p class="text-sm text-surface-400 border-l-2 border-surface-600 pl-4">
      <strong>What the picture lied about:</strong> the “field lines” are a 1970s heuristic borrowed
      from magnetostatics. In the real theory there are no lines, only a Lie-algebra-valued connection
      A<sub>μ</sub><sup>a</sup>(x) with a = 1…8 at each point, and the “bundling” is a statement about
      the energy of a trial configuration.
    </p>
  </section>

  <!-- ───────────────────────── act 3 ───────────────────────── -->
  <section class="space-y-4">
    <div class="flex items-center justify-between flex-wrap gap-2">
      <h2 class="h2">3. You can never see a quark</h2>
      <LieMeter honesty={58} />
    </div>
    <p class="text-surface-300">
      Pull two quarks apart and the energy stored between them grows <em>linearly</em>, forever:
      V(r) ≈ σr with σ ≈ 0.9 GeV per femtometre. That is a constant force of about 150 kilonewtons,
      the weight of fifteen tonnes, and it does not fall off with distance. In every other force you
      have ever met, the field spreads out and weakens. Here it collapses into a narrow tube of field
      energy that refuses to spread.
    </p>
    <p class="text-surface-300">
      So confinement is not “quarks are stuck”. It is: <em>the field will not let the energy go
      anywhere except into making more quarks</em>. At about 1.2 fm the stored energy exceeds
      2m<sub>q</sub>c², and the tube snaps by creating a fresh quark–antiquark pair out of the vacuum.
      You started with two quarks and you now have four, in two colour-neutral mesons, and the original
      quarks are still trapped. Repeat forever. This is why every attempt to isolate a quark produces a
      shower of hadrons instead.
    </p>
    <FluxTube />
    <p class="text-sm text-surface-400 border-l-2 border-super-600 pl-4">
      <strong>What the picture lied about:</strong> the tube is a metaphor for the behaviour of a
      Wilson loop, and the “snap” is a non-perturbative transition that no diagram describes. The
      numbers (σ, the ~1.2 fm breaking length) are real.
    </p>
  </section>

  <!-- ───────────────────────── act 4 ───────────────────────── -->
  <section class="space-y-4">
    <div class="flex items-center justify-between flex-wrap gap-2">
      <h2 class="h2">4. The force gets weaker when you squeeze</h2>
      <LieMeter honesty={52} />
    </div>
    <p class="text-surface-300">
      Here is the fact that won the 2004 Nobel Prize and still sounds like a typo. The strong coupling
      is not a constant. It <em>runs</em>, and it runs backwards compared to every coupling you know.
      Probe a quark gently and the strong force is enormous. Probe it violently — hit it with a very
      high-energy particle, which means looking at very short distances — and the coupling shrinks
      toward zero. Inside a proton, quarks rattle around as if they were free particles. Deep
      inelastic scattering experiments in the late 1960s saw exactly this and had no idea what to make
      of it.
    </p>
    <p class="text-surface-300">
      This is <strong>asymptotic freedom</strong>, and it is why the phrase “quarks inside a proton” is
      not an oxymoron: the quarks really are nearly free, and simultaneously absolutely unremovable.
      The same coupling is weak at short range and infinite at long range. It is one force doing both
      of those things at once, because “the coupling” was never a number — it is a function of the
      scale you ask about.
    </p>
    <RunningCoupling />
    <p class="text-sm text-surface-400 border-l-2 border-surface-600 pl-4">
      <strong>What the picture lied about:</strong> the curve is a genuine two-loop numerical
      integration, so if anything is accurate here it is this. It is also where the honest answer
      becomes “we don't have one”: at low Q the curve goes off-scale because perturbation theory has
      stopped existing, and the plot has no way to say what replaces it.
    </p>
  </section>

  <!-- ───────────────────────── act 5 ───────────────────────── -->
  <section class="space-y-4">
    <div class="flex items-center justify-between flex-wrap gap-2">
      <h2 class="h2">5. The vacuum is a material</h2>
      <LieMeter honesty={41} />
    </div>
    <p class="text-surface-300">
      In classical physics the vacuum is nothing. In QCD the vacuum is a medium with a density, a
      pressure, a structure, and a phase diagram. It contains a non-zero density of quark–antiquark
      pairs (the <span class="term" data-tip="⟨q̄q⟩ ≈ −(250 MeV)³ — the chiral condensate, the order parameter of spontaneous chiral symmetry breaking.">chiral condensate</span>),
      a non-zero gluon field strength even with no particles present (the gluon condensate), and lumps
      of topological charge called instantons through which the field <em>tunnels</em> between
      configurations that cannot be continuously deformed into each other.
    </p>
    <p class="text-surface-300">
      The vacuum is also a <em>superconductor</em> of a strange kind — the dual superconductor picture
      says colour magnetic monopoles condense, and that the flux tubes of act 3 are the resulting
      vortices, exactly as magnetic field lines are squeezed into tubes inside a superconductor. Whether
      this is literally the mechanism is still contested. That it produces the right physics is not.
    </p>
    <VacuumStructure />
    <div class="rounded-lg border border-error-800/50 bg-error-950/20 p-4 text-sm text-surface-300 space-y-2">
      <div class="font-semibold text-error-300">And now the part that is genuinely deranged</div>
      <p>
        If you add up the energy density of these condensates at face value, the vacuum should weigh
        enough to curve the universe into a closed ball. The observed dark-energy density is about
        10<sup>44</sup> times smaller than the naive QCD contribution. Something cancels it to forty-four
        decimal places, and nobody knows what. This is not a footnote; it is the
        <strong>cosmological constant problem</strong>, and it is the clearest sign that we are missing
        something enormous. QCD is one of the two places the offending number comes from.
      </p>
    </div>
  </section>

  <!-- ───────────────────────── act 6 ───────────────────────── -->
  <section class="space-y-4">
    <div class="flex items-center justify-between flex-wrap gap-2">
      <h2 class="h2">6. You are 99% strong force</h2>
      <LieMeter honesty={33} />
    </div>
    <p class="text-surface-300">
      The Higgs mechanism gives quarks their mass. For the up and down quarks, that is about 2.2 and
      4.7 MeV — a rounding error. A proton weighs 938.272 MeV. The other 99.04% is not “stuff”. It is
      the kinetic and field energy of quarks and gluons inside the proton, converted to mass by
      E = mc². Your body's mass, at the deepest level anyone can currently check, is almost entirely
      the energy of the strong force refusing to let three quarks apart.
    </p>
    <ProtonMass />
    <p class="text-surface-400 text-sm">
      The rigorous statement is the trace anomaly: the proton's mass comes from the breaking of scale
      invariance in QCD, which is itself an effect of the running coupling from act 4. Mass is not a
      substance here. Mass is a symptom of a theory that has no intrinsic length scale and has to invent
      one, and that invented scale is Λ<sub>QCD</sub> ≈ 200 MeV. Every visible kilogram in the universe
      is a consequence of a numerical accident in the running of a coupling constant.
    </p>
  </section>

  <!-- ───────────────────────── act 7 ───────────────────────── -->
  <section class="space-y-4">
    <div class="flex items-center justify-between flex-wrap gap-2">
      <h2 class="h2">7. A proton has no particle count</h2>
      <LieMeter honesty={24} />
    </div>
    <p class="text-surface-300">
      “A proton is three quarks” is a statement with a hidden qualifier: <em>at low resolution</em>.
      Ask the same question with a harder probe and the proton contains three valence quarks plus an
      ever-growing swarm of gluons and quark–antiquark pairs that were not in the previous answer. The
      number of partons grows roughly like ln² of the resolution. It does not converge. There is no
      fact of the matter about how many quarks are in a proton.
    </p>
    <PartonZoom />
    <div class="grid sm:grid-cols-2 gap-3 text-xs">
      <div class="bg-surface-800/60 rounded p-3 space-y-1">
        <div class="font-semibold text-surface-300">The proton spin crisis</div>
        <div class="text-surface-500">
          The quarks' spins account for only ~30% of the proton's spin. The rest has to come from
          gluon spin and orbital angular momentum of a swirling relativistic fluid. The “simplest
          possible” explanation — that the quarks carry all of it — was ruled out by the EMC
experiment in 1988, and the accounting is still not
          settled.
        </div>
      </div>
      <div class="bg-surface-800/60 rounded p-3 space-y-1">
        <div class="font-semibold text-surface-300">The proton has no radius either</div>
        <div class="text-surface-500">
          The “proton radius” came out 4% smaller in muonic hydrogen than in electronic hydrogen,
          kicking off a decade-long crisis. A new generation of electron-scattering measurements has
          largely closed the gap — a reminder that these quantities are definitions as much as
          measurements.
        </div>
      </div>
    </div>
  </section>

  <!-- ───────────────────────── act 8 ───────────────────────── -->
  <section class="space-y-4">
    <div class="flex items-center justify-between flex-wrap gap-2">
      <h2 class="h2">8. The maths does not converge</h2>
      <LieMeter honesty={12} />
    </div>
    <p class="text-surface-300">
      The only reason anyone can compute anything in particle physics is perturbation theory: expand
      the answer in powers of the coupling and add up Feynman diagrams. It works spectacularly for
      quantum electrodynamics, where the expansion is at least well-behaved.
    </p>
    <p class="text-surface-300">
      For QCD at low energies it is worse than useless in a specific, quantifiable way. The series is
      <em>asymptotic</em>: the number of diagrams at order n grows like n!, so the terms grow like
      (α·n)ⁿ. Adding diagrams makes the answer worse after a point. There is an irreducible minimum
      error of order e<sup>−1/α</sup> that no cleverness removes. Dyson saw the reason in 1952: a
      convergent series would have to remain finite at negative coupling, but negative coupling means
      like charges attract, and the theory must fall apart there.
    </p>
    <DiagramExplosion />
    <p class="text-sm text-surface-400">
      Which leaves lattice QCD: replace spacetime with a 4-dimensional grid, put a 3×3 unitary matrix
      on every link (that is 8 real numbers × 4 directions per site), and integrate over all of them
      with Monte Carlo. This is how the proton mass and the hadron spectrum are actually computed from
      first principles, on machines that draw megawatts. It also has a wall called the
      <strong>sign problem</strong>: at non-zero baryon density the integrand stops being positive and
      the sampling fails. So we cannot compute the interior of a neutron star from first principles,
      not because we lack the hardware but because we lack an algorithm.
    </p>
  </section>

  <!-- ───────────────────────── act 9 ───────────────────────── -->
  <section class="space-y-4">
    <div class="flex items-center justify-between flex-wrap gap-2">
      <h2 class="h2">9. The theory is not known to exist</h2>
      <LieMeter honesty={4} />
    </div>
    <p class="text-surface-300">
      Here is where this stops being an explainer and becomes a confession.
    </p>
    <div class="space-y-3">
      <div class="rounded-lg border border-surface-700 bg-surface-800/50 p-4">
        <div class="font-semibold text-surface-200 mb-1">
          Nobody has proved that QCD as a mathematical object exists — nor that it has a mass gap.
        </div>
        <p class="text-sm text-surface-400">
          “Yang–Mills existence and mass gap” is one of the Clay Millennium Prize problems. In plain
          terms: construct a quantum field theory with a non-abelian gauge group that satisfies the
          axioms, and prove that its lightest excitation has strictly positive mass. Both are open.
          The million dollars is unclaimed. We have been computing with this theory since 1973: we can
          measure the proton's mass to nine significant figures, and predict it from the equations to
          at the percent level, without being able to prove that the object we are computing with
          exists.
        </p>
      </div>
      <div class="rounded-lg border border-surface-700 bg-surface-800/50 p-4">
        <div class="font-semibold text-surface-200 mb-1">Confinement is not proved either.</div>
        <p class="text-sm text-surface-400">
          It is not “known” from the equations that quarks are confined. It is overwhelming numerical
          evidence plus fifty years of experiment. There is no proof. The Clay problem above is
          essentially the demand for one.
        </p>
      </div>
      <div class="rounded-lg border border-surface-700 bg-surface-800/50 p-4">
        <div class="font-semibold text-surface-200 mb-1">
          The theory contains ghosts that are not particles and cannot be removed.
        </div>
        <p class="text-sm text-surface-400">
          To quantise a non-abelian gauge theory at all, you must add Faddeev–Popov ghost fields:
          scalar fields with the wrong statistics (fermionic), violating the spin–statistics theorem,
          which exist only inside loops and never appear as asymptotic states. They are not
          “undiscovered particles”. They are a bookkeeping device that the mathematics refuses to do
          without. And the gauge-fixing procedure they rescue is itself ambiguous — Gribov copies mean
          there is no globally valid way to fix the gauge at all, which is one of the reasons the
          rigorous construction above is so hard.
        </p>
      </div>
      <div class="rounded-lg border border-surface-700 bg-surface-800/50 p-4">
        <div class="font-semibold text-surface-200 mb-1">
          And there is a knob in the theory that should be enormous and is instead zero.
        </div>
        <p class="text-sm text-surface-400">
          QCD's Lagrangian permits a term that violates CP symmetry, with a coefficient θ. It is
          perfectly legal, contributes to the neutron's electric dipole moment, and is measured to be
          |θ| &lt; 10<sup>−10</sup>. There is no known reason for it to be anywhere near that small. The
          <strong>strong CP problem</strong> is the reason people spent forty years looking for axions —
          an entire hypothetical particle invented to explain why a number in an unrelated theory is
          zero. It has not been found.
        </p>
      </div>
      <div class="rounded-lg border border-surface-700 bg-surface-800/50 p-4">
        <div class="font-semibold text-surface-200 mb-1">
          The vacuum has infinitely many ground states and the real one is a superposition of all of
          them.
        </div>
        <p class="text-sm text-surface-400">
          Non-abelian gauge theories have a winding number: field configurations that cannot be
          continuously unwound, classified by an integer. The true vacuum is
          |θ⟩ = Σ<sub>n</sub> e<sup>inθ</sup>|n⟩ — an infinite quantum superposition over every
          topological sector, with a phase you cannot compute and a parameter you cannot explain.
          The object you are breathing is a superposition of infinitely many distinct vacuums.
        </p>
      </div>
      <div class="rounded-lg border border-surface-700 bg-surface-800/50 p-4">
        <div class="font-semibold text-surface-200 mb-1">
          Chiral symmetry breaking, and the reason the pion is light
        </div>
        <p class="text-sm text-surface-400">
          In the limit of massless quarks, QCD has an exact symmetry relating left- and right-handed
          quarks. The vacuum breaks it spontaneously — the same mechanism that gives superconductors
          their behaviour — and the pion is the resulting (pseudo-)Goldstone boson. That is why the pion
          is 140 MeV while the proton is 938: it is light <em>because</em> the vacuum is not symmetric.
          Most of the mass of the matter around you is a side effect of a symmetry the vacuum chose to
          break.
        </p>
      </div>
    </div>
  </section>

  <!-- ───────────────────────── coda ───────────────────────── -->
  <section class="space-y-4">
    <h2 class="h2">Coda: what you should take away</h2>
    <div class="grid sm:grid-cols-2 gap-3">
      <div class="rounded-lg border border-surface-700 bg-surface-800/40 p-4 space-y-2">
        <div class="font-semibold text-primary-300">Things QCD is spectacularly good at</div>
        <ul class="text-sm text-surface-400 space-y-1 list-disc list-inside">
          <li>Predicting the proton mass from first principles (lattice QCD, few-per-mille)</li>
          <li>Running of α<sub>s</sub>, matched across four decades of energy</li>
          <li>Jet production rates at the LHC</li>
          <li>Hadron spectroscopy, including exotics found since 2003</li>
          <li>Properties of the quark–gluon plasma, whose shear viscosity comes within a factor of a few of the conjectured KSS bound η/s = 1/4π</li>
        </ul>
      </div>
      <div class="rounded-lg border border-surface-700 bg-surface-800/40 p-4 space-y-2">
        <div class="font-semibold text-error-300">Things QCD cannot do</div>
        <ul class="text-sm text-surface-400 space-y-1 list-disc list-inside">
          <li>Prove it has a mass gap, or that it exists at all</li>
          <li>Prove confinement</li>
          <li>Say what number a late-order perturbation series sums to</li>
          <li>Explain why θ ≈ 0</li>
          <li>Compute dense matter (neutron star interiors) from first principles</li>
          <li>Be drawn, in any way that is not a lie</li>
        </ul>
      </div>
    </div>
    <div class="rounded-lg border border-primary-800/50 bg-primary-950/20 p-5 space-y-3">
      <p class="text-surface-300">
        The honest summary is that the strong force is a theory of a field that does not know how to be
        weak, in a vacuum that does not know how to be empty, whose fundamental excitations cannot
        exist alone, computed with a series that does not converge, and whose mathematical existence is
        an open problem worth a million dollars. It is also the most accurately tested theory in
        history, and it is holding your atoms together right now, without any need for your approval.
      </p>
      <p class="text-sm text-surface-400">
        Every drawing above was a lie. The lies were at 4% to 78% fidelity, and the ones near 4% were
        the most accurate. That is what an honest picture of nature at this scale looks like: not a
        diagram, but a set of invariants that survive every change of coordinates.
      </p>
    </div>

    <div class="text-xs text-surface-500 space-y-1">
      <div>
        Numbers used: α<sub>s</sub>(M<sub>Z</sub>) = 0.1179 (PDG); σ ≈ 0.9–1 GeV/fm; m<sub>p</sub> =
        938.272 MeV; m<sub>u</sub> ≈ 2.16 MeV, m<sub>d</sub> ≈ 4.67 MeV; Λ<sub>QCD</sub> ≈ 200 MeV;
        ⟨q̄q⟩ ≈ −(250 MeV)³; ⟨α<sub>s</sub>G²⟩ ≈ 0.012 GeV⁴; |θ| &lt; 10<sup>−10</sup>. Running
        coupling integrated in-browser with the two-loop β-function and flavour thresholds. Everything
        else is a cartoon, as advertised.
      </div>
    </div>
  </section>
</div>
