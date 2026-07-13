# HANDOFF — Routing / Shell / early-app overhaul

## The goal (get this right first)
Do a **complete rework of the routing mechanism in the early app / shell / layer** —
`crates/hotkey-editor/src/components/app/{route.rs,mod.rs}`, everything under
`components/app/components/shell/`, and `services/navigation/**`. The **wall violations were a tiny
fraction** and are already fixed; "wall fixes" and "CI green" are NOT the goal. Verify both in the
browser (back/forward, deep links, search, view switches) and with `moon run :ci`.

## HARD spec rules that were violated (do NOT repeat)
- **No abbreviations, EVER** (RUST_STYLE "Full semantic names everywhere"). **"Nav" is forbidden.** The
  existing `NavSnapshot`, `NavDecision`, `DecodedEditorNav`, `nav_snapshot.rs`, `use_synced_route`, etc.
  are themselves spec violations — **rename them to full `Navigation…` forms** as part of this rework
  (`NavigationSnapshot`, `NavigationDecision`, `DecodedEditorNavigation`, `navigation_snapshot.rs`, …).
  Never introduce a new "Nav" anything.
- **No `verb_noun` / `_for`-suffixed names** (RUST_STYLE). `entry_for(kind)` is forbidden. Model it as a
  noun — `Index` by `CollisionKind`, a method on the kind, or a named struct — never `x_for`.
- Read `docs/RUST_STYLE.md`, `docs/COMPONENTS.md`, `docs/ARCHITECTURE.md` **in full** before editing
  (the spec-gate hook enforces this on the first edit each session).

## Target architecture
**Route = single source of truth; the navigation signals become a pure read-cache** (mirrors the app's
own R1 localStorage model: one source of truth, signals rebuilt from it). Make it **unidirectional**:
- **Write side:** every mutation (view switch, race/mode/unit change, search, collision/resolve entry
  pick) builds a typed navigation snapshot and pushes/replaces a `Route`. **Push-vs-replace is decided
  at the mutation site** (page/context change → push; entry pick or search past the first keystroke →
  replace). This replaces `NavigationDecision::between`.
- **Read side:** one direction only — route params reconcile into the cache signals (keep the existing
  `restore`/`restore_view` idea minus the guard, or a single central effect).
- **DELETE:** the `synced_route` echo guard (its Signal + `use_synced_route` context + every
  `synced_route.set`), `use_url_sync` (the signal→route diff), `NavigationDecision`, and simplify the
  `RouteBootstrap` canonicalize-on-entry.
- **Layering seam:** `Route` lives in the component layer (`components/app/route.rs`); the
  `services/navigation` layer must not import it. Give the navigation service a **component-layer
  callback** (supplied by the shell) that it invokes with a typed command; the shell — the only place
  that may name `Route` — does `navigator().push/replace(Route::from(&snapshot))`. (ARCHITECTURE.md §3:
  internal layering is organizational, not a hard wall, but keep it clean.) Name the command type fully
  (`NavigationCommand`), never "Nav".

## Mutation surface (already mapped)
Consumers only READ the accessors (many sites, leave unchanged). Convert these to route-push:
- `ViewNavigationContext` mutators: `apply(view)` — 5 sites (brand_host, collisions_button_host,
  burger_menu, resolve_button, collisions_page breadcrumbs); `select_race` (race_tabs_host);
  `select_mode` (mode_tabs_host); `open_unit` — 4+ sites (island_conflict_unit, conflict_ability,
  conflict_detail_unit, carrier_card, fight_column).
- Reconcile (route→signals) to keep/simplify (drop `synced_route.set`): `editor_page/presentation`
  (`restore`), `collisions_page/presentation` (`restore_view` + selection-from-route),
  `resolve_page/presentation` (`restore_view` + selection).
- Entry PICKS (currently set a selection signal directly → must push/replace a route instead):
  `…/sidebars/island_sidebar/presentation/mod.rs:22` (`selected_island.set`). The hotkey-unit and
  unit-position pick sites did NOT surface in a `.set(` grep — find them (likely the other sidebars /
  detail components). Resolve move-category pick: `resolve_page/presentation` (~line 598).
- Search: `…/unit_list/presentation/mod.rs:78` (`search_query.set(value)`) and `:84` (clear). Route it
  through a search mutation that keeps the debounce/session (push the first keystroke, replace the rest,
  500 ms reset) — that logic is in `use_url_sync`'s `SessionQuery` arm today.
- `synced_route.set` sites to DELETE: `editor_page/presentation:26`, `collisions_page/presentation:553`,
  `resolve_page/presentation:605`, `shell/presentation/mod.rs:396/401/413`.

## Also in scope: the god-hook
`shell/presentation/mod.rs::use_app_signals` births ~29 signals and provides ~7 contexts; `use_shell`
adds ~5 more. `EditorState::new` takes 15 args under `#[allow(clippy::too_many_arguments)]` (the tell).
Consider each service owning/creating its own signals seeded from the route via its own `use_*`
initializer, instead of the shell minting them all and threading them in.

## Repo state right now
- **Wall fixes DONE & correct (keep):** template import routed through `CustomKeysService::import_overlay`;
  resolve-apply uses a new `ResolveConflictsCommand`
  (`services/customkeys/commands/resolve_conflicts_command.rs`, registered in `commands/mod.rs`,
  `service.resolve_conflicts()`); preview reads `exported_text()`; removed an unused `use ddd::Service`
  in `resolve_page/presentation`. Correct but **UNVERIFIED by CI** (CI was blocked by the tw-lint OOM).
- **Navigation rework: NOT started.** Two exploratory nav edits (a `NavCommand`, an `entry_for`) were
  reverted for using forbidden names; `nav_snapshot.rs` and `collision_selection/mod.rs` are back to
  their original content. Start clean.

## tw-lint OOM (separate; fixed & verified, needs deploy — this blocks CI)
The `tailwind/lint` CI task runs `tw-lint` (repo `~/.local/src/tw-lint`), which OOM'd on this large repo
(the 11-min "hang" was the client blocking on the already-dead server — a symptom). Cause: (1)
`join.rs::collect_corpus` preloaded every source file's text + every class block; (2) a NEW synthetic LSP
document per chunk, so the language server's memory grew with the chunk count. Fixed in tw-lint: streaming
(bounded to one chunk / one file — new `each_source_file`) + one reused synthetic document via `didChange`
(new `Synthetic` type; `Client::open_document`/`change_document` in `lsp/client.rs`). Verified: the full
warcraft repo completes under a 1.5 GB cap and all 3 tw-lint e2e tests pass. **NOT deployed:** the warcraft
`flake.nix` input `tw-lint = "github:clemenscodes/tw-lint"` is still the stale pinned rev — CI won't pick
up the fix until tw-lint is committed+pushed and the flake input bumped (`nix flake update tw-lint`).

## Verify
`moon run :ci` (fmt + clippy + tests + wasm build + e2e). Won't go green until the tw-lint fix is deployed.
Then browser-verify at `http://localhost:8123/warcraft-hotkey-editor/` (base path — bare `/` is 404):
editor `…/`, collisions `…/collisions`, resolve `…/resolve`; test back/forward, deep links, search typing,
and race/mode/unit switches.
