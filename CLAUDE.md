# Project rules

> ## ⛔ RULE ZERO — READ ALL DOCS IN THE DEEPEST DETAIL, BY DEFAULT, FOR ANY TASK ⛔
>
> **No change will EVER be approved unless you have first studied ALL of the project
> docs in the deepest possible detail.** This is not scoped to "non-trivial" changes,
> not to changes that "touch state", not to anything — it is **every task, always, by
> default, no exceptions.** A one-line edit and a full subsystem rewrite carry the
> exact same prerequisite.
>
> Before your FIRST action on any task, Read — in full, no offset, no limit, top to
> bottom, every single line, in the deepest grit and detail, not skimmed and not from
> memory:
>
> - `docs/ARCHITECTURE.md`, `docs/COMPONENTS.md`, `docs/RUST_STYLE.md`, `docs/AGENTS.md`
> - and any task-specific handoff in `docs/` (e.g. `docs/CQI_CASCADE_HANDOFF.md`), plus
>   the reference component trees they name (`shell/header`, `shell/footer`,
>   `grid_editors/*`) **in full**.
>
> **Why this is Rule Zero:** every hard-won rule that keeps this codebase coherent lives
> in those docs. An agent that skimmed them mass-converted ~150 files, shipped
> completely broken layout (the whole `cqi` model depends on a rule stated plainly in
> `COMPONENTS.md`), had everything reverted, and was fired. **Skimming the docs is the
> single most expensive mistake you can make here.** The `spec-gate.sh` hook enforces a
> floor by blocking edits until it sees full-file Reads — treat that as the minimum, not
> the goal. Reading everything deeply first is mandatory and always worth it.

This project edits **`CustomKeys.txt`** for Warcraft III: Reforged. It is a
pure-frontend web app — no server, no database, no cloud.

## What this actually is — understand the game first

You cannot reason about hotkeys until you understand the game they belong to.
The domain is not "5 races and some keys"; it is Warcraft III. Grasp this before
touching any `Race`, unit, or hotkey code — none of it is stated in the code.

**The game.** Warcraft III (Reign of Chaos + its expansion The Frozen Throne,
re-released as *Reforged*) is a **real-time strategy** game. You pick a race,
build a base, harvest three resources (**gold, lumber, food/upkeep**), train
units and RPG-like **Heroes** (they level up and learn abilities), research
upgrades, and fight. The map is also full of **creeps** — neutral hostile units
guarding gold mines and neutral buildings.

**Four races are selectable at the start of a match: Human, Orc, Night Elf,
Undead** — a fact about the game, not the data (you will not find it in
`warcraft-data`). The fifth `Race` variant, **Neutral**, is the one you cannot
pick at the start — yet everyone plays it anyway: **tavern Heroes** (bought
in-game by any race), hireable **mercenaries** that fight for you, plus creeps
and neutral buildings. Those neutral units and heroes need custom hotkeys just
like the rest, so **in this editor Neutral is a full, equal race alongside the
four** — five first-class race tabs, none second-class. Either way the race set
is *closed, compile-time, and game-defined*: exactly these five, each once —
never runtime data you "fetch" through a service or an aggregate, and never a
`[Race; N]` that could hold duplicates or the wrong count.

**The command card.** Select any unit or building and the UI shows its **command
card**: a fixed **4×3 grid of 12 buttons** for everything it can do — Move /
Stop / Hold / Attack, its spells and abilities, a worker's build menu, a
building's train/research menu. Different races have different units and
buildings, hence different command cards — that is why the editor is organised
by race.

**Hotkeys.** Each button can be fired by a keyboard key. WC3 is brutally
APM-intensive (200+ actions/minute) and the *default* keys are scattered across
the keyboard and differ per unit — ergonomically awful. Players remap them, most
commonly to a **Grid layout**: the 12 command-card positions map onto a fixed
keyboard block (e.g. `QWER / ASDF / ZXCV`), so the key you press is the button's
**position**, not a per-ability letter to memorise.

