# Editor Controls — Phase C (Independent Switches) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Reskin the two independent booleans — **No abilities** and **All variants** — from identical gold toggle buttons into **track+knob switches**, add a `Show:` group microlabel, and stop using the shared `ToggleButton` for them.

**Architecture:** A new `SwitchToggle` (a `role="switch"` control drawn as a rounded track with a sliding knob) replaces `ToggleButton` behind the existing `CatalogVisibilityButton` seam. Because on/off are a UI state, they are **two components** (`OnSwitchToggle`/`OffSwitchToggle`) behind a dispatcher — never a class table — each drawing its own track color + knob position, sharing the knob leaf. The state wiring is unchanged (reads `use_editor_state().show_abilityless_units()` / `.expand_variants()`, sets `!current`). A `Show:` label leaf is added ahead of the two switches. The shared `ToggleButton` is not touched (still used by nobody after this — see the dead-code note).

**Tech Stack:** Rust, Dioxus 0.7, `tw_macro`, the `ddd` `View → Model → Presentation` pipeline. No new deps.

**Scope note:** Phase C only. Design source: `docs/superpowers/specs/2026-07-15-editor-controls-affordances-redesign-design.md` §5 Phase C.

## Global Constraints

- **Verification is the running dev server + browser — NEVER run `moon`/`cargo`/`dx` while the dev server is up** (it fights the cargo lock and wedges the rebuild). The dev server auto-compiles on save. After class edits, `touch crates/hotkey-editor/tailwind.css` then reload `http://localhost:8123/warcraft-hotkey-editor/`. Do not start the server; drive the running one with Playwright. Do not run `moon run :ci`. Do not commit unless asked.
  - **`catalog_visibility_toggle/` is a tracked dir, so its edits DO auto-rebuild** (unlike `unit_controls/`). New files under it also rebuild.
