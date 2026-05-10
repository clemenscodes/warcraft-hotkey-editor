# Architecture

This document is the **contract** for this project. Every contributor — human
or agent — must follow it. Any change that violates these rules is a bug,
even if it compiles and "looks right". When in doubt, the rules win.

If you find yourself wanting to bend a rule, stop and ask first. The bugs
that motivated this document all came from quiet rule-bending.

---

## 1. The product, in one sentence

A pure-frontend editor for **a single `CustomKeys.txt` file**. There is no
server, no database, no cloud sync. The user opens the page, edits a file,
downloads the file. That's it.

## 2. The Single Source of Truth

The canonical state of the workbench is **one string** in `localStorage`:

```
key:   warcraft-hotkey-editor.custom-keys
value: the full text of CustomKeys.txt, fully normalized
```

That string IS the state. There is no parallel in-memory state that "is the
real one" until it gets persisted later. There is no derived cache that can
disagree with it. There is no "uncommitted edit". Every mutation produces a
new fully-normalized text and writes it to that key **immediately**, before
returning to the event loop.

Other localStorage keys (UI-only state like the chosen grid layout, dialog
open state, last-selected unit, etc.) MAY exist, but they are strictly UI
preferences. They never duplicate or shadow data that lives in the
CustomKeys.txt string. If a fact can be expressed as a CustomKeys.txt field,
it goes in CustomKeys.txt — not in a sidecar key.

## 3. The two crates and the wall between them

```
┌──────────────────────────────────────────────┐
│   hotkey-editor  (wasm, Dioxus, browser)     │   ← pure renderer + dispatcher
│                                              │
│   - reads localStorage                       │
│   - asks domain crate to parse it            │
│   - displays bindings as-is                  │
│   - on user gesture: calls a domain command, │
│     writes returned text to localStorage,    │
│     re-renders                               │
└──────────────────────────────────────────────┘
                       │
                  no domain logic
                  crosses this line
                       │
┌──────────────────────────────────────────────┐
│   warcraft-keybinds  (pure Rust, native)     │   ← all domain logic
│   + warcraft-api, warcraft-database          │
│                                              │
│   - parse / serialize CustomKeys.txt         │
│   - normalize: cascade collisions,           │
│     dedupe hotkeys, resolve positions,       │
│     materialize defaults                     │
│   - validate: collisions, duplicates         │
│   - mutate: every command returns a fully-   │
│     normalized file                          │
│   - 100% covered by unit tests               │
│   - zero wasm / dioxus / web-sys deps        │
└──────────────────────────────────────────────┘
```

### What lives in `warcraft-keybinds` (the domain crate)

EVERYTHING that is not pure presentation:

- `CustomKeys.txt` parser and serializer.
- The full position-cascade algorithm.
- Duplicate-hotkey detection.
- Collision detection between bindings.
- Apply-grid-layout (assign hotkey letters from a layout to all positions).
- Template overlay logic.
- Default-position materialization from the game database.
- All knowledge of unit command cards, build menus, research menus,
  uprooted menus, system hotkey sections.
- All "what hotkey does this slot have" / "what position is this binding
  at" / "is this a passive ability" queries.

These all run **at write time**, not at read time. The output of any
mutation is a CustomKeys.txt string in which every binding has its final,
displayable position and hotkey already baked in.

### What lives in `hotkey-editor` (the frontend)

ONLY presentation:

- Dioxus components and RSX.
- Reading localStorage, calling the domain crate's parse function, and
  displaying the result.
- Mapping user events (clicks, drags, keystrokes) to domain command calls,
  then persisting the returned text to localStorage.
- CSS, icons, tooltips, focus management, race tabs, dialog open/close.
- TypeScript glue for browser APIs the framework doesn't cover.

That is the entire allowed scope. If something else looks domain-shaped, it
isn't supposed to be here.

## 4. Hard rules (mechanically checkable)

These are the rules every change must obey. Treat each as a compile-time
constraint, even when the compiler can't enforce it.

**R1. localStorage is the source of truth.**
There is no `Signal<CustomKeysFile>` that holds "the real" state in memory
while localStorage trails behind. Every mutation writes to localStorage
synchronously, in the same tick. The signal pattern, if used at all, is a
read cache that is rebuilt from localStorage after every write.

