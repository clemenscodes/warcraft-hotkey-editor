# Performance & bundle

This file records the performance-relevant decisions for the shipped WASM SPA:
what is tuned, where the real levers are given the deployment, and which bigger
wins are deliberately deferred (with the blocker that defers them). It is the
baseline the survey in `docs/ARCHITECTURE.md`/`docs/COMPONENTS.md` did not have.

The product is a **client-only WASM SPA deployed to GitHub Pages** (see
`.github/workflows/deploy.yml`, `actions/deploy-pages`). That deployment target
decides which optimizations matter — see "What GitHub Pages already does".

---

## What is tuned today

### 1. Release profile (`Cargo.toml`, `[profile.release]`)

`codegen-units = 1`, `lto = "fat"`, `opt-level = "z"`, `panic = "abort"`,
`strip = "symbols"`. The app is DOM-bound — per-frame Rust work is light — so the
binary is tuned for **size**, which is the lever that moves Lighthouse. `opt-level
= "z"` is the one knob to revisit if interaction latency ever regresses: bump it to
`"s"`. `panic = "abort"` drops the unwinding machinery and its JS glue; cargo forces
unwind back on for `cargo test`, so the native `warcraft-keybinds` tests are
unaffected.

### 2. wasm-opt (`crates/hotkey-editor/Dioxus.toml`, `[web.wasm_opt]`)

`level = "z"`, `debug = false`. `dx` runs binaryen's wasm-opt on release builds
already (binaryen is installed in the Dockerfile); pinning the level makes the size
target explicit rather than relying on the default.

### 3. Font: `swap` + preload

- `tailwind.input.css` sets `font-display: swap` on Friz Quadrata (was `block`,
  which Lighthouse flags — it hides text until the face loads).
- `document_head/mod.rs` preloads the `.ttf` (`rel="preload" as="font"
  crossorigin`), so the swap window is tiny. `crossorigin` is required even
  same-origin: font fetches are CORS-mode, and without it the preload double-fetches.

### 4. Reactivity firebreak (`use_memo` on hot query reads)

`CustomKeysService`/`GridLayoutService` each wrap a **single monolithic signal**
(the whole parsed document). Every query (`slot_binding`, `unit_collisions`, …)
reads that whole signal, so a bare call re-renders the reader on *any* edit
anywhere. The fix is to wrap the query read in a `use_memo` keyed on the specific
id: the memo still recomputes, but only notifies the component when its own result
changes (`PartialEq`), confining the re-render.

- **Reference:** `grid_editors/grid_editor/mod.rs` (memoizes `Vec<RenderedTile>`
  in its own reactive scope).
- **Applied:** `system_hotkeys_dialog/.../slot_button/hooks.rs` (memoizes
  `SlotBindingView`) — every slot in the dialog no longer re-renders on every
  keystroke elsewhere.
- **Still unmemoized** (same pattern applies when they show up hot): the inventory
  slot buttons, the unit list, and unit-detail readers. Apply the firebreak the
  same way — the query view types already derive `PartialEq`.

---

## What GitHub Pages already does (so we don't)

GitHub Pages serves the static bundle through its own CDN. It **compresses
responses and sets its own `Cache-Control` headers, and ignores any custom header
config**. So:

- **Compression and cache headers are not our lever.** The nginx config in the
  root `Dockerfile` (`gzip`, `Cache-Control`) would only affect the local
  `docker/serve` path, which is **not** the production deployment. It is
  intentionally left untuned — changing it does nothing for the deployed
  Lighthouse score.
- The lever we *do* control is **raw bundle size** (profile + wasm-opt above);
  smaller uncompressed is smaller compressed too.

---

## Deferred — bigger wins, and the blocker that defers each

These were evaluated and are genuinely applicable, but each has a concrete blocker
that makes flipping it on blind a regression risk. They are follow-ups, not
oversights.

- **Static prerender / SSG** (`dx` SSG). The largest FCP/LCP win: the page is blank
  until wasm boots and hydrates; prerendering the shell to static HTML paints real
  content first. Deferred because it changes the launch/build mode (fullstack/server
  feature + hydration) and needs the routes to be prerender-clean — a focused change
  with its own test surface, not a config flip.
- **Route-level bundle splitting** (`wasm-split`, Dioxus 0.7). All three pages ship
  in one wasm today; splitting would load `collisions`/`resolve` on navigation.
  Deferred because `wasm-split` needs a nightly toolchain and the repo is pinned to
  **stable Rust 1.96.1** (`rust-toolchain.toml`, Dockerfile); enabling it is a
  toolchain change that would touch CI.
- **`dioxus-stores` migration.** Replacing the monolithic `Signal<Option<CustomKeys>>`
  in `CustomKeysService` with a `Store` would make the per-slice firebreak automatic
  instead of hand-placed. Deferred because it is an architectural change to the DDD
  service layer that needs its own tests; the `use_memo` firebreak above captures the
  bulk of the responsiveness win now, per the rule of three.

---

## Verify

Same gates as any change (`docs/COMPONENTS.md`):

```
nix develop -c cargo clippy -p hotkey-editor -p gallery --target wasm32-unknown-unknown
nix develop -c cargo test -p warcraft-keybinds
nix develop -c cargo fmt --check
nix develop -c moon run hotkey-editor:bundle   # validates the release profile + Dioxus.toml
```
