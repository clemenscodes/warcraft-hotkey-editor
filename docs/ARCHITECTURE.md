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

### Internal layering inside `hotkey-editor`

The frontend is split into three layer directories that mirror the `ddd`
layer markers each type carries. This is organizational, not a second wall:
a directory split does not enforce the dependency rule — the `ddd`
`Layered` / `ApplicationService` / `Adapter` markers plus the crate's import
graph are the guardrail. Only the crate boundary in §3 is truly enforced.

- `services/` — application + presentation. Application services
  (`CustomKeysService`, `GridLayoutService`, `UndoHistory`) implement
  `ddd::Service<Aggregate>` and are the only sanctioned way for the renderer
  to mutate state; every command funnels through `commit` (write-through:
  mutate the snapshot → persist via the repository → replace the live
  signal). UI-only signal bags (dialog-open flags, selection, drag,
  navigation) also live here as presentation state and carry no repository.
- `repository/` — infrastructure. One `ddd::Repository<Aggregate> + Adapter`
  per persisted aggregate (`CustomKeysRepository`, `GridLayoutRepository`,
  `EditorHistoryRepository`); loads and saves whole aggregates as canonical
  text.
- `persistence/` — infrastructure. Typed wrappers over the individual
  localStorage keys plus the `LocalStorage` primitive. Compression (the undo
  blob's deflate/base64) lives here, never in the domain crate (R8).

A service can only get the full pattern if it is backed by a domain
aggregate (`ddd::AggregateRoot`, which lives in `warcraft-keybinds`); that
trait bound is what stops a repository being bolted onto non-domain UI
state.

## 4. Hard rules (mechanically checkable)

These are the rules every change must obey. Treat each as a compile-time
constraint, even when the compiler can't enforce it.

### How the wall is enforced today

All internal mutation helpers on `CustomKeys` that the renderer must not
call are `pub(crate)`:

- `binding_or_default_mut` — `pub(crate)`
- `command_or_default_mut` — `pub(crate)`
- `system_mut` — `pub(crate)`

Calling any of these from `hotkey-editor` is a **compile error**. The only
public API the renderer may use to mutate state is named facade commands:
`set_hotkey`, `move_slot`, `assign_position`, `apply_grid_to_all_bindings`,
`set_system_hotkey`, `swap_system_bindings`, `normalize`, `serialize`.

**When adding a new mutation to `CustomKeys`:**
- If the renderer needs it, add a named command method (`pub fn do_thing`)
  that encapsulates the entire operation and returns a normalized result.
- Do NOT add a `pub fn something_mut` that exposes a raw `&mut` reference.
  That bypasses normalization and re-opens the R4 violation.
- Internal helpers that are only needed by other `CustomKeys` methods stay
  `pub(crate)` or private.

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
`warcraft-keybinds` builds and tests as a plain native Rust crate. No
`wasm-bindgen`, no `web-sys`, no `dioxus`, no `gloo`. Its game data comes
from `warcraft-api` and `warcraft-database`, consumed as a git-pinned
external dependency (`github.com/clemenscodes/warcraft-data`, tag `v0.1.0`)
and regenerated in that repo, not here; otherwise `serde` is the only
dependency it may add — nothing else.

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

One deliberate reinterpretation: **editor revision history is domain
state.** The undo/redo timeline is modelled as an `EditorHistory` aggregate
in `warcraft-keybinds` (persisted under its own localStorage key via
`EditorHistoryRepository`), not as loose UI signals — so it can use the same
`Service`/`Repository`/`commit` pattern as `CustomKeys`. Everything else in
the R10 list stays UI state; this is the only place a "history of edits" is
treated as a domain concept rather than presentation scratch.

## 5. History: the original violations are resolved

An earlier revision of this document catalogued a set of rule violations — an
in-memory `loaded_keys` signal that trailed localStorage (R1), renderer-time
cascade shims (R3), a `write_container_resolved` that refused to bake resolved
positions (R2), an `explicit_export.rs` re-derivation pipeline (R2/R5), UI
`binding.set_*` mutations (R4), and a 1779-line `lib.rs` — together with a
seven-phase plan to fix them.

**That plan is complete.** localStorage is the source of truth, cascade is
baked at write time, the renderer mutates only through the `CustomKeys` facade,
and `warcraft-keybinds` has a semantic module tree with the three aggregate
roots (`CustomKeys`, `GridLayout`, `EditorHistory`) already marked via `ddd`.
The hard rules R1–R10 in §4 stand unchanged and are the live contract; this
section is kept only as the historical record of what motivated them.

## 6. Current work: the domain-crate DDD + quality refactor

The active effort is bringing `warcraft-keybinds` up to the architectural and
code-quality bar `hotkey-editor` reached — full CQRS adoption of the `ddd`
vocabulary (realized cross-crate), full `RUST_STYLE.md` compliance, and
hierarchical file decomposition of the remaining monoliths.

- **Structural + DDD conventions for the domain crate:** `docs/DOMAIN.md`.
- **The design spec:**
  `docs/superpowers/specs/2026-07-06-warcraft-keybinds-ddd-refactor-design.md`
  (§9 holds the phase list).
- **Per-phase implementation plans:** `docs/superpowers/plans/`, one file per
  phase, authored just-in-time against the real post-split code.

Guardrails unchanged from §4: DomainEvents are transient (no event store),
localStorage-materialized text stays the source of truth (R1/R2/R5), and no
domain logic crosses the wall — only application-layer `Command`/`Query` shells
live in `hotkey-editor`. Each phase ends with `moon run :ci` green and the app
working in the browser.

## 7. Build, test, and release

### Moon tasks — quick reference

```bash
# Local (Nix dev shell)
moon run :dev                           # tailwind/build → dx serve (localhost:8123)
moon run :bundle                        # tailwind/build → dx build --release
moon run hotkey-editor:playwright/test  # tailwind/build → e2e (starts own server)
moon run :check                         # fast cargo check (wasm) — compile only, NOT the gate
moon run :ci                            # fmt + clippy + tests + wasm build + e2e

# Docker
moon run :docker/up                     # dev server in container (localhost:8123)
moon run :docker/down                   # stop docker compose
moon run :docker/e2e                    # e2e tests in container
moon run hotkey-editor:docker/serve     # build prod image + serve on localhost:8123
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

`tailwind/build` runs `tailwindcss -i tailwind.css -o assets/tailwind.css
--minify` from `crates/hotkey-editor/`. Moon caches it on inputs
(`tailwind.css`, `styles/**`, `src/**/*.rs`) so it only re-runs when
those files change.

### End-to-end tests

Tests live in `crates/hotkey-editor/e2e/tests/` and run with
[Playwright](https://playwright.dev) against a live dev server on port 8123.

`moon run hotkey-editor:playwright/test` runs `tailwind/build` first, then
hands off to `e2e/run.mjs`, which owns the server lifecycle:

1. Check whether port 8123 is already open. If so, reuse it.
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
versions as the Nix dev shell: Rust 1.96.1, dioxus-cli 0.7.9,
wasm-bindgen-cli 0.2.126, Node.js 24.15.0, pnpm 11.0.9, moon 2.0.3,
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
the server is reachable from the host on port 8123.

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
moon run hotkey-editor:docker/serve   # build prod image → serve on localhost:8123
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