**R2. Stored state is fully normalized.**
The text in localStorage is post-cascade, post-collision-resolution, post-
materialization. Reading it yields concrete `Buttonpos=` values for every
binding the editor cares about. The renderer never asks "where would this
go after cascading?" — that question has no meaning at render time.

**R3. The renderer never computes domain decisions.**
No cascade, no collision-resolve, no "is this position occupied", no
"materialize the default", no "what hotkey would the grid layout assign
here". If you need any of those, you call a domain function — and that
function's contract is to return something already-resolved, never to be
called repeatedly during render.

**R4. The renderer never mutates `CustomKeysFile` directly.**
All mutations go through named domain commands:
`apply_hotkey`, `move_or_swap`, `apply_grid_layout`, `apply_template`,
`import_uploaded_file`, etc. Each command takes the current state (text or
parsed file), produces a new fully-normalized state, and is the only thing
that touches the binding fields. UI code never calls
`binding.set_hotkey(...)` itself.

**R5. Export is a copy of localStorage.**
"Export" and "preview" must be implemented as `localStorage.getItem(KEY)`.
No re-serialize, no re-overlay, no re-normalize at export time. If the
preview is wrong, the bug is upstream — fix the mutation that produced bad
state, not the export.

**R6. Boot is deterministic.**
On startup:

  1. If `localStorage[KEY]` exists, take it.
  2. Otherwise, take the bundled default `CustomKeys.txt`.
  3. Pass it through the domain normalize function.
  4. Write the normalized result to `localStorage[KEY]`.
  5. Render from `localStorage[KEY]`.

The bundled default is a static asset. The normalize step is idempotent —
running it again on already-normalized text returns the same text.

**R7. Imports replace, then normalize.**
File upload and template-apply both work the same way: hand the uploaded
text to the domain crate, get back a normalized text, write it to
localStorage. No "overlay onto the in-memory copy" path exists in the
frontend.

**R8. The domain crate has zero browser dependencies.**
`warcraft-keybinds` (and `warcraft-api`, `warcraft-database`) build and
test as plain native Rust crates. No `wasm-bindgen`, no `web-sys`, no
`dioxus`, no `gloo`. The crate's only external dependency may be `serde`
if it becomes useful — nothing else.

**R9. The domain crate is fully tested.**
Every cascade rule, every collision case, every duplicate-detection
behavior, every grid-layout application has unit tests. New behavior comes
with new tests in the same change. A bug fix starts with a failing test
that reproduces the bug.

**R10. UI state is UI state; domain state is domain state.**
"Which dialog is open", "which unit is selected", "is the user currently
dragging" — these are UI signals, never written to the CustomKeys.txt
string. "What hotkey does ability X have", "where does it sit in the
grid" — these live in CustomKeys.txt and only there.

## 5. Where today's code violates these rules

These are the active violators that motivated this document. The refactor
plan in §6 names the order in which to fix them.

- **`hotkey-editor/src/app.rs`** holds `loaded_keys: Signal<Option<CustomKeysFile>>`
  as an in-memory copy that's mutated directly by UI code. localStorage is
  written from a `use_effect` that trails the signal. **Violates R1.**

- **`hotkey-editor/src/domain/positions.rs`** is a thin shim over
  `warcraft_keybinds::cascade::*` and is called from render code in
  `command_grid.rs`, `inspector_detail.rs`, etc. The cascade runs every
  render. **Violates R3.**

- **`warcraft_keybinds::cascade::write_container_resolved`** explicitly
  refuses to write back resolved ability `Buttonpos` values:

  > "Ability button positions are NOT written back here."

  This is the smoking gun. The on-disk file does not match what the
  renderer shows. Anyone reading the file in another tool sees stale
  positions. **Violates R2.**

- **`hotkey-editor/src/customkeys/explicit_export.rs`** rebuilds an export
  file by re-running overlay + materialize + normalize on every save.
  This is also how localStorage is written, which means localStorage is
  the *output* of an export pipeline rather than the canonical state.
  **Violates R2 and R5.**

- **UI components mutate bindings directly via `binding.set_*`** through
  `Positions::assign` and `Positions::move_or_swap`. **Violates R4.**

- **`warcraft-keybinds/src/lib.rs`** carries the parser, the model, the
  serializer, and a giant raw-section-preservation cache in 1779 lines.
  The "raw section preservation" path is dead weight given R2: if state
  is always normalized, there is no untouched raw text to preserve.
  Round-trip-byte-identical isn't a goal anymore.

