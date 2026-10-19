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
> - `docs/ARCHITECTURE.md`, `docs/COMPONENTS.md`, `docs/RUST_STYLE.md`, `docs/AGENTS.md`,
>   `docs/DOMAIN.md`, `docs/PRODUCT.md`
> - and any task-specific handoff in `docs/` (e.g. `docs/CQI_CASCADE_HANDOFF.md`), plus
>   the reference component trees they name (`shell/header`, `shell/footer`,
>   `grid_editors/*`) **in full**.
>
> **Why this is Rule Zero:** every hard-won rule that keeps this codebase coherent lives
> in those docs. An agent that skimmed them mass-converted ~150 files, shipped
> completely broken layout (the whole `cqi` model depends on a rule stated plainly in
> `COMPONENTS.md`), had everything reverted, and was fired. **Skimming the docs is the
> single most expensive mistake you can make here.** The `full-spec-gate.sh` hook enforces
> a floor by blocking the first edit/write/bash of a session and reproducing all six docs
> in full in your context, treat that as the minimum, not the goal. Reading everything
> deeply first is mandatory and always worth it.

This project edits **`CustomKeys.txt`** for Warcraft III: Reforged. It is a
pure-frontend web app — no server, no database, no cloud.

## The six documents that define this project

Per **Rule Zero**, all six are mandatory reading — in full, in the deepest detail —
before **any** task. The rules themselves live in these docs; this file carries only
the meta-layer they do not cover, so **do not re-derive or restate one of their rules
here.**

- `docs/ARCHITECTURE.md` — _where_ code lives: the wall between renderer and domain
  crate, the localStorage source-of-truth model, and the hard rules **R1–R10**.
- `docs/RUST_STYLE.md` — _how_ Rust is written (full semantic names, no tuples, private
  fields, no `as` casts, idiomatic standard traits, derive everything).
- `docs/COMPONENTS.md` — _how_ renderer components are named, laid out on disk, and
  written (directory == component == class, pure-RSX bodies, render-tree ==
  directory-tree, compose a look rather than share styles). The `shell/header` /
  `shell/footer` subsystems are the worked example.
- `docs/AGENTS.md` — _how you work_: the exact commands (`moon run :ci` / `:check` /
  `:dev`), the dev URL and its base path, the dev/e2e loop and its traps, and the
  hard-won gotchas (pointer-capture click-vs-drag, snapshot `peek`, coupled e2e selectors).
- `docs/DOMAIN.md` — _how_ the domain crate `warcraft-keybinds` is structured: its
  DDD/CQRS conventions, module layout, and the three aggregate roots.
- `docs/PRODUCT.md` — _why_ the product exists and _for whom_, plus the game and domain
  primer: the command card, the `CustomKeys.txt` format, grid layouts, and how the five
  races are modeled.

If you skip these and "just patch the bug", you will almost certainly violate one of
their rules and reintroduce a bug we already fixed. Do not do this.

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
  (`warcraft-data`, `ddd`, …) is a follow-up in _that_ repo (edit → publish →
  retag), not a local workaround here.
- If the spec is genuinely silent, or a rule would have to break, **stop and
  surface it** with a recommendation. Do not decide unilaterally.

## Working here

- If you notice a violation while doing unrelated work, **say so**. Do not expand scope
  to "fix it while you're there" without asking.
- Verification is `moon run :ci` green, and for any UI change actually opening the app in
  the browser and using the feature (see `docs/AGENTS.md`). Type checks and tests verify
  code correctness, not feature correctness.
