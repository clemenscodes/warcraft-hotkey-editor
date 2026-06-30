# Dialog & Grid refactor — handoff

Continuation handoff for the dialog/grid component refactor. Start by reading
`docs/COMPONENTS.md` in full: it was amended heavily this session and is the
contract. This file says where things stand and what to do next.

## The mission

Reshape every dialog (and the command grid) to the `grid_editor` component
style: one class per component, props in `props.rs`, logic in hooks, CSS owned
per component, public modules with full-path imports.

## Rules established this session (all now in `docs/COMPONENTS.md`)

Read the doc, but the headline rules, each of which has a worked example there:

1. **Body is flat hooks then pure RSX.** Logic composes through hooks: primitive
   leaf hooks (`use_custom_keys`, `use_dialog_open`, `use_body_scroll_lock`, …)
   plus one composed hook per component. Domain / localStorage / web APIs are
   reached only through hooks.
2. **CSS isolation by construction.** Every component owns its CSS in its own
   `styles/` dir. Global stylesheets hold only design tokens (`var(--color-*)`,
   `--font-*`). Duplicating a few rules beats coupling. Nothing global styles a
   component.
3. **One attribute per line in RSX.**
4. **One class per component.** Each markup file has exactly one classed element,
   its root. Every other classed element is its own child component. There is no
   "too many components", only too few — a single styled paragraph is a
   component. Variant/state modifiers on the root (`button button-primary`) are
   fine; a second distinct classed element in the file is not.
5. **Library components are exempt** from dir==component==class: `document::Stylesheet`
   and the `dioxus_primitives` dialog parts (`DialogRoot`, `DialogContent`) may
   appear in a body directly.
6. **Modules are `pub mod`; imports carry the full path.** NO flat re-export
   lists that thread descendants up the tree. A component re-exports only its own
   `pub use props::XProps` (and `state`). Children are reached by traversing
   `pub mod`. `lib.rs` curates the gallery's public surface with full-path
   `pub use`.
7. **Props always live in `props.rs`, never inline in `mod.rs`.** (Broken four
   times this session — do not.)
8. **Base + variants are flat siblings.** Behaviour-generic base vs plain leaf
   composition (see COMPONENTS.md).

## Architecture decisions (don't relitigate)

- `dioxus-primitives` stays an external dep. It is NOT vendored: the registry
  entries are styled wrappers that still depend on the crate, so vendoring buys
  nothing. `#[css_module]` (manganis CSS Modules) was considered and declined.
  The per-component plain-CSS is a transitional step; the user intends to bridge
  back to pure Tailwind after the CSS migration. Stay on the established pattern.
- **Grid is a pure drawer.** `GridProps { tiles: [GridTileProps; COMMAND_GRID_TILE_COUNT] }`.
  All behaviour (drag/drop, selection, moves) lives in `GridEditor`:
  `grid_editor/logic/render.rs` builds the finished tiles with handlers,
  `logic/mechanics.rs` + `logic/drag_state.rs` are the verbatim drag code,
  `DragFollowerOverlay` is the editor's. `HeadedGrid`
  (`grid_editor/components/headed_grid/`) is the presentational heading+grid, with
  `grid` and `grid_heading` as its subcomponents. `impl From<&RenderedTile> for
  GridTileProps` (in `grid_tile/props.rs`) is the single domain→tile mapping the
  editor and the templates preview both use. `COMMAND_GRID_TILE_COUNT`
  (= columns × rows = 12, hard invariant) lives in `warcraft-keybinds`.
- **Dialog base** (`dialogs/dialog/`): `Dialog` (`.dialog` overlay) → `DialogPanel`
  (`.dialog-panel`) → `DialogHeader` + `DialogBody` + `DialogFooter`. A variant
  composes `Dialog` and passes `panel_class` (its own size/identity class),
  `children` (the body), and an optional `footer: Option<Element>`. `DialogFooter`
  renders `.dialog-footer` only when `Some`.
- **e2e selectors target real component classes** (`.template-card`,
  `.preview-textarea`, `.download-info-actions`). NEVER reintroduce identity
  classes onto the markup to make tests pass — fix the selectors.

## Done and verified (full `moon run :ci` green: 133 Playwright e2e + all rust tests + clippy + fmt)

- Grid refactor: behaviour → `GridEditor`, `Grid` pure, `HeadedGrid`, `[_; 12]`
  tiles array.