- **`hotkey-editor/src/customkeys/baseline.rs`** + the `overlay` module +
  `materialize_default_positions` form a multi-step "build the export"
  pipeline that exists only because the in-memory state isn't normalized.
  Once R2 holds, all of that collapses into a single
  `CustomKeys::default()` constructor that runs once at boot.

## 6. Refactor plan

Phased so each phase ships a working app. Don't merge a phase that breaks
the build. Don't leave a phase half-done across sessions.

**Phase 1 — Establish the contract (this commit).**
- Write this document.
- Write `/CLAUDE.md` with the agent-facing rules.
- No production code change yet beyond a stub.

**Phase 2 — A canonical facade in `warcraft-keybinds`.**
Add a public `CustomKeys` type that is the only thing the frontend may
touch:

```rust
pub struct CustomKeys { /* opaque */ }

impl CustomKeys {
    pub fn from_text(text: &str) -> Self;          // parses + normalizes
    pub fn from_default() -> Self;                 // bundled baseline + normalize
    pub fn to_text(&self) -> &str;                 // canonical, already-normalized
    pub fn parsed(&self) -> &CustomKeysFile;       // read-only view for the renderer

    // Commands. Each one re-normalizes before returning.
    pub fn assign_position(&mut self, ...);
    pub fn move_or_swap(&mut self, ...);
    pub fn apply_grid_layout(&mut self, layout: GridLayout) -> usize;
    pub fn apply_template(&mut self, template_text: &str);
    pub fn replace_with_uploaded(&mut self, uploaded_text: &str);
    pub fn clear_override(&mut self, slot: GridSlotId);
    // ...
}
```

The internals can keep delegating to the existing `cascade`, `overlay`,
`export` modules for now — the goal in this phase is to give the frontend
exactly one API surface to depend on.

**Phase 3 — Make localStorage the source of truth.**
- `LocalStorageCache::save(&CustomKeys)` writes `CustomKeys::to_text()` directly.
- Remove `ExplicitExport`. Export and preview read `localStorage` and show
  the string verbatim.
- Boot path: `CustomKeys::from_default()` if no entry, else
  `CustomKeys::from_text(stored)`. Either way, write back so the entry is
  always present and normalized.

**Phase 4 — Bake cascade into stored state.**
- Change `write_container_resolved` to write resolved ability `Buttonpos`
  back to the file (the comment that forbids this becomes obsolete once
  the container model is unit-scoped).
- Add a regression test: after `from_text`, every binding that was visible
  in any container has a concrete `Buttonpos=` matching what the renderer
  would show.
- Delete the renderer-time cascade calls in `positions.rs` and replace
  them with simple lookups: `file.binding(id).button_position()`.

**Phase 5 — Strip domain logic out of the renderer.**
- Delete `hotkey-editor/src/domain/positions.rs` (or shrink it to a
  display-only adapter that contains zero logic).
- All `command_grid.rs` / `inspector_detail.rs` reads become direct field
  accesses on `CustomKeys::parsed()`.
- All UI mutations route through `CustomKeys` commands. No
  `binding.set_*` outside `warcraft-keybinds`.

**Phase 6 — Clean up the domain crate internals.**
- Drop the raw-section-preservation cache (R2 makes it dead weight).
- Split `lib.rs` into focused modules (`parser.rs`, `model.rs`,
  `serializer.rs`).
- Replace the per-field setter sprawl with idiomatic builder / patch
  patterns where it improves clarity.
- Audit and trim `building`, `catalog`, `unit_slots`, `lookup` for things
  that should live in `warcraft-database` instead.

**Phase 7 — Tests.**
- Property tests: `from_text(to_text(x)) == x` for normalized inputs.
- Regression tests for every shipped fix in `cascade_tests`.
- A test that asserts the renderer never imports anything from
  `warcraft_keybinds::cascade` (use a deny-list lint or grep guard in CI).

Each phase ends with `moon run :ci` green and the app working in the
browser.

## 7. Build, test, and release

### Moon tasks — quick reference