**`CustomKeys.txt`** is the file the game reads to apply those overrides. Each
entry is a section keyed by an object's four-character rawcode, with hotkey
fields **`Hotkey`**, **`Unhotkey`** (cancel/unlearn) and **`Researchhotkey`** (a
hero learning an ability), plus grid-position fields **`Buttonpos`**,
**`Unbuttonpos`**, **`Researchbuttonpos`** given as `x,y` in that 4×3 grid
(`x` 0–3 left→right, `y` 0–2 top→bottom). **This app edits exactly that file.**
The whole domain is: races → their units/buildings → command-card buttons →
those buttons' hotkeys and grid positions.

## Improvising is strictly forbidden

This repo is a deliberate system of conventions and specs. **Never invent,
guess, or "try" a new pattern.** Every recurring shape already has exactly one
canonical form — find it and mirror it exactly:

- Read-side domain data → a `ddd::Query` in `services/customkeys/queries/` +
  `impl QueryHandler` on the service + a service method + a `use_*_service()`
  accessor. **Never** an ad-hoc struct reaching for domain data (that is a wall
  violation).
- Before writing anything, grep a sibling (an existing query, component, or
  command) and copy its structure line-for-line.
- **A local `[patch]` is never an option** — it can never ship (CI breaks on
  it). A change that "needs" patching an external pinned dep
  (`warcraft-data`, `ddd`, …) is a follow-up in *that* repo (edit → publish →
  retag), not a local workaround here.
- If the spec is genuinely silent, or a rule would have to break, **stop and
  surface it** with a recommendation. Do not decide unilaterally.

## Only three commands exist — nothing else is allowed

There are exactly **three** commands you may ever run for this project. Do not
invent, guess, or "try" any other target, and do not fall back to bare
`cargo`/`dx`/`playwright`/`moon <anything-else>`:

- **Test gate:** `moon run :ci` — the one and only way to verify. It runs
  fmt, clippy, tests, the wasm build, and the Playwright e2e gate across every
  project. If you want to know whether the work is green, you run `moon run
  :ci`. Full stop. Never `moon run <crate>:rust/test`, never `moon run
  e2e:playwright/test`, never `cargo test`, never `cargo clippy`, never
  `cargo check` — those are not the gate and are forbidden.
- **Compile check:** `moon run :check` — a fast "does it compile" check
  (`cargo check` against the wasm target, nothing else). Use it to iterate
  quickly while writing code; it runs no fmt, clippy, tests, wasm build, or
  e2e. It is **NOT** the gate and it does **NOT** verify the work — passing
  `:check` says only that the code type-checks, never that it is green. When
  the work is done you still run `moon run :ci`. This is the one and only
  allowed compile-check shape: never bare `cargo check`, never a narrower
  `moon` target.
- **Dev server:** `moon run :dev` — the one and only way to run the app.
  Never `dx serve`, never `moon run hotkey-editor:dx/serve`, never anything
  else.

  **THE DEV URL IS `http://localhost:8123/warcraft-hotkey-editor/` — NOT `/`.**
  The app is served under the base path `warcraft-hotkey-editor`
  (`base_path` in `crates/hotkey-editor/Dioxus.toml`). The bare root `/`
  and `/index.html` return **404** — that is not a broken server, it is you
  using the wrong URL. ALWAYS open/navigate/screenshot
  `http://localhost:8123/warcraft-hotkey-editor/` (trailing slash included).
  Routes hang off that base: editor `…/warcraft-hotkey-editor/`, collisions
  `…/warcraft-hotkey-editor/collisions`, resolve `…/warcraft-hotkey-editor/resolve`.
  (The gallery — `crates/gallery`, port 8200 — has no base path and serves at `/`.)

  **DO NOT wait on the "Your app is being rebuilt" overlay.** The dev server
  hot-rebuilds Rust *and* CSS, and `dx` regularly gets STUCK showing that
  overlay (or a stale page) long after the build is actually done — it will
  never "finish" if you sit polling `dev.log` for a new "Build completed" line.
  The fix is almost always a **page refresh** (re-navigate to the base URL),
  NOT waiting. Refresh first; only investigate a real build error (a compiler
  error printed in `dev.log`) if the refresh still shows the overlay. Hours have
  been wasted waiting on a `dx` stuck state that one reload cleared.

