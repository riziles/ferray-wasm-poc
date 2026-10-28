# AGENTS.md

Working notes for AI agents on this repo. Read this before opening a PR.

## Merging: publish, then wait

The maintainer reviews and tests between publishing a PR and merging it. That gap
is the point — it is not a formality to shortcut.

- **Open the PR, report it (number, CI status, what changed), and stop.** Do not
  run `gh pr merge` in the same turn as `gh pr create`.
- Wait for an explicit go-ahead naming that PR (e.g. "merge #52"). A general
  "please fix both" or "yes" to something else is not a merge instruction.
- If a merge instruction is ambiguous — it could mean several PRs, or one that is
  already merged — **ask which one**. Do not infer, and do not widen the scope.
- Never merge a PR you did not open (contributor/fork PRs) without an explicit
  instruction naming it.
- Never push directly to `main`, and never force-push a shared branch.

Merging is otherwise fine — squash merges, branch deleted. The rule is about
timing and scope, not about refusing to merge.

## Project shape

- `svelte-app/` — SvelteKit 5 + Svelte 5 runes, Skeleton v4 (cerberus theme),
  Tailwind 4, pnpm. Root is a Rust crate compiled to WASM.
- CI (`.github/actions/build-app`) builds the WASM, then
  `pnpm install --frozen-lockfile && pnpm run build`, then asserts that specific
  routes and OG artifacts exist. Commit `pnpm-lock.yaml` with any dependency change.
- `svelte-app/src/lib/wasm/ferray_wasm_poc.{js,wasm}` are **generated and
  gitignored** (by `scripts/build-wasm.sh` or CI). A fresh checkout cannot build
  until they exist.

## Things that will waste your time if you don't know them

- **`pnpm run build` rewrites `static/og-*.png`** for every card, so files you
  never touched show up dirty. `git checkout -- svelte-app/static/og-*.png`
  before committing (keep the new ones you just added).
- **`pnpm run check` reports ~134 errors that are not yours** — generated WASM
  glue plus older demo components. Establish the baseline first; only worry about
  new ones in files you touched.
- **Adding an OG card** means four edits: the `static/og-<name>.svg`, a route
  entry in `svelte-app/og-routes.json`, the `svg2png` argument list in
  `svelte-app/package.json`'s `build` script, and an assertion in
  `.github/actions/build-app/action.yml`. Nested routes work as keys
  (`notes/catoid` → `dist/notes/catoid.html`).
- **A page that must render without WASM** has to be added to
  `wasmOptionalRoutes` in `svelte-app/src/routes/+layout.svelte`, otherwise the
  layout blocks it behind the WASM load.
- **SvelteKit is `ssr = false`** (`routes/+layout.ts`); pages are prerendered as
  shells, so verify behaviour in a browser, not by grepping the built HTML.
- **Skeleton v4 has no unsuffixed `preset-filled-*`** — only
  `preset-filled-<role>-<shade>`. Preset utilities are single-class rules, so two
  of them on one element means "later in the bundle wins"; make them mutually
  exclusive instead.

## Media and infrastructure

- Long-form pages load screenshots from **R2**, not from git: bucket
  `ferray-wasm-poc-media`, public via the custom domain `https://media.riziles.com`
  (`cf r2 buckets ...`; `cf` is authenticated against the `riziles.com` zone).
- A zone **Cache Rule** caches `media.riziles.com` (edge 1 year, browser 4 hours).
  Note `HEAD` bypasses that cache and still reports `DYNAMIC` — test with `GET`.
- Objects carry no `Cache-Control` of their own (`cf`'s upload has no flag for it);
  purge with `cf cache purge --files https://media.riziles.com/<key>` when
  replacing one.