- `pub mod` + semantic-path conversion of the whole `grid_editors` and `dialogs`
  subtrees, plus every consumer (`lib.rs`, `views`, `tile_override`,
  `unit_detail`, the templates preview).
- Converted dialogs, fully decomposed one-class-per-component with `props.rs`:
  `preview_dialog`, `download_info_dialog`, `upload_info_dialog`,
  `templates_dialog`. The templates preview reuses `HeadedGrid`.
- Shared `Button` leaf (`components/shared/button/`, Primary/Secondary).
- Templates preview sizing fixed: `.headed-grid` uses `flex: 1 1 0` (not
  `width: 100%`) so the two preview grids sit side by side and shrink to
  thumbnail size; the editor (single grid) is unchanged. Verified in the browser.
- `DialogFooter` slot added to the base (this is the only new, unconverted-content
  piece in flight).

## In progress: help_dialog

The base now has the footer slot help_dialog needs. The content is NOT yet
converted. `dialogs/help_dialog/mod.rs` (~345 lines) still uses the OLD
`.dialog-overlay` / `.dialog-shell` / `.help-dialog` structure and inline data.

To convert it:
- Move the `const` data (`CONFLICT_KINDS`, `RESOLVER_WALKTHROUGH`, `RESOLVER_PARTS`,
  `RESOLVER_MOVES`, `LEGEND_ENTRIES`) out of the body into a data/logic module.
- The dismiss action (`OnboardingPersistence::mark_seen`) → a hook or a `From`-built
  handler, not in the body.
- Decompose into ~2 dozen one-class components: a body wrapper, `HelpSectionTitle`
  (reused), `HelpCallout`, `HelpWorkflow`/`HelpWorkflowStep`, `HelpInlineIcon`,
  `HelpLegend`/`HelpLegendRow`/`HelpLegendIcon`/`HelpLegendLabel`/`HelpLegendDescription`,
  `HelpResolverProse`, `HelpBodyText` (reused), `HelpStepNumber`, the glossary
  columns/entries, and the dismiss button rendered into the `Dialog` footer slot.
- e2e: `onboarding.spec.ts` and `archmage-qwer-rearrange.spec.ts` locate `.help-dialog`.
  When you convert it, update those to a real help component class (the same way
  the other dialog selectors were fixed).

## Remaining after help_dialog

- `layout_editor` (~284 lines): picker/snapshot logic + toast → a hook; the grid
  of editable cells → components. Still old dialog structure.
- `system_hotkeys` dialog + `key_picker_dialog` + `tile_override/position_picker`
  dialogs → the `Dialog` base.
- Empty the global dialog CSS once every consumer is converted:
  `styles/primitives.css`, `styles/mobile-foundation.css`,
  `components/dialogs/dialogs.css`. The OLD `dialogs/dialog_header`, `help_dialog`,
  and `layout_editor` still depend on those legacy classes.

## How to verify

- `nix develop -c moon run :ci` is the gate (fmt, lint, test, build, the four
  Playwright suites). Do NOT pipe it through `tail` — that masks the real exit
  code. Capture `$?` or read the log.
- Fast loop: `nix develop -c cargo clippy -p hotkey-editor -p gallery --target wasm32-unknown-unknown`;
  `cargo test -p warcraft-keybinds`; `cargo fmt --check`.
- Dev server is at `localhost:8123/warcraft-hotkey-editor/`. Component CSS changes
  need a rebuild (dx serve does not hot-reload `asset!` CSS). Use the Playwright
  MCP, not headless chromium. The onboarding help dialog auto-opens and intercepts
  clicks — close it first.

## Pitfalls hit this session (avoid)

- Props inline in `mod.rs` — always `props.rs`.
- Reintroducing identity/legacy classes to satisfy e2e — fix the selectors instead.
- Flat re-export lists threading descendants up — `pub mod` + full paths.
- `manganis` `asset!` caps source paths near 256 bytes; very deep nesting with
  long repeated prefixes overflows. Keep names tight (drop redundant subsystem
  prefixes per COMPONENTS.md).
- The `gallery` crate (`crates/gallery`) imports components from the `hotkey_editor`
  lib root. When you rename/move/delete a component, update its stories and the
  `lib.rs` re-exports, or `gallery:rust/build` fails.
