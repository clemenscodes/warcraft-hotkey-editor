# Editor Controls — Phase B.2 (Inline Search-Scope Dropdown) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move the Unit/Ability search-scope selector into the search bar as an inline **`Unit ▾` dropdown** pinned at the left of the search box, and delete the standalone `SearchFieldToggle` from the controls row.

**Architecture:** A new connected `SearchScopePicker` dropdown mirrors the existing `HeroLevelPicker` (the repo's canonical hand-rolled dropdown) — a `relative` wrapper → trigger button (closed/open variant split, `Unit ▾`/`Ability ▾` + chevron) → gated `if is_open` menu (`role="listbox"`, two options) + a full-screen transparent backdrop. Open state is a local UI `use_signal::<bool>`; the picker self-sources the scope signal exactly like `SearchFieldToggle` did (`use_editor_state().search_field()`, `.set(SearchField::UnitName|Ability)`). The search box styling re-homes from the `<input>` up to the `unit_list_search` container so the scope picker and input share one framed box with a divider; the input goes borderless. `SearchFieldToggle` is then removed from `UnitControls`.

**Tech Stack:** Rust, Dioxus 0.7, `tw_macro` styling, the `ddd` `View → Model → Presentation` pipeline, `warcraft_api::SearchField`. No new dependencies (no `dioxus_primitives` — dropdowns are hand-rolled here).

**Scope note:** Phase B.2 only. B.1 (unify the search look) already shipped. Phases C/D/E get their own just-in-time plans. Design source: `docs/superpowers/specs/2026-07-15-editor-controls-affordances-redesign-design.md` (§5 Phase B.2).

## Global Constraints

- **Verification is the running dev server + the browser — NEVER run `moon`/`cargo`/`dx` while the dev server is up.** Running `moon run :check` against the live dev server fights it for the cargo build lock and wedges its rebuild (this cost hours in Phase A). The dev server compiling + serving **is** the compile signal. After any class edit, `touch crates/hotkey-editor/tailwind.css` then reload `http://localhost:8123/warcraft-hotkey-editor/` (trailing slash). Do **not** start the dev server (the user owns port 8123); drive the running one with Playwright. Do **not** run `moon run :ci` (the user runs the gate). Do **not** commit unless the user asks.
- **Mirror, don't invent.** The dropdown copies `HeroLevelPicker` structurally; the ARIA copies `system_hotkeys_breadcrumbs`. Read those live files and adapt — do not design a new dropdown.
- **`directory == component (PascalCase) == class (kebab)`**, enforced by `assert_component!`. One classed root per component. Mutually-exclusive looks (closed/open trigger, active/idle option) are **separate components behind a `match`/`if` dispatcher**, never a state-keyed class table.
- **Render-tree == directory-tree:** a component you render is your child, reached via `use components::…` — never a cross-subtree `use super::…`/`use crate::…` for a rendered child.
- **`View → Model → Presentation`:** leaves carry `view/` + `model/` (`#[derive(Props)]`, `From<&View>`) + `mod.rs`; a connected component's `presentation/` builder owns the `use_signal`/`use_effect` and context reads and returns a signal-free result; **no `Signal<T>` in a View/Model/Presentation type**.
- **Styling:** `base` carries no band prefix; each band array carries only its own prefix; share utility *values*, never composite looks; tokens for color/type/radius/shadow. Do **not** restyle the shared `ToggleButton`.
- **UI state (dropdown open/closed) is a UI signal**, never written to CustomKeys.txt. Scope (`SearchField`) is read/set through `use_editor_state()`, exactly as `SearchFieldToggle` did.
- **Shell nukes `min-w` on mobile/tablet** (`mobile:**:min-w-0`); size with `w-[…]`/`flex-basis` + `flex-none`, never `min-w`, there.
- **`[…]` values marked `TUNE`** are dialed in the browser.

Path prefix (abbreviated **`…/unit_list/`**):
`crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_workspace/components/race_theme/components/shared/unit_list/`

Canonical siblings to read before writing:
- Dropdown pattern: `…/unit_detail/…/hero_level_picker/` (root `mod.rs` + `presentation/mod.rs` + `hero_level_trigger/` closed+open + `hero_level_menu/` + `hero_level_option/` active+idle + `hero_level_backdrop/`). Find with `rg -l hero_level_picker crates/hotkey-editor/src` / `find … -name hero_level_picker`.
- ARIA: `system_hotkeys_breadcrumbs_trigger` (`aria_haspopup: "listbox"`, `aria_expanded: is_open`) and `system_hotkeys_breadcrumbs_menu` (`role: "listbox"`). Find with `rg -l 'aria_haspopup|role: "listbox"' crates/hotkey-editor/src`.
- The control being replaced: `…/unit_list/components/search_field_toggle/` (its `presentation/mod.rs` self-sources `search_field` and calls `.set(SearchField::UnitName|Ability)`; `data/mod.rs` holds `UNIT="Unit"`, `ABILITY="Ability"`).

---

### Task B2.1: Build `SearchScopePicker` (mirror `HeroLevelPicker`)

Create a self-contained connected dropdown that reads the current scope and sets it, with local open/close state. **Not yet wired into the search bar** (that's B2.2) — it compiles but isn't rendered. Because the dev server compiles the whole crate, an unrendered new component still gets type-checked on the next rebuild; there's no isolated browser check for this task, so verification is "the dev server rebuilds clean (no compiler error in its output) after B2.2 mounts it." Keep this task's deliverable purely additive.

**Files (create — mirror the same file shapes as the `hero_level_picker` sibling, renamed):**
- `…/unit_list/components/unit_list_search/components/search_scope_picker/` with:
  - `mod.rs` (root: renders trigger + `if is_open { menu + backdrop }`)
  - `presentation/mod.rs` (the connected builder: owns `use_signal::<bool>` open state; reads `use_editor_state().search_field()`; exposes `is_open`, current scope label, `toggle`, `dismiss`, and per-option select handlers that `set` the `SearchField` and close)
  - `view/mod.rs`, `model/mod.rs` (empty `SearchScopePickerView;` / `SearchScopePickerModel;` — a parameterless connected component still takes an empty Model built from an empty View, per COMPONENTS.md)
  - `style/mod.rs` (root wrapper: `relative`, `flex-none`; TUNE any width)
  - `data/mod.rs` (`pub(super) const UNIT: &str = "Unit";` `pub(super) const ABILITY: &str = "Ability";` — copied from `search_field_toggle/data/mod.rs`)
  - `components/mod.rs` and the children below.
- `…/search_scope_picker/components/search_scope_trigger/` → `mod.rs` (`if is_open { OpenSearchScopeTrigger } else { ClosedSearchScopeTrigger }`), `view/`, `model/`, plus:
  - `components/closed_search_scope_trigger/` (button, chevron `▾`) and `components/open_search_scope_trigger/` (button, chevron `▴`), each `mod.rs`+`view/`+`model/`+`style/`. The trigger button carries **`aria_haspopup: "listbox"`** and **`aria_expanded: is_open`** (copy from `system_hotkeys_breadcrumbs_trigger`), `r#type: "button"`, `onclick: toggle`, and shows the **current scope label** (`Unit`/`Ability`) + chevron.
- `…/search_scope_picker/components/search_scope_menu/` → `mod.rs` (a `div`/`ul` with **`role: "listbox"`**, renders two options), `view/`, `model/`, `style/` (`absolute`, `top-[calc(100%+4px)]`, `left-0`, `z-20`, panel surface — mirror `hero_level_menu/style`).
  - `components/search_scope_option/` → `mod.rs` dispatcher (`if is_active { ActiveSearchScopeOption } else { IdleSearchScopeOption }`), `view/`, `model/`, plus `components/active_search_scope_option/` and `components/idle_search_scope_option/` (each `mod.rs`+`view/`+`model/`+`style/`; a `<button role="option" aria_selected=…>` with the label; `onclick` = that option's select handler).
- `…/search_scope_picker/components/search_scope_backdrop/` → `mod.rs` (`div { class: CLASS, onclick: dismiss }`), `style/mod.rs` (`fixed inset-0 z-10 bg-transparent cursor-default` — copy `hero_level_backdrop/style`).

**Interfaces:**
- Consumes: `use_editor_state().search_field() -> Signal<SearchField>` (accessor already exists); `warcraft_api::SearchField` (`UnitName` | `Ability`, `Default = UnitName`).
- Produces: `pub fn SearchScopePicker(props: SearchScopePickerModel) -> Element` (empty Model), self-contained. Class names: `search-scope-picker`, `search-scope-trigger`, `closed-search-scope-trigger`, `open-search-scope-trigger`, `search-scope-menu`, `search-scope-option`, `active-search-scope-option`, `idle-search-scope-option`, `search-scope-backdrop`.

- [ ] **Step 1: Read the sibling patterns**

Read every `.rs` under the `hero_level_picker` tree (root `mod.rs` + `presentation/mod.rs` + trigger closed/open + menu + option active/idle + backdrop), plus `system_hotkeys_breadcrumbs_trigger/mod.rs` and `system_hotkeys_breadcrumbs_menu/mod.rs` for the ARIA, plus `search_field_toggle/presentation/mod.rs` + `search_field_toggle/data/mod.rs` for the scope wiring. These are your templates; you will reproduce their file shapes with the `search_scope_*` names.

- [ ] **Step 2: Write the presentation builder (the connected wiring)**

`search_scope_picker/presentation/mod.rs`, adapting `hero_level_picker/presentation/mod.rs` + `search_field_toggle/presentation/mod.rs`. It must:
- own `let mut open = use_signal::<bool>(|| false);`
- read `let mut search_field = use_editor_state().search_field();` and `let current = *search_field.read();`
- expose `is_open = *open.read()`, `current_label` (`data::UNIT` if `current == SearchField::UnitName` else `data::ABILITY`),
- `toggle` = `EventHandler` flipping `open`, `dismiss` = `EventHandler` setting `open` false,
- `select_unit` / `select_ability` = `EventHandler`s that `search_field.set(SearchField::UnitName|Ability)` **and** `open.set(false)`,
- `unit_is_active = current == SearchField::UnitName`, `ability_is_active = current == SearchField::Ability`.
Return a signal-free struct of these values/handlers (no `Signal<T>` escapes). Mirror the exact `EventHandler::new(move |_event: MouseEvent| { … })` shapes from `search_field_toggle/presentation/mod.rs`.

- [ ] **Step 3: Write the root, trigger, menu, option, backdrop components**

Reproduce each `hero_level_*` file as its `search_scope_*` counterpart with the names above, wiring the presentation's fields into the RSX. Root `mod.rs`:

```rust
#[component]
pub fn SearchScopePicker(props: SearchScopePickerModel) -> Element {
    let picker = use_search_scope_picker();
    // destructure picker fields
    rsx! {
        div {
            class: CLASS,
            SearchScopeTrigger { /* current_label, is_open, onclick: toggle */ }
            if is_open {
                SearchScopeMenu { /* unit_is_active, ability_is_active, select_unit, select_ability */ }
                SearchScopeBackdrop { onclick: dismiss }
            }
        }
    }
}
```

Trigger carries `aria_haspopup: "listbox"` + `aria_expanded: is_open`; menu root carries `role: "listbox"`; each option is `<button role="option" aria_selected: is_active>`. Style lists mirror the hero picker's (panel surface, chevron, option hover) using theme tokens; mark any bespoke width/offset `TUNE`.

- [ ] **Step 4: Let the dev server rebuild; confirm no compiler error**

Do **not** run `moon`. Save triggers the dev server's rebuild. Then `touch crates/hotkey-editor/tailwind.css`. Reload the app; it should still render (the picker isn't mounted yet, so nothing visible changes). If the dev server's output (the user's terminal) shows a compiler error, fix it. Expected: app loads unchanged, no error.

- [ ] **Step 5: Commit (only if the user authorizes)**

```bash
git add "crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_workspace/components/race_theme/components/shared/unit_list/components/unit_list_search/components/search_scope_picker"
git commit -m "feat(editor-controls): add SearchScopePicker dropdown (unmounted)"
```

---

### Task B2.2: Re-home the box and mount the scope picker in the search bar

Move the framed box from the `<input>` up to the `unit_list_search` container, make the input borderless, and render `SearchScopePicker` at the left with a divider. This is the visible change.

**Files:**
- Modify: `…/unit_list_search/mod.rs` (render `SearchScopePicker {}` before the input; reach it via `use components::search_scope_picker::SearchScopePicker`)
- Modify: `…/unit_list_search/style/mod.rs` (container becomes the framed box)
- Modify: `…/unit_list_search/components/unit_list_search_input/style/mod.rs` (input goes borderless/transparent)
- Possibly modify: `…/unit_list_search/components/unit_list_search_icon/style/mod.rs` (reposition the magnifier now that the scope picker occupies the left edge — TUNE)
- Modify: `…/unit_list_search/components/mod.rs` (add `pub mod search_scope_picker;`)

**Interfaces:**
- Consumes: `SearchScopePicker` (empty props, self-sourcing) from B2.1.
- `UnitListSearch` keeps its Model (`value`, `placeholder`, `on_input`, `on_keydown`) — unchanged; it simply gains a self-sourcing child. (Note: the subtree now reads context, so `UnitListSearch` is no longer gallery-pure — acceptable, matches how `SearchFieldToggle` self-sourced.)

- [ ] **Step 1: Re-home the box to the container**

Edit `…/unit_list_search/style/mod.rs` — move the box classes here (from the input): `h-11`, `rounded-panel`, `border`, `border-warcraft-gold/45`, `bg-panel-dark`, `shadow-bevel`, plus a `kb-focus-within`/focus treatment if the input's focus ring should show on the box (TUNE). Keep `relative flex items-center gap-2 min-w-0 flex-none`. Result (starting point, TUNE):

```rust
use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "flex",
        "items-center",
        "gap-2",
        "min-w-0",
        "flex-none",
        "h-11",
        "rounded-panel",
        "border",
        "border-warcraft-gold/45",
        "bg-panel-dark",
        "shadow-bevel",
    ],
}
```

- [ ] **Step 2: Make the input borderless**

Edit `…/unit_list_search_input/style/mod.rs` — drop the box classes now owned by the container (`h-11`, `rounded-panel`, `border`, `border-warcraft-gold/45`, `bg-panel-dark`, `shadow-bevel`) and the `pl-10` if the icon moves; keep `flex-1 min-w-0 w-full text-base tracking-label text-white placeholder:italic placeholder:text-warcraft-text-primary/50` and add `bg-transparent border-0`. Keep/relocate the `kb-focus` treatment (TUNE — focus may now read better on the container). Keep `[&::-webkit-search-cancel-button]:appearance-none`.

- [ ] **Step 3: Render the scope picker + divider in the search bar**

Edit `…/unit_list_search/components/mod.rs` to add `pub mod search_scope_picker;`. Edit `…/unit_list_search/mod.rs`:

```rust
use components::search_scope_picker::SearchScopePicker;
// … existing imports …
rsx! {
    div {
        class: CLASS,
        SearchScopePicker {}
        UnitListSearchIcon {}
        UnitListSearchInput { value, placeholder, on_input, on_keydown }
    }
}
```

The divider between scope and input: add it as a container rule (e.g. a `[&>*+*]` left-border is wrong here since the icon is also a child) — instead give `SearchScopePicker`'s root a `border-r border-warcraft-gold/30 pr-2` (TUNE), so the divider belongs to the scope control. The magnifier icon is currently `absolute left-3`; with the scope picker occupying the left, either (a) keep the icon absolute but relative to an inner input wrapper, or (b) make it a static inline flex child sitting between the picker and the input. **Pick the simplest that looks right in the browser and TUNE** — favor (b) inline if it reads clean, dropping the input's `pl-10`.

- [ ] **Step 4: Regenerate CSS, reload, verify the whole flow in the browser**

`touch crates/hotkey-editor/tailwind.css`, reload, and confirm at **mobile / tablet / laptop / desktop / qhd / uhd**:
- The search bar reads as **one framed box** containing `Unit ▾ │ 🔍 Search units…` — scope on the left, divider, magnifier, input filling the rest.
- Clicking `Unit ▾` opens a listbox with **Unit / Ability**; picking one closes the menu, updates the trigger label, and **actually changes what the search filters** (type a query and confirm Ability-scope vs Unit-scope differ). Clicking outside (backdrop) closes it.
- Keyboard focus is visible; no layout collapse on mobile/tablet (remember the shell nukes `min-w`).
Expected: inline scope dropdown works and filters; single-box look at every band.

- [ ] **Step 5: Commit (only if the user authorizes)**

```bash
git add "crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_workspace/components/race_theme/components/shared/unit_list/components/unit_list_search"
git commit -m "feat(editor-controls): inline Unit/Ability scope dropdown in the search bar"
```

---

### Task B2.3: Remove `SearchFieldToggle` from the controls row

The controls row no longer needs the standalone Unit/Ability toggle.

**Files:**
- Modify: `…/unit_list/components/unit_controls/mod.rs` (drop the `SearchFieldToggle` import + render)

**Interfaces:**
- `UnitControls(tabs: Vec<UnitKind>)` signature unchanged; it now renders only `CatalogVisibilityToggle` + `MobileCategoryTabs`.

- [ ] **Step 1: Remove the render + import**

Edit `…/unit_controls/mod.rs` — delete `use super::search_field_toggle::SearchFieldToggle;` and the `SearchFieldToggle {}` line:

```rust
mod style;

use super::catalog_visibility_toggle::CatalogVisibilityToggle;
use super::mobile_category_tabs::MobileCategoryTabs;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
use warcraft_api::UnitKind;

#[component]
pub fn UnitControls(tabs: Vec<UnitKind>) -> Element {
    rsx! {
        div {
            class: CLASS,
            CatalogVisibilityToggle {}
            MobileCategoryTabs {
                tabs,
            }
        }
    }
}

assert_component!(UnitControls);
```

- [ ] **Step 2: Surface `search_field_toggle` as now-unused (do NOT delete yet)**

The `search_field_toggle/` component directory (and its `pub mod search_field_toggle;` in `…/unit_list/components/mod.rs`) is now rendered by nobody. Per "mention dead code, don't delete," **report it to the controller/owner for a decision** rather than deleting in this task. (Deleting it also means dropping its `pub mod` line; leaving it compiles fine as an unused-but-public module.)

- [ ] **Step 3: Regenerate CSS, reload, verify**

`touch crates/hotkey-editor/tailwind.css`, reload, confirm at mobile + laptop: the controls row no longer shows the standalone Unit/Ability buttons; the only place to change scope is now the `Unit ▾` dropdown in the search bar; No-abilities/All-variants and the category chips are unaffected.
Expected: Unit/Ability gone from the filters row; scope still changeable via the search dropdown.

- [ ] **Step 4: e2e coupling check**

`rg -n "search-field|SearchField|Unit|Ability" crates/hotkey-editor/e2e` — if any spec selected the old standalone toggle by class/structure, update it in the same commit; if it selects by role/text against the new dropdown, adjust accordingly. Report findings.

- [ ] **Step 5: Commit (only if the user authorizes)**

```bash
git add "crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_workspace/components/race_theme/components/shared/unit_list/components/unit_controls/mod.rs"
git commit -m "refactor(editor-controls): drop standalone Unit/Ability toggle (now inline in search)"
```

---

## Self-review

**Spec coverage (design §5 Phase B.2):**
- "inline `Unit ▾` dropdown mirroring HeroLevelPicker, self-sourcing search_field" → B2.1. ✓
- "re-home the box from the input to the search container; input borderless; divider" → B2.2. ✓
- "ARIA `haspopup=listbox`/`aria_expanded`/`role=listbox`/`role=option`" → B2.1 Step 3. ✓
- "delete search_field_toggle from unit_controls" → B2.3 (with the component dir surfaced, not deleted). ✓
- "dropdown open state is a UI signal; scope via use_editor_state" → B2.1 Step 2. ✓

**Placeholder scan:** No TBD/TODO. `TUNE` items (picker width, divider, icon placement, focus-ring home, menu offset) are genuine browser-tuned visual dimensions with concrete starting values and a stated acceptance ("reads as one framed box; dropdown filters correctly"). The dropdown scaffolding is specified by "mirror `hero_level_picker/<x>` renamed" rather than reproduced verbatim — that is the repo's mandated "copy the sibling line-for-line" convention (`docs/CLAUDE.md`), and the exact wiring (signal, setters, labels, ARIA) is given explicitly.

**Type consistency:** `SearchScopePicker` takes an empty `SearchScopePickerModel` (parameterless connected component per COMPONENTS.md). The scope handlers use `warcraft_api::SearchField` (`UnitName`/`Ability`) and `use_editor_state().search_field() -> Signal<SearchField>`, matching `SearchFieldToggle`'s existing wiring exactly. `UnitListSearch`'s Model is unchanged.

**Open items carried forward:**
- `search_field_toggle/` becomes dead after B2.3 — owner decides deletion (mention, don't delete).
- After later phases, re-confirm the shared `ToggleButton`'s remaining users (`catalog_visibility_toggle` still uses it until Phase C).