```bash
# Local (Nix dev shell)
moon run :dev                           # tailwind/build → dx serve (localhost:8080)
moon run :bundle                        # tailwind/build → dx build --release
moon run hotkey-editor:playwright/test  # tailwind/build → e2e (starts own server)
moon run :ci                            # fmt + clippy + tests + wasm build + e2e

# Docker
moon run :docker/up                     # dev server in container (localhost:8080)
moon run :docker/down                   # stop docker compose
moon run :docker/e2e                    # e2e tests in container
moon run hotkey-editor:docker/serve     # build prod image + serve on localhost:8080
moon run hotkey-editor:docker/down      # stop the prod container
```

### Tailwind

`assets/tailwind.css` is a build artifact — it is not committed to git. It
must be compiled before `dx serve` or `dx build` runs. Every task that
starts the app lists `tailwind/build` as a dependency:

```
tailwind/build  →  dx/serve   (dev)
tailwind/build  →  dx/build   (production bundle)
tailwind/build  →  playwright/test  (e2e, because run.mjs starts dx serve directly)
```

`tailwind/build` runs `tailwindcss -i tailwind.input.css -o assets/tailwind.css
--minify` from `crates/hotkey-editor/`. Moon caches it on inputs
(`tailwind.input.css`, `styles/**`, `src/**/*.rs`) so it only re-runs when
those files change.

### End-to-end tests

Tests live in `crates/hotkey-editor/e2e/tests/` and run with
[Playwright](https://playwright.dev) against a live dev server on port 8080.

`moon run hotkey-editor:playwright/test` runs `tailwind/build` first, then
hands off to `e2e/run.mjs`, which owns the server lifecycle:

1. Check whether port 8080 is already open. If so, reuse it.
2. Otherwise spawn `dx serve`, stream stdout/stderr, and wait until
   `"launching app"` appears (compilation done, server live).
3. Run `playwright test`.
4. Kill the server on exit.

The tests are a CI gate — `moon run :ci` will not pass without them.

### Nix (reproducible release)

```bash
nix build .#warcraft-hotkey-editor
```

`Dioxus.toml` sets `base_path = "warcraft-hotkey-editor"` for GitHub Pages.
All asset URLs the bundler generates include that prefix.

### Docker

All Docker paths use `ubuntu:24.04` as the base and pin the same tool
versions as the Nix dev shell: Rust 1.95.0, dioxus-cli 0.7.9,
wasm-bindgen-cli 0.2.121, Node.js 24.15.0, pnpm 11.0.9, moon 2.0.3,
tailwindcss 4.3.0. `git` is installed in the image because moon requires
it to detect the workspace root and changed files.

#### Dev server (`Dockerfile` — root)

```bash
moon run :docker/up     # same as: docker compose up hotkey-editor
```

`docker-compose.yml` defines the `hotkey-editor` service. It mounts the
entire repo at `/app` and uses named volumes for `target/`, `node_modules/`,
and the moon and Cargo caches so they survive container restarts. The
service runs:

```
moon run hotkey-editor:dev/docker
```

That task first runs `tailwind/build`, then starts
`dx serve --platform web --addr 0.0.0.0` from `crates/hotkey-editor/` so
the server is reachable from the host on port 8080.

#### E2e tests (`Dockerfile` — root)

```bash
moon run :docker/e2e    # same as: docker compose --profile e2e run --rm e2e
```

The `e2e` service uses the same image (which also includes the Playwright
Chromium binary and its system dependencies). It runs:

```
moon run hotkey-editor:playwright/test
```

That task builds tailwind first, then `e2e/run.mjs` starts `dx serve`
inside the container and runs the Playwright suite against it.

#### Production image (`crates/hotkey-editor/Dockerfile`)

```bash
moon run hotkey-editor:docker/serve   # build prod image → serve on localhost:8080
moon run hotkey-editor:docker/down    # stop the prod container
```

Multi-stage build:

| Stage | What it does |
|---|---|
| `base` | Installs all tooling (Rust, Node, pnpm, moon, tailwindcss, dx, wasm-bindgen, git) |
| `builder` | Copies source, runs `moon run hotkey-editor:bundle` (tailwind → dx build --release) |
| `production` | `nginx:alpine` image; copies `target/dx/hotkey-editor/release/web/public` and `nginx.conf` |

The nginx config (`crates/hotkey-editor/nginx.conf`) redirects `/` to
`/warcraft-hotkey-editor/` and serves the SPA with `try_files` fallback to
`index.html` for client-side routing. The build output lands in
`target/dx/hotkey-editor/release/web/public` — not `dist/`.