- **Mutually-exclusive looks are components behind a dispatcher** (`match is_on`), never a state-keyed class table. `on`/`off` switches are two components.
- **`directory == component (PascalCase) == class (kebab)`**, `assert_component!` matches dir. One classed root per component.
- **Render-tree == directory-tree:** rendered children via `use components::…`; shared leaves (≥2 render sites) in a `shared/` dir at the common parent, reached by full path. A `super::`-imported rendered child is a violation.
- **`View → Model → Presentation`:** leaves carry `view/`+`model/`(`#[derive(Props)]`,`From<&View>`)+`mod.rs`; dispatchers own no class; no `Signal<T>` in any View/Model/Presentation.
- **Props private; data as named fields** (no `..spread` of another component's props, no `impl From<&ParentProps>`). Content/labels are props sourced from a `data.rs`.
- **Styling:** `base` no band prefix, bands only their own prefix; tokens for color/type/radius/shadow; bespoke geometry (track/knob sizes) is arbitrary `[…]` and inline, marked `TUNE`.
- **Accessibility:** each switch is a `<button type="button" role="switch" aria_checked={is_on}>`; it needs an accessible name — provide `aria_label` (the item text) since the visible text label sits beside it. Keep the existing `title` tooltip on the "No abilities" switch.

Path prefix (abbreviated **`…/catalog/`**):
`…/editor_page/components/editor_workspace/components/race_theme/components/shared/unit_list/components/catalog_visibility_toggle/`

Siblings to read before writing:
- `ToggleButton` dispatcher + `active_toggle_button`/`idle_toggle_button` (`…/editor_page/components/shared/toggle_button/`) — the `match`-dispatcher + Active/Idle leaf shapes to mirror.
- `mobile_category_tab` Active/Idle (`…/unit_list/components/mobile_category_tabs/components/mobile_category_tab/`) — another Active/Idle own-markup example.
- `race_tab` shared leaf under `shared/` (`…/race_tabs_host/…/shared/race_tab_state/components/shared/race_tab/`) — the shared-leaf-in-`shared/` placement, reached by full path.
- Current `…/catalog/mod.rs`, `presentation/mod.rs`, `style/mod.rs`, `components/catalog_visibility_button/mod.rs` (+ its `model/`,`view/`).

---

### Task C1: Build `SwitchToggle` + `FilterGroupLabel` + `SwitchItemLabel`, and rewire `catalog_visibility_toggle`

One cohesive task (the new leaves aren't visible until mounted, and `catalog_visibility_toggle/` auto-rebuilds, so verify the whole thing live at the end).

**New component tree (create):**
```
…/catalog/components/
  filter_group_label/            leaf: the "Show:" group label. mod+view+model+style+data (const LABEL="Show:")
  catalog_visibility_switch/     (RENAME of catalog_visibility_button) — per-item wrapper: renders label + switch
    mod.rs                       div.CLASS { SwitchItemLabel { text: label }  SwitchToggle { is_on: active, title, aria_label: label, onclick } }
    model/ view/                 (keep the existing {label,is_active,title,on_toggle} fields; add nothing)
    style/mod.rs                 the item row: flex items-center gap (TUNE)
    components/
      switch_item_label/         leaf: the "No abilities"/"All variants" text. mod+view+model+style
      switch_toggle/             DISPATCHER (no class): match is_on { true => OnSwitchToggle, false => OffSwitchToggle }
        mod.rs view/ model/      Model {label? no — label is beside; fields: is_on, title: Option<&'static str>, aria_label, onclick}
        components/
          on_switch_toggle/      button role=switch aria_checked=true; own markup; gold track, knob right. mod+view+model+style
          off_switch_toggle/     button role=switch aria_checked=false; resting track, knob left. mod+view+model+style
          shared/
            switch_knob/         leaf: the knob circle. mod+view+model+style (rendered by on+off → 2 sites → shared/)
```

**Interfaces:**
- Consumes: `use_editor_state().show_abilityless_units()`/`.expand_variants()` → `Signal<bool>` (unchanged wiring in `catalog/presentation/mod.rs`).
- `SwitchToggle` Model fields: `is_on: bool`, `title: Option<&'static str>`, `aria_label: &'static str`, `onclick: EventHandler<MouseEvent>`. (No `label` — the visible label is the sibling `SwitchItemLabel`; `aria_label` is the a11y name.)
- Class names: `filter-group-label`, `catalog-visibility-switch`, `switch-item-label`, `switch-toggle`(dispatcher, no class), `on-switch-toggle`, `off-switch-toggle`, `switch-knob`.

- [ ] **Step 1: Read the sibling templates**

Read `ToggleButton`'s dispatcher (`toggle_button/mod.rs`) + `active_toggle_button/mod.rs` + `idle_toggle_button/mod.rs` (+ their `model/`,`view/`,`style/`) — mirror the `match`-dispatcher and the Active/Idle full-`<button>`-with-own-style shape. Read the current `catalog/components/catalog_visibility_button/{mod,model,view}.rs` (its props are `{label, is_active, title: &'static str, on_toggle}`). Read `race_tab`'s `shared/` placement for the shared-leaf path form.

- [ ] **Step 2: Build the shared knob leaf**

`…/switch_toggle/components/shared/switch_knob/` — a plain leaf: `div { class: CLASS }` (empty). `style/mod.rs`: a circle — `rounded-full`, a bespoke size (`w-[1.1rem] h-[1.1rem]` TUNE), `bg-warcraft-gold` (the knob is gold on both states; the TRACK color differs), a subtle shadow token if one fits. No `components/`. `view/`+`model/` empty (passthrough leaf, no props) OR a fieldless Model — match how a no-prop leaf is written elsewhere (e.g. `UnitListSearchIcon` takes no props and has just `mod.rs`+`style/`); prefer that minimal shape (no view/model) since the knob has no data.

- [ ] **Step 3: Build `on_switch_toggle` and `off_switch_toggle`**

Each is a full `<button>`, mirroring the Active/Idle toggle-button shape but drawing a track:
```rust
// on_switch_toggle/mod.rs (off is the same shape, aria_checked:false, off style, knob at start)
#[component]
pub fn OnSwitchToggle(props: OnSwitchToggleModel) -> Element {
    let title = props.title;
    let aria_label = props.aria_label;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            role: "switch",
            aria_checked: true,
            aria_label,
            title,
            onclick,
            SwitchKnob {}
        }
    }
}
```
`on_switch_toggle/style/mod.rs` (TUNE all geometry): the button IS the track — `inline-flex items-center` + `justify-end` (knob right) + a bespoke track size (`w-[2.2rem] h-[1.25rem]` TUNE) + `rounded-full` + `p-[0.15rem]` + track color **on** (`bg-warcraft-gold/70` or a gold token — TUNE) + border token + `cursor-pointer` + `transition-[background-color] duration-base` + focus (`focus:outline-none kb-focus:shadow-focus` per the repo idiom). `off_switch_toggle/style` = same geometry, `justify-start`, resting track (`bg-warcraft-bg-base/70` + muted border — TUNE). Reach the knob via `use super::shared::switch_knob::SwitchKnob;` from each track's parent... **note the path:** `SwitchKnob` lives at `switch_toggle/components/shared/switch_knob/`, and `on_switch_toggle` is at `switch_toggle/components/on_switch_toggle/`, so from `on_switch_toggle/mod.rs` it is `use super::shared::switch_knob::SwitchKnob;` — wait: `super` from `on_switch_toggle` is `switch_toggle/components/`, so `super::shared::switch_knob::SwitchKnob` is correct (the sanctioned shared-leaf path). Confirm against the `race_tab` example.

- [ ] **Step 4: Build the `switch_toggle` dispatcher**

`switch_toggle/mod.rs` — no class, `match props.is_on { true => rsx!{ OnSwitchToggle{ title, aria_label, onclick } }, false => rsx!{ OffSwitchToggle{ title, aria_label, onclick } } }`. Model `{ is_on, title: Option<&'static str>, aria_label: &'static str, onclick }`. `components/mod.rs`: `pub mod on_switch_toggle; pub mod off_switch_toggle; pub mod shared;` and `shared/mod.rs`: `pub mod switch_knob;`.

- [ ] **Step 5: Build `switch_item_label` and `filter_group_label`**

`switch_item_label/` — leaf: `span { class: CLASS, {text} }`, Model `{ text: &'static str }`, style a small readable label (`text-md`/`text-sm` token + `text-warcraft-text-secondary` + `whitespace-nowrap` — TUNE). `filter_group_label/` — same shape, its `data/mod.rs` holds `pub(super) const LABEL: &str = "Show:";`, style slightly emphasized (`text-warcraft-gold/80` or `uppercase tracking-caps` — TUNE). (Both are single-render-site leaves nested under their renderer for Phase C; Phase D may promote `filter_group_label` to a `shared/` when the categories group reuses it — leave that to Phase D.)

- [ ] **Step 6: Rewrite `catalog_visibility_switch` (was `catalog_visibility_button`) to render label + switch**

Rename the directory `catalog_visibility_button` → `catalog_visibility_switch`, component `CatalogVisibilityButton` → `CatalogVisibilitySwitch`, update `assert_component!` and `…/catalog/components/mod.rs` (`pub mod catalog_visibility_switch;`). Its `mod.rs`:
```rust
#[component]
pub fn CatalogVisibilitySwitch(props: CatalogVisibilitySwitchModel) -> Element {
    let label = props.label;
    let is_on = props.is_active;
    let title = Some(props.title);
    let onclick = props.on_toggle;
    rsx! {
        div {
            class: CLASS,
            SwitchItemLabel { text: label }
            SwitchToggle { is_on, title, aria_label: label, onclick }
        }
    }
}
```
Keep the Model/View field names (`label`, `is_active`, `title`, `on_toggle`). Drop the `ToggleButton` import. `style/mod.rs`: item row `flex items-center gap-2` (TUNE).

- [ ] **Step 7: Update `catalog_visibility_toggle/mod.rs` — add the `Show:` label, render the switches**

Render `FilterGroupLabel {}` first, then the two `CatalogVisibilitySwitch` (unchanged props). Fix the container `role`/`aria_label` to reflect "these are independent switches" — keep `role: "group"`, change `aria_label` to `"Show"` (or keep "Catalog visibility"; TUNE the wording). Update the import to `use components::catalog_visibility_switch::CatalogVisibilitySwitch;` and add `use components::filter_group_label::FilterGroupLabel;`.

- [ ] **Step 8: Restyle the container**

`…/catalog/style/mod.rs`: it currently has `flex flex-row flex-[2_1_0] gap-2 [&>button]:min-h-14!` + mobile/tablet widths. The `[&>button]:min-h-14!` was for the old full-height toggle buttons; the switches are small — **drop it** (TUNE). Keep it a `flex flex-row items-center gap-3` row (label + two switch items). Re-TUNE the mobile/tablet widths for the new content.

- [ ] **Step 9: Let the dev server rebuild; regenerate CSS; verify live**

Do NOT run any build. `touch crates/hotkey-editor/tailwind.css`, reload, and verify at **mobile / tablet / laptop / desktop / qhd / uhd**:
- The catalog row reads `Show:  No abilities [switch]   All variants [switch]` — two track+knob switches, each with its text label; visually a **switch**, clearly different from the connected mode segments and the (still-button) category filters.
- Each switch toggles **independently**: clicking "No abilities" flips only its knob/track (on↔off) and changes the unit list (abilityless units appear/disappear); clicking "All variants" flips only its own and expands/collapses variants. Neither affects the other (independent booleans).
- The knob sits right/gold-track when on, left/resting when off. Keyboard focus is visible; the "No abilities" tooltip still shows on hover.
Expected: two independent switches, correct on/off visuals, correct filtering, distinct affordance.

- [ ] **Step 10: e2e coupling check**

`rg -n "catalog-visibility|No abilities|All variants|toggle-button" crates/hotkey-editor/e2e` — if any spec selected these by the old class/structure, update in the same commit; report findings.

- [ ] **Step 11: Commit (only if the user authorizes)**

```bash
git add "…/catalog_visibility_toggle"
git commit -m "feat(editor-controls): No-abilities/All-variants as independent switches + Show: label"
```

---

## Self-review

**Spec coverage (design §5 Phase C):**
- "two independent booleans → track+knob switches" → C1 (SwitchToggle on/off components). ✓
- "keep the existing Signal::set(!current) handlers + the No-abilities tooltip" → wiring unchanged; `title` threaded. ✓
- "Show: microlabel via a shared filter_group_label leaf (text as prop)" → C1 Step 5 (nested for Phase C; promote to `shared/` in D). ✓
- "on/off are the dispatcher-of-two-looks pattern, no class swap" → switch_toggle dispatches OnSwitchToggle/OffSwitchToggle. ✓
- "distinct affordance from segmented/chips" → track+knob shape. ✓

**Placeholder scan:** No TBD. `TUNE` = bespoke switch geometry (track/knob sizes, colors, gaps, container widths, aria wording) dialed in-browser — genuine visual dimensions with concrete starting values + a clear acceptance ("reads as a switch; toggles independently; correct filtering").

**Type consistency:** `SwitchToggle` Model `{is_on, title: Option<&'static str>, aria_label, onclick}`; `CatalogVisibilitySwitch` still receives `{label, is_active, title: &'static str, on_toggle}` (unchanged) and maps `is_active→is_on`, `label→aria_label`+`SwitchItemLabel.text`, `title→Some(title)`, `on_toggle→onclick`. `catalog/presentation/mod.rs` is unchanged. `OnSwitchToggle`/`OffSwitchToggle` share the same Model field set (minus `is_on`, already matched).

**Open items:**
- After Phase C, the shared `ToggleButton` has lost its last renderer (mode→segments in A, scope→dropdown in B.2, catalog→switches in C; categories never used it). It is now **dead** — surface for deletion (owner decides), don't delete here. Same for `search_field_toggle/` (already surfaced in B.2).
- `filter_group_label` is nested under `catalog` for now; Phase D promotes it to a `shared/` when the categories group adds its own group label.