Any other command shape is wrong by definition. If `moon run :ci` fails, fix
the code and run `moon run :ci` again — never route around it with a
narrower command.

Four documents define the rules of this project. Per **Rule Zero** above, all
are mandatory reading — in full, in the deepest detail — before **any** task, not
merely "non-trivial" ones:

- `docs/ARCHITECTURE.md` — _where_ code lives (the wall between renderer
  and domain crate, the localStorage source-of-truth model).
- `docs/RUST_STYLE.md` — _how_ Rust code is written (naming, no tuples,
  private fields, no `as` casts, etc.).
- `docs/COMPONENTS.md` — _how_ renderer components are named, laid out on
  disk, and written (directory equals component equals class, pure-RSX
  bodies, props via `From<&ParentProps>`, base/variant layout). Distilled
  from the `grid_editor` subsystem, the worked example for every rule.
- `docs/AGENTS.md` — _how you work_: the commands/CI rules (never run a
  second `moon run :ci` concurrently — there is no CPU-load flakiness), the
  dev/e2e loop and its traps, and the hard-won technical gotchas
  (pointer-capture click-vs-drag, snapshot `peek`, coupled e2e selectors).

If you skip these and "just patch the bug", you will almost certainly
violate one of the rules below and reintroduce a bug we already fixed.
Do not do this.

---

## The wall

There are two halves of this project, and a wall between them. **Do not
move logic across the wall.** If a change feels like it has to, you are
solving the wrong problem.

```
hotkey-editor (wasm, Dioxus)        warcraft-keybinds (pure Rust)
───────────────────────────         ────────────────────────────
renders state                       owns state shape
dispatches commands                 parses, serializes, normalizes
reads localStorage                  cascades, dedupes, materializes
writes localStorage                 has 100% test coverage
no domain logic                     no browser deps
```

## Hard rules

These are the rules. They are not guidelines. They come from real bugs.

1. **localStorage is the source of truth.** The single key
   `warcraft-hotkey-editor.custom-keys` holds the full canonical
   `CustomKeys.txt` text. There is no in-memory state that diverges from
   it. Every mutation writes to localStorage in the same tick.

2. **Stored state is fully normalized.** The text in localStorage has
   every cascade resolved, every collision settled, every default
   materialized. Reading it gives you the final positions and hotkeys.
   Renderer never re-derives them.

3. **The renderer never computes domain decisions.** No cascade lookup,
   no collision resolution, no "where would this go", no "what hotkey
   would this be". If you want one of those, call into
   `warcraft-keybinds`. Domain calls happen at write time, not at render
   time.

4. **The renderer never mutates `CustomKeysFile` directly.** All
   mutations go through named domain commands on the canonical
   `CustomKeys` facade. UI code never calls `binding.set_*` itself.

5. **Export and preview are `localStorage.getItem(KEY)`.** Nothing more.
   Do not re-serialize, re-overlay, or re-normalize at export time. If
   the export is wrong, the bug is in the mutation that produced bad
   state — fix that.

6. **Boot path:** read localStorage if present, else load the bundled
   default text, run it through the domain normalize function, write the
   normalized result back to localStorage, then render.

7. **Imports replace, then normalize.** Upload and template-apply both
   hand the new text to the domain crate, get a normalized text back,
   write it to localStorage. No "overlay onto the in-memory copy".

8. **`warcraft-keybinds` is pure Rust.** No `wasm-bindgen`, no
   `web-sys`, no `dioxus`, no `gloo`. Native cargo test must run on it.
   Allowed deps: `warcraft-api`, `warcraft-database`, optionally `serde`.

9. **Every domain change ships with tests.** A bug fix starts with a
   failing test that reproduces the bug. Cascade behavior, collision
   resolution, duplicate detection, grid-layout application — all
   covered.

