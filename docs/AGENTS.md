# Agent workflow & hard-won gotchas

Operational rules and lessons that cost real time to learn. The *design* rules
live in the three spec docs below; this file is the *how you work* companion —
commands, the dev/e2e loop, and the specific traps that have burned agents.

Read this alongside:

- `docs/ARCHITECTURE.md` — the wall (domain vs renderer, localStorage source of truth).
- `docs/RUST_STYLE.md` — how Rust is written.
- `docs/COMPONENTS.md` — how renderer components are named, laid out, and written
  (render-tree == directory-tree, compose-don't-share-styles, no `states!`, one
  class per component, per-state components, `Element` is never a value, …). All
  design-law questions are answered there — do not re-derive them here.

---

## Commands and CI

- **Only two commands exist:** `moon run :ci` (the one and only verification gate —
  fmt, clippy, tests, wasm build, Playwright e2e) and `moon run :dev` (the one and
  only dev server). Never a narrower `moon`/`cargo`/`dx`/`playwright` target. If
  `moon run :ci` fails, fix the code and run it again — never route around it.
  (`cargo fmt` directly is fine to *format*; the gate is still `moon run :ci`.)

- **NEVER run a second `moon run :ci` while one is already running.** There is **no
  CPU-load flakiness** in this project — the tests are deterministic. A mass failure
  spread that is **all-firefox / zero-chromium** across many unrelated specs is the
  unmistakable signature of **two concurrent gates** (yours colliding with the
  user's) fighting over CPU, the dev port (8123), and the e2e browsers. Before
  running the gate: `pgrep -af "moon run|playwright|dx"` and check port 8123 — or
  just ask whether CI is already running. Chromium-green + the user's own CI is the
  real signal.

- **Verify the gate's OWN exit code, not a pipe tail.** `moon … | tail` reports
  tail's exit (always 0) → false green. Read the process's own `$?`/PIPESTATUS, or
  read the output file for the pass/fail summary. Don't trust a stale exit code on a
  long job either — a 7-minute e2e that "returned 0" long ago may be stuck, not done.

## Dev server and the e2e loop

- **The dev URL is `http://localhost:8123/warcraft-hotkey-editor/`** (trailing slash
  included) — NOT `/`. The app is served under the `warcraft-hotkey-editor` base
  path; bare `/` and `/index.html` return **404** — that is the wrong URL, not a
  broken server. Routes hang off the base (`…/`, `…/collisions`, `…/resolve`). The
  gallery (port 8200) serves at `/`.

- **Do not wait on the "Your app is being rebuilt" overlay.** `dx` regularly gets
  stuck showing it long after the build is done. The fix is a **page refresh**
  (re-navigate to the base URL), not waiting. Only investigate a real compiler error
  printed in `dev.log`.

- **Never squat the dev ports** (8123 dev / 8200 gallery) with a stray `dx serve`.

- **Do not edit repo source while the e2e suite is running.** A save mid-run makes
  `dx serve` flash its own `__dx-toast "Hot-patch success!"` dev toast, which
  intercepts pointer events and fails clustered header-click tests. Not a code bug —
  edit on a quiet tree.

- **Visual verification IS available** via the Playwright MCP. Never stop and claim a
  change is "visually unverifiable" — screenshot before/after and check.

## E2E selectors are coupled — renames scream, or should

- The e2e specs select on component **identity classes** and (historically) data
  attributes. Renaming a class an e2e selects on requires updating the spec **in the
  same commit** and running the gate. Treat identity classes referenced by
  `services/` or e2e as a coupled contract.

## Hard-won technical gotchas

- **Click-vs-drag on one element: defer pointer capture.** A component that both
  clicks-to-edit and drags-to-swap must NOT `setPointerCapture` or mutate drag
  signals in `on_pointerdown`. Record a *pending* drag; promote it (capture + set the
  drag signals + mount the follower) only on the first `pointermove` past the
  threshold — mirror `grid_editor`'s mechanics. Capturing/mutating on pointerdown
  drops the `click` in Chromium (the captured host ≠ the mousedown target, or the
  mutation unmounts the mousedown target) — silent in Firefox. Verify with a REAL
  browser and stepped `page.mouse.move(…, {steps})`, not `dragTo` or JS-synthetic
  events (`setPointerCapture` no-ops for synthetic pointers).

- **Dioxus `ddd::Service::snapshot` must `peek()`, not `read()`.** A `read()` inside a
  snapshot whose commit writes the same signal (in a `use_effect`) is an infinite
  boot render loop (blank page). Compile/unit checks do not catch it.

- **No domain work in the renderer.** Never format/parse/construct domain values
  (coordinate display, hotkey cascade, "where would this go") in a component. Call
  into the domain crate; domain calls happen at write time, not render time. (See
  `docs/ARCHITECTURE.md`.)

## Repo facts

- **`develop` is a throwaway branch** — a complete rewrite, squashed on merge to
  `master`. Do not suggest commit-message/stash/commit-hygiene work; focus on the
  code and the design.

- **Commit identity:** commit as `Clemens <clemenscodes@gmail.com>` (the global git
  config), never a session-context email.

- **Why this repo exists:** it is a playground to extract a personal Dioxus
  meta-framework — the long-term deliverable is migrating these prose rules into
  types that enforce them at compile time. Resist premature framework-ization
  (rule of three).
