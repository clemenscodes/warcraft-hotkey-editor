# Editor Controls — Phase D (Category Chips + ARIA fix) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Fix the category filter's wrong ARIA (independent multi-select mis-marked as tabs), rename the mis-named `mobile_category_tabs` → `category_filters`, and add a `Categories:` group label (sharing `filter_group_label` with the `Show:` group).

**Architecture:** The category controls are already separated, gapped, rounded chips that fill gold when shown / show a resting outline when hidden — the chosen affordance — and their `collapsed_categories: HashSet<UnitKind>` toggle wiring is correct. The only defects are (1) ARIA: the container is `<nav role="tablist">` and each chip is `<button role="tab" aria_selected>`, which lies — these are **independent multi-select toggles**, not tabs; (2) the name `mobile_category_tabs` (they render on all bands and aren't tabs). Phase D fixes the ARIA to `role="group"` + per-chip `<button aria_pressed>`, renames the tree, and adds the `Categories:` label by **promoting `filter_group_label` from `catalog_visibility_toggle/components/` to a `shared/` dir** and parameterizing it by `text` (so both `Show:` and `Categories:` reuse it). The visible look stays (TUNE height only). State wiring is unchanged.

**Tech Stack:** Rust, Dioxus 0.7, `tw_macro`, `ddd` pipeline, `warcraft_api::UnitKind`. No new deps.

**Scope note:** Phase D only. Design source: `docs/superpowers/specs/2026-07-15-editor-controls-affordances-redesign-design.md` §5 Phase D.

## Global Constraints

- **Verification is the dev server + browser — NEVER run `moon`/`cargo`/`dx` while it's up** (wedges the rebuild). After class edits, `touch crates/hotkey-editor/tailwind.css`, reload. Drive the running server with Playwright. No `moon run :ci`. No commit unless asked.
  - **`category_filters/` (renamed from `mobile_category_tabs/`) and `catalog_visibility_toggle/` are tracked → auto-rebuild.** **`unit_controls/` is UNTRACKED → its edits need the user to press `r`** in the dev server; tell the controller when a `unit_controls` change needs that.
- **ARIA:** multi-select category chips are `<button type="button" aria_pressed={is_shown}>` inside a container with `role="group"` + `aria_label="Categories"`. **No `role="tab"`, `role="tablist"`, or `aria_selected` anywhere in this tree** — that markup is the bug being removed.
- **`directory == component (PascalCase) == class (kebab)`**, `assert_component!` matches dir. Mutually-exclusive looks are components behind a `match` dispatcher (shown xor hidden). One classed root per component.
- **Render-tree == directory-tree:** children via `use components::…`; shared leaves (≥2 render sites) in a `shared/` dir at the common parent, reached by the full `super::shared::…` path.
- **`View → Model → Presentation`**, props private, data as named fields; no `Signal<T>` in View/Model/Presentation.
- **Styling:** tokens for color/type/radius/shadow; `base` no band prefix, bands only their own prefix. Bespoke geometry inline, `TUNE`.
- **RUST_STYLE:** full semantic names, `Self` in impls, derive standard traits.

Path prefix (abbreviated **`…/`**):
`…/editor_page/components/editor_workspace/components/race_theme/components/shared/unit_list/components/`

### Exact current state (verbatim, for precise edits)

`…/mobile_category_tabs/mod.rs` renders:
```rust
nav { class: CLASS, role: "tablist", aria_label: "Unit categories",
    for kind in props.tabs { MobileCategoryTab { key: "{unit_kind_key(kind)}", kind } } }
```
`…/mobile_category_tab/mod.rs` is a dispatcher: `use_mobile_category_tab(kind)` → `{is_active, model{label, onclick}}`, `match is_active { true => ActiveMobileCategoryTab{label,onclick}, false => IdleMobileCategoryTab{label,onclick} }`. Its `presentation/mod.rs` reads `use_editor_state().collapsed_categories()` (a `Signal<HashSet<UnitKind>>`), `is_active = !collapsed.contains(&kind)`, `onclick` inserts/removes `kind`, `label = kind.category_label()`.
`ActiveMobileCategoryTab/mod.rs`: `button { class: CLASS, role: "tab", r#type: "button", aria_selected: true, onclick, {label} }` — style `bg-panel-gold-active border-warcraft-gold text-warcraft-gold shadow-glow` + geometry `flex-1 min-w-0 min-h-14 px-2 border rounded-card text-sm tracking-label uppercase text-center …`. Idle is identical geometry with `bg-panel-gold-resting border-warcraft-gold-border text-warcraft-text-secondary hover:…`.
`…/catalog_visibility_toggle/components/filter_group_label/mod.rs`: `#[component] pub fn FilterGroupLabel() -> Element { let label = data::LABEL; rsx!{ span { class: CLASS, {label} } } }`, `data::LABEL = "Show:"`, style `text-sm uppercase tracking-caps text-warcraft-gold/80 whitespace-nowrap`.
`…/unit_controls/mod.rs` renders `CatalogVisibilityToggle {}` then `MobileCategoryTabs { tabs }`.
`…/unit_list/components/mod.rs` has `pub mod mobile_category_tabs;` and (Phase B) `pub mod unit_controls;` etc.

---

### Task D1: Promote `filter_group_label` to shared + parameterize by text

Do this first — it's a prerequisite for the `Categories:` label and it touches the (tracked, auto-rebuilding) `catalog` tree only.

**Files:**
- Create dir: `…/unit_list/components/shared/` with `mod.rs` = `pub mod filter_group_label;`
- Move (git mv): `…/catalog_visibility_toggle/components/filter_group_label/` → `…/unit_list/components/shared/filter_group_label/`
- Modify: the moved `filter_group_label/mod.rs` (take a `text` prop), add `model/`+`view/` (it now has a prop), delete its `data/mod.rs`
- Modify: `…/unit_list/components/mod.rs` (add `pub mod shared;`)
- Modify: `…/catalog_visibility_toggle/components/mod.rs` (remove `pub mod filter_group_label;`)
- Modify: `…/catalog_visibility_toggle/mod.rs` (import from `super::shared::filter_group_label`, pass `text`)
- Create: `…/catalog_visibility_toggle/data/mod.rs` (or reuse existing) with `pub(super) const SHOW_LABEL: &str = "Show:";`

- [ ] **Step 1: `git mv` the dir and re-point modules**

```bash
git mv "…/catalog_visibility_toggle/components/filter_group_label" "…/unit_list/components/shared/filter_group_label"
```
Create `…/unit_list/components/shared/mod.rs`:
```rust
pub mod filter_group_label;
```
In `…/unit_list/components/mod.rs` add `pub mod shared;` (keep the others). In `…/catalog_visibility_toggle/components/mod.rs` remove the `pub mod filter_group_label;` line.

- [ ] **Step 2: Parameterize `FilterGroupLabel` by `text`**

Rewrite the moved `filter_group_label/mod.rs`:
```rust
mod model;
mod view;
mod style;
use dioxus::prelude::*;
use model::FilterGroupLabelModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FilterGroupLabel(props: FilterGroupLabelModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(FilterGroupLabel);
```
Add `filter_group_label/view/mod.rs` (`pub struct FilterGroupLabelView { pub text: &'static str }` + `impl ddd::View`) and `filter_group_label/model/mod.rs` (`#[derive(Props, Clone, PartialEq)] pub struct FilterGroupLabelModel { pub text: &'static str }` + `From<&FilterGroupLabelView>` + `impl ddd::Model`). Mirror the `switch_item_label` model/view shape (it already takes a `text` field — read it as the template). Delete `filter_group_label/data/mod.rs` and its `mod data;`. Style unchanged.

- [ ] **Step 3: Update `catalog_visibility_toggle` to pass the text**

Create/extend `…/catalog_visibility_toggle/data/mod.rs`: `pub(super) const SHOW_LABEL: &str = "Show:";` (add `mod data;` to `catalog_visibility_toggle/mod.rs`). In `catalog_visibility_toggle/mod.rs` change the import to `use super::shared::filter_group_label::FilterGroupLabel;` and render `FilterGroupLabel { text: data::SHOW_LABEL }` (was `FilterGroupLabel {}`).

- [ ] **Step 4: Verify the `Show:` label still renders**

`touch crates/hotkey-editor/tailwind.css`, reload (catalog auto-rebuilds), confirm the switches row still shows `Show:` unchanged. If the app didn't rebuild, the dev-server output has a compiler error to fix. Expected: `Show:` renders exactly as before.

---

### Task D2: Rename `mobile_category_tabs` → `category_filters`, fix ARIA, add `Categories:` label

**Renames (git mv the dirs; rename components + `assert_component!` + view/model type names + class names):**
- `mobile_category_tabs/` → `category_filters/` (`MobileCategoryTabs`→`CategoryFilters`, class `category-filters`, `MobileCategoryTabsView`/`Model`→`CategoryFiltersView`/`Model`)
- `…/components/mobile_category_tab/` → `…/components/category_filter/` (`MobileCategoryTab`→`CategoryFilter`, `MobileCategoryTabModel`/`View`, `use_mobile_category_tab`→`use_category_filter`, `MobileCategoryTabDispatch`/`Inputs`/`Presentation`→`CategoryFilter…`)
- `…/active_mobile_category_tab/` → `…/shown_category_chip/` (`ActiveMobileCategoryTab`→`ShownCategoryChip`)
- `…/idle_mobile_category_tab/` → `…/hidden_category_chip/` (`IdleMobileCategoryTab`→`HiddenCategoryChip`)

**Files (after rename):**
- Modify: `category_filters/mod.rs` — container ARIA + `Categories:` label
- Modify: `category_filter/mod.rs` — render `ShownCategoryChip`/`HiddenCategoryChip`
- Modify: `shown_category_chip/mod.rs` + `hidden_category_chip/mod.rs` — ARIA (`aria_pressed`, drop `role="tab"`/`aria_selected`)
- Create: `category_filters/data/mod.rs` — `pub(super) const CATEGORIES_LABEL: &str = "Categories:";`
- Modify: `…/unit_list/components/mod.rs` — `pub mod mobile_category_tabs;` → `pub mod category_filters;`
- Modify: `…/unit_controls/mod.rs` — `MobileCategoryTabs`→`CategoryFilters` (UNTRACKED → controller presses `r`)

- [ ] **Step 1: git mv the four dirs, then rename every identifier**

```bash
cd crates/hotkey-editor/src/.../unit_list/components
git mv mobile_category_tabs category_filters
git mv category_filters/components/mobile_category_tab category_filters/components/category_filter
git mv category_filters/components/category_filter/components/active_mobile_category_tab category_filters/components/category_filter/components/shown_category_chip
git mv category_filters/components/category_filter/components/idle_mobile_category_tab category_filters/components/category_filter/components/hidden_category_chip
```
Then in every moved file rename the component fns, `assert_component!(…)`, `pub use view::…View`, the `…View`/`…Model` type names, the `use_mobile_category_tab`→`use_category_filter` and the presentation structs, and the `use components::…` / `pub mod …` lines to the new names. (The `key: "{unit_kind_key(kind)}"` and the `presentation/` wiring stay; only names change.)

- [ ] **Step 2: Fix the container ARIA + add the label — `category_filters/mod.rs`**

```rust
use components::category_filter::CategoryFilter;
use components::shared::filter_group_label::FilterGroupLabel;   // NOTE path — see below
// … unit_kind_key import stays …
rsx! {
    div {
        class: CLASS,
        role: "group",
        aria_label: "Categories",
        FilterGroupLabel { text: data::CATEGORIES_LABEL }
        for kind in props.tabs {
            CategoryFilter {
                key: "{unit_kind_key(kind)}",
                kind,
            }
        }
    }
}
```
The `nav` becomes a `div`; `role: "tablist"` → `role: "group"`; `aria_label: "Unit categories"` → `"Categories"`. **`FilterGroupLabel` path:** it lives at `…/unit_list/components/shared/filter_group_label/`, and `category_filters` is at `…/unit_list/components/category_filters/`, so from `category_filters/mod.rs` it is `use super::shared::filter_group_label::FilterGroupLabel;` (the sanctioned shared-leaf `super::shared::…` path — mirror how `catalog_visibility_toggle` reaches it after Task D1). Add `mod data;` and `…/category_filters/data/mod.rs` with `CATEGORIES_LABEL`.

- [ ] **Step 3: Fix the chip ARIA — `shown_category_chip/mod.rs` + `hidden_category_chip/mod.rs`**

```rust
// shown_category_chip/mod.rs (hidden is identical but aria_pressed:false)
rsx! {
    button {
        class: CLASS,
        r#type: "button",
        aria_pressed: true,
        onclick,
        {label}
    }
}
```
**Remove `role: "tab"` and `aria_selected`.** `ShownCategoryChip` = `aria_pressed: true`, `HiddenCategoryChip` = `aria_pressed: false`. Styles keep the shown=filled-gold / hidden=resting-outline look verbatim; **TUNE** the height (`min-h-14` is tall for a chip — try `min-h-9`/a `py-2`) so it reads as a chip, not a tab, but keep it a browser call.

- [ ] **Step 4: Point `unit_controls` at the renamed component**

`…/unit_controls/mod.rs`: `use super::category_filters::CategoryFilters;` and render `CategoryFilters { tabs }` (was `MobileCategoryTabs`). Update `…/unit_list/components/mod.rs`: `pub mod mobile_category_tabs;` → `pub mod category_filters;`. **`unit_controls/` is untracked — tell the controller a `r` rebuild is needed after this.**

- [ ] **Step 5: e2e coupling**

`rg -n "tablist|role=.tab|aria-selected|mobile-category|category-tab|Heroes|Workers" crates/hotkey-editor/e2e` — any spec selecting the categories by `role=tab`/`tablist`/`aria-selected`/the old class must switch to `role=group`/`getByRole("button", {pressed})`/the new class, in the same commit. Report findings.

- [ ] **Step 6: Regenerate CSS, `r`-rebuild, verify live**

`touch crates/hotkey-editor/tailwind.css`; the controller presses `r` (for the `unit_controls` change) then reloads. Verify at **mobile / tablet / laptop / desktop / qhd / uhd**:
- The category row shows `Categories:  [Heroes][Units][Workers][Buildings]` as separated chips, filled-gold when shown, resting-outline when hidden.
- **ARIA:** container is `role="group"` aria-label "Categories"; each chip is `role="button"`(default) with `aria-pressed` true/false; **no `role=tab`/`tablist`/`aria-selected` remain** (`document.querySelectorAll('[role=tab],[role=tablist],[aria-selected]').length === 0` inside this row).
- Each chip toggles **independently** (hiding "Workers" leaves the others shown and removes worker cards from the list; showing it again restores them); multiple can be hidden at once.
- Distinct from the switches and segments; keyboard focus visible.
Expected: correct multi-select chips + correct ARIA + `Categories:` label.

- [ ] **Step 7: Commit (only if the user authorizes)**

```bash
git add "…/unit_list/components/category_filters" "…/unit_list/components/shared" "…/unit_list/components/mod.rs" "…/catalog_visibility_toggle" "…/unit_controls/mod.rs" crates/hotkey-editor/e2e
git commit -m "feat(editor-controls): category multi-select chips + correct ARIA (group/aria-pressed) + Categories label"
```

---

## Self-review

**Spec coverage (design §5 Phase D):**
- "rename mobile_category_tabs→category_filters, MobileCategoryTab→CategoryFilter" → D2 Step 1. ✓
- "ARIA: role=tablist + role=tab/aria_selected → role=group + button aria_pressed" → D2 Steps 2–3. ✓
- "separated gapped rounded chips, filled=shown/outline=hidden; dispatcher shown/hidden" → look kept (already correct); dispatcher renamed Shown/Hidden. ✓
- "Categories: label via shared filter_group_label (text prop)" → D1 (promote+parameterize) + D2 Step 2. ✓
- "collapsed_categories HashSet toggle unchanged" → `presentation/` wiring untouched by the rename. ✓

**Placeholder scan:** No TBD. `TUNE` = chip height (currently tab-tall) dialed in-browser. The rename is mechanical (git mv + identifier substitution); the ARIA change is exact attribute swaps.

**Type consistency:** `FilterGroupLabel` now takes `FilterGroupLabelModel { text: &'static str }` (mirrors `switch_item_label`); both `catalog_visibility_toggle` (`SHOW_LABEL`) and `category_filters` (`CATEGORIES_LABEL`) pass `text`, reached via `super::shared::filter_group_label`. `CategoryFilters` keeps the `tabs: Vec<UnitKind>` prop; `CategoryFilter` keeps `kind`. `use_category_filter` returns the same `{is_shown, label, onclick}` shape (renamed from `is_active`).

**Open items:**
- After this, `filter_group_label` has 2 render sites (catalog + categories) → correctly in `shared/`.
- The shared `ToggleButton` (dead since Phase C) and `search_field_toggle/` (dead since B.2) remain surfaced-not-deleted for the owner.