10. **UI state ≠ domain state.** Dialog open, current selection, drag
    state → UI signals. Hotkey, position, binding fields → CustomKeys.txt.

## Rust style — mandatory

Full rules: `docs/RUST_STYLE.md`. Headlines (every one is enforced):

- **Full semantic names everywhere.** No abbreviations, no single letters,
  no shortened forms — types, fields, locals, parameters alike.
- **No section header comments** (`// === Rendering ===`). Split the file
  instead.
- **No tuples in any form.** No plain tuples, no tuple structs, no
  newtypes. Always named structs with named fields.
- **No `print*` functions.** Implement `Display`, call `println!` at the
  call site.
- **Private fields with explicit accessors.** No `pub` fields to skip
  writing getters/setters.
- **Assign structs to a variable before passing them.** No inline struct
  construction in argument position.
- **No evaluated expressions as arguments.** Bind every method call,
  field access, or conversion to a named local first; functions receive
  plain variables only.
- **No inline numeric type suffixes** (`0u32`, `2.0f32`). Annotate the
  binding instead.
- **Prefer struct composition over field copying.** When all fields come
  from another struct, embed it as a named sub-field.
- **No `verb_noun` free functions** (`render_hero`, `parse_ability`).
  Make the noun a struct and the verb a method.
- **No `as` casts outside `From`/`TryFrom` impl bodies.** Use `From`,
  `Into`, or `TryFrom` everywhere else.
- **Use idiomatic standard traits — highest priority.** If a standard
  trait covers what you are doing (`Display`, `From`, `TryFrom`,
  `Default`, `Iterator`, `FromStr`, `Error`, `Index`, etc.), implement
  it. Never invent a parallel convention. See `docs/RUST_STYLE.md`.
- **Derive every trait the type qualifies for.** Always derive
  `Clone`, `Debug`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`,
  `Copy`, `Default` whenever the type mechanically supports them. `Copy`
  must be derived (not skipped) for any type that is cheaply bitwise-
  copyable (small value types, ids, enums without heap data). Do not
  leave these off because writing them out feels like effort.
- **Use `Self` inside `impl` blocks.** Never repeat the concrete struct
  name — return types, construction, associated calls all use `Self`.

These apply to every line of new Rust. They also apply to existing code
you edit — if you touch a function, leave it conformant on the way out
within the scope of the change. Don't drag in scope-creep cleanups; do
respect the rules in the lines you write.

## When you start a task

- Re-read `docs/ARCHITECTURE.md` if your change touches state, persistence,
  cascading, or any "what position is this at?" question.
- Re-read `docs/COMPONENTS.md` if your change adds or refactors a renderer
  component. Mirror the `grid_editor` shape: directory equals component
  equals class, pure-RSX body, child props via `From<&ParentProps>`.
- If the task seems to require breaking a rule, stop and surface it to
  the user. Don't decide unilaterally.
- If you see a violation while doing unrelated work, say so. Don't expand
  scope to "fix it while you're there" without asking.

## When you finish a task

- Confirm `moon run :ci` is green. This now includes the Playwright e2e
  gate — all four smoke tests must pass.
- For UI changes, actually open the app in a browser and use the feature.
  Type checking and tests verify code correctness, not feature
  correctness.
- Confirm no new code in `hotkey-editor/` imports from
  `warcraft_keybinds::cascade` or calls `binding.set_*`. If it does, you
  added a violator — route the change through the `CustomKeys` facade
  instead.
- Re-read your diff against `docs/RUST_STYLE.md`. Common slips to grep
  for: tuple return types `-> (`, tuple structs `struct \w\+(`, `pub `
  fields, `as ` casts, `print_` function names, single-letter locals,
  inline struct literals at call sites, numeric suffixes like `0u32`.
- For component changes, re-read your diff against `docs/COMPONENTS.md`.
  Common slips: a directory whose name differs from its component or CSS
  class, `let`/logic in a component body, child props passed by hand
  instead of `From<&ParentProps>` spread, an `Option<T>` prop for a value
  that is always present, a variant nested inside its base instead of a
  flat sibling.
