# Editor Controls — Phase A (Mode Segmented Control) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn the Melee/Campaign mode selector into a **connected segmented control**, move it out of the unit-filters row into the race-tabs section (killing a render-tree==dir-tree violation), and slim the race badges.

**Architecture:** Mode currently renders the shared `ToggleButton` (two gapped gold pills) and is mounted cross-subtree inside `UnitControls`. Phase A gives mode its own **connected** look (two borderless segments inside one bordered, overflow-clipped pill with a divider), rehomes it under `EditorTabsBar` beside the race badges, and shortens the race carousel. State/handlers are untouched — only presentation + placement change. `ToggleButton` is **not** modified (it still serves search-scope and catalog toggles until Phases B.2/C).

**Tech Stack:** Rust, Dioxus 0.7, the `tw_macro` styling engine (`classes!`/`tw!`/`assert_component!`), the `ddd` `View → Model → Presentation` pipeline. No new dependencies.

**Scope note:** This plan covers **Phase A only**. Phases B.2 (inline search scope), C (switches), D (chips + ARIA), E (layout/polish) get their own just-in-time plans authored against the post-Phase-A tree, per `docs/ARCHITECTURE.md §6`. Design source: `docs/superpowers/specs/2026-07-15-editor-controls-affordances-redesign-design.md`.

## Global Constraints

Copied verbatim from the design spec / project docs; every task implicitly includes these:

- **Verification is `moon run :check` + the browser — NEVER `moon run :ci`** (the owner runs that). After any class edit, `touch crates/hotkey-editor/tailwind.css` (the watcher is inotify-blind to new classes), then reload. There is **no renderer unit-test suite**; a task is verified when `:check` compiles clean and the change looks right in the browser at all six bands.
- **Do not start the dev server** (the owner owns port 8123). Drive the **already-running** server with the Playwright MCP at `http://localhost:8123/warcraft-hotkey-editor/` (trailing slash — bare `/` is 404).
- **Do not commit unless the owner asks.** Commits are GPG-signed; a signed `git commit` is required (no `--no-gpg-sign`). The commit blocks in each task are written for when the owner authorizes committing.
- **Six responsive bands** (disjoint, nothing inherits across them): `mobile <768`, `tablet 768–1279`, `laptop 1280–1919`, `desktop 1920–2559`, `qhd 2560–3839`, `uhd ≥3840`; `base` is always-on. A `base` class must carry no band prefix; a band array must carry only its own prefix (macro-enforced).
- **`directory == component (PascalCase) == class (kebab)`**, enforced by `assert_component!`. One classed element per component. Mutually-exclusive looks are **separate components** behind a `match` dispatcher — never a state-keyed class table.
- **`@container` collapses under `w-auto`/`w-fit`** — a `@container` element needs a **definite width** (from flex in a definite-width parent, or an explicit `w-[…]`).
- **Shell nukes `min-w` on mobile/tablet** (`mobile:**:min-w-0` + `tablet:**:min-w-0`); size with `w-[…]`/`flex-basis` + `flex-none`, never `min-w`, on those bands.
- **`[…]` starting values marked `TUNE`** are dialed in the browser pass; the exact px/rem is a visual decision, not a fixed requirement.
- **Do NOT restyle the shared `ToggleButton`** (`editor_page/components/shared/toggle_button`). Phase A builds new leaves instead.

Path prefix used below (abbreviated **`…/editor_page/`**):
`crates/hotkey-editor/src/components/app/components/shell/components/editor_page/`

---

### Task A1: Connected mode segments + `ModeTab` dispatcher + connected container

Give mode its own connected look **in place** (still mounted in `UnitControls` at this point, so it's browser-verifiable immediately). Two new leaves mirror `ActiveToggleButton`/`IdleToggleButton` **minus** the per-segment border/rounding/`title` (the container owns the frame). `ModeTab` becomes a `match` dispatcher; `ModeTabs` becomes the bordered, overflow-clipped pill.

**Files:**
- Create: `…/editor_page/components/editor_tabs_bar/components/mode_tabs_host/components/mode_tabs/components/mode_tab/components/mod.rs`
- Create leaf `active_mode_segment/`: `mod.rs`, `view/mod.rs`, `model/mod.rs`, `style/mod.rs` under `…/mode_tab/components/active_mode_segment/`
- Create leaf `idle_mode_segment/`: `mod.rs`, `view/mod.rs`, `model/mod.rs`, `style/mod.rs` under `…/mode_tab/components/idle_mode_segment/`
- Modify: `…/mode_tab/mod.rs` (leaf → dispatcher)
- Modify: `…/mode_tabs/style/mod.rs` (gapped → connected pill)

**Interfaces:**
- Consumes: `ModeTabModel { label: &'static str, active: bool, onclick: EventHandler<MouseEvent>, onkeydown: EventHandler<KeyboardEvent> }` (unchanged, in `…/mode_tab/model/mod.rs`).
- Produces: `ActiveModeSegment` / `IdleModeSegment`, each taking a Model with fields `{ label: &'static str, onclick: EventHandler<MouseEvent>, onkeydown: EventHandler<KeyboardEvent> }` (no `active`, no `title`). Class names: `active-mode-segment`, `idle-mode-segment`.

- [ ] **Step 1: Create the children module file**

Create `…/mode_tab/components/mod.rs`:

```rust
pub mod active_mode_segment;
pub mod idle_mode_segment;
```

- [ ] **Step 2: Create `active_mode_segment/view/mod.rs`**

```rust
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveModeSegmentView {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for ActiveModeSegmentView {}
```

- [ ] **Step 3: Create `active_mode_segment/model/mod.rs`**

```rust
use super::view::ActiveModeSegmentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveModeSegmentModel {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&ActiveModeSegmentView> for ActiveModeSegmentModel {
    fn from(view: &ActiveModeSegmentView) -> Self {
        let ActiveModeSegmentView {
            label,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            label,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Model for ActiveModeSegmentModel {
    type View = ActiveModeSegmentView;
}
```

- [ ] **Step 4: Create `active_mode_segment/style/mod.rs`**

Mirrors `active_toggle_button/style` but **drops** `border`, `rounded-panel`, `shadow-glow` (the `ModeTabs` container owns the frame; `shadow-glow` would be clipped by the container's `overflow-hidden`). Keeps the gold-active fill + type treatment + band steps verbatim.

```rust
use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex-1",
        "min-w-0",
        "px-6",
        "text-xl",
        "uppercase",
        "tracking-caps",
        "whitespace-nowrap",
        "text-shadow-drop",
        "cursor-pointer",
        "transition-[color,box-shadow]",
        "duration-base",
        "bg-panel-gold-active",
        "text-warcraft-gold",
        "focus:outline-none",
        "kb-focus:text-white",
        "kb-focus:shadow-focus",
    ],
    mobile: tw![
        "mobile:text-base",
        "mobile:px-2.5",
    ],
    tablet: tw![
        "tablet:text-md",
        "tablet:px-4",
    ],
    laptop: tw![
        "laptop:text-md",
        "laptop:px-4",
    ],
    desktop: tw![
        "desktop:text-md",
        "desktop:px-4",
    ],
}
```

- [ ] **Step 5: Create `active_mode_segment/mod.rs`**

```rust
mod model;
mod view;

pub use view::ActiveModeSegmentView;
mod style;

use dioxus::prelude::*;
use model::ActiveModeSegmentModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveModeSegment(props: ActiveModeSegmentModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_pressed: true,
            onclick,
            onkeydown,
            {label}
        }
    }
}

assert_component!(ActiveModeSegment);
```

- [ ] **Step 6: Create `idle_mode_segment/view/mod.rs`**

```rust
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IdleModeSegmentView {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for IdleModeSegmentView {}
```

- [ ] **Step 7: Create `idle_mode_segment/model/mod.rs`**

```rust
use super::view::IdleModeSegmentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IdleModeSegmentModel {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&IdleModeSegmentView> for IdleModeSegmentModel {
    fn from(view: &IdleModeSegmentView) -> Self {
        let IdleModeSegmentView {
            label,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            label,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Model for IdleModeSegmentModel {
    type View = IdleModeSegmentView;
}
```

- [ ] **Step 8: Create `idle_mode_segment/style/mod.rs`**

Mirrors `idle_toggle_button/style` but **drops** `border`, `border-warcraft-gold-border`, `rounded-panel` (container owns the frame); keeps the resting fill + hover + type + band steps.

```rust
use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex-1",
        "min-w-0",
        "px-6",
        "bg-panel-gold-resting",
        "text-warcraft-text-secondary",
        "text-xl",
        "uppercase",
        "tracking-caps",
        "whitespace-nowrap",
        "text-shadow-drop",
        "cursor-pointer",
        "transition-[color,box-shadow]",
        "duration-base",
        "hover:text-warcraft-gold",
        "focus:outline-none",
        "kb-focus:text-white",
        "kb-focus:shadow-focus",
    ],
    mobile: tw![
        "mobile:text-base",
        "mobile:px-2.5",
    ],
    tablet: tw![
        "tablet:text-md",
        "tablet:px-4",
    ],
    laptop: tw![
        "laptop:text-md",
        "laptop:px-4",
    ],
    desktop: tw![
        "desktop:text-md",
        "desktop:px-4",
    ],
}
```

- [ ] **Step 9: Create `idle_mode_segment/mod.rs`**

```rust
mod model;
mod view;

pub use view::IdleModeSegmentView;
mod style;

use dioxus::prelude::*;
use model::IdleModeSegmentModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IdleModeSegment(props: IdleModeSegmentModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_pressed: false,
            onclick,
            onkeydown,
            {label}
        }
    }
}

assert_component!(IdleModeSegment);
```

- [ ] **Step 10: Rewrite `…/mode_tab/mod.rs` as a dispatcher**

Replace the whole file. It now owns `components`, renders no class of its own (a dispatcher, exactly like `ToggleButton`), and drops the cross-subtree `ToggleButton` import.

```rust
mod model;
mod view;

pub use view::ModeTabView;
pub mod components;

use components::active_mode_segment::ActiveModeSegment;
use components::idle_mode_segment::IdleModeSegment;
use dioxus::prelude::*;
use model::ModeTabModel;
use tw_macro::assert_component;

#[component]
pub fn ModeTab(props: ModeTabModel) -> Element {
    let label = props.label;
    let active = props.active;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    match active {
        true => rsx! {
            ActiveModeSegment {
                label,
                onclick,
                onkeydown,
            }
        },
        false => rsx! {
            IdleModeSegment {
                label,
                onclick,
                onkeydown,
            }
        },
    }
}

assert_component!(ModeTab);
```

- [ ] **Step 11: Restyle `…/mode_tabs/style/mod.rs` into a connected pill**

Replace the whole file. Drops `gap-2`; adds the shared outer frame (`rounded-panel border overflow-hidden`) and a divider before every non-first child.

```rust
use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex",
        "flex-row",
        "min-w-0",
        "h-14",
        "rounded-panel",
        "border",
        "border-warcraft-gold-border",
        "overflow-hidden",
        "[&>*+*]:border-l",
        "[&>*+*]:border-warcraft-gold-border",
    ],
}
```

- [ ] **Step 12: Compile-check**

Run: `moon run :check`
Expected: `Finished` with no errors (a clean `hotkey-editor:rust/check`). If a class name is rejected by the macro or an `assert_component!` name mismatches its directory, fix before proceeding.

- [ ] **Step 13: Regenerate CSS and verify in the browser**

Run: `touch crates/hotkey-editor/tailwind.css`
Then, via the Playwright MCP against `http://localhost:8123/warcraft-hotkey-editor/` (reload after the touch), confirm at **mobile (390), laptop (1440), and uhd (3840)**:
- The Melee/Campaign control (still in the filters row for now) reads as **one connected pill**: outer rounded border, a single divider between the two segments, no gap.
- The active segment is filled gold; the idle segment is resting; hover on idle turns text gold; keyboard focus is visible.
Expected: connected segmented look at every band; no console errors.

- [ ] **Step 14: Commit (only if the owner authorizes)**

```bash
git add "crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_tabs_bar/components/mode_tabs_host/components/mode_tabs"
git commit -m "feat(editor-controls): mode selector as a connected segmented control"
```

---

### Task A2: Move mode into the race section; remove the cross-subtree debt

`ModeTabsHost` is defined under `editor_tabs_bar` but mounted inside `UnitControls` via a `crate::…::editor_tabs_bar::…::ModeTabsHost` import — a render-tree==dir-tree violation. Move the mount to `EditorTabsBar` (its real parent), delete the import from `UnitControls`, and restyle `ModeTabsHost` for its new stacked home.

**Files:**
- Modify: `…/editor_page/components/editor_tabs_bar/mod.rs`
- Modify: `…/editor_page/components/editor_tabs_bar/components/mode_tabs_host/style/mod.rs`
- Modify: `…/editor_page/components/editor_workspace/components/race_theme/components/shared/unit_list/components/unit_controls/mod.rs`

**Interfaces:**
- Consumes: `ModeTabsHost` (no props; sources its own state), `RaceTabsHost` (no props). `EditorTabsBar` root is `flex flex-col gap-2.5` (unchanged).
- Produces: `UnitControls` no longer renders `ModeTabsHost` (unchanged signature `UnitControls(tabs: Vec<UnitKind>)`).

- [ ] **Step 1: Render `ModeTabsHost` inside `EditorTabsBar`**

Replace `…/editor_tabs_bar/mod.rs`. `ModeTabsHost` is placed **above** `RaceTabsHost` (TUNE: above vs below the badges is a browser call in Step 5).

```rust
pub mod components;
mod model;
mod style;
mod view;

pub use view::EditorTabsBarView;

use components::mode_tabs_host::ModeTabsHost;
use components::race_tabs_host::RaceTabsHost;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn EditorTabsBar() -> Element {
    rsx! {
        div {
            class: CLASS,
            ModeTabsHost {}
            RaceTabsHost {}
        }
    }
}

assert_component!(EditorTabsBar);
```

- [ ] **Step 2: Restyle `ModeTabsHost` for the stacked race-section home**

Replace `…/mode_tabs_host/style/mod.rs`. The old `flex-[2_1_0]` + `mobile/tablet` fixed widths were sized for the `UnitControls` flex-row; in the flex-col `EditorTabsBar` the control should be a compact, left-aligned block with a **definite width** (`@container` collapses under `w-auto`). Values marked TUNE.

```rust
use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex-none",
        "w-[16rem]",
    ],
    mobile: tw![
        "mobile:w-full",
    ],
    tablet: tw![
        "tablet:w-full",
    ],
}
```

- [ ] **Step 3: Remove `ModeTabsHost` from `UnitControls`**

Replace `…/unit_controls/mod.rs` — delete the cross-subtree `use` line and the `ModeTabsHost {}` render.

```rust
mod style;

use super::catalog_visibility_toggle::CatalogVisibilityToggle;
use super::mobile_category_tabs::MobileCategoryTabs;
use super::search_field_toggle::SearchFieldToggle;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
use warcraft_api::UnitKind;

#[component]
pub fn UnitControls(tabs: Vec<UnitKind>) -> Element {
    rsx! {
        div {
            class: CLASS,
            SearchFieldToggle {}
            CatalogVisibilityToggle {}
            MobileCategoryTabs {
                tabs,
            }
        }
    }
}

assert_component!(UnitControls);
```

- [ ] **Step 4: Compile-check**

Run: `moon run :check`
Expected: `Finished` with no errors. (If the compiler flags `ModeTabsHost` as unused anywhere else, there should be none — it was only ever mounted in `UnitControls`.)

- [ ] **Step 5: Regenerate CSS and verify placement in the browser**

Run: `touch crates/hotkey-editor/tailwind.css`, reload, and confirm at **mobile / tablet / laptop / desktop / qhd / uhd**:
- The mode segmented control now sits in the **race-tabs section** (with the badges), and is **gone** from the unit-filters row (which now starts with Unit/Ability).
- The control is left-aligned and compact on wide bands, full-width on mobile/tablet, and not collapsed (definite width holds).
- TUNE here: the `w-[16rem]` width and the above/below-badges order — adjust in this pass until it "looks good and still reads as a peer of the race selector" (handoff §1).
Expected: mode grouped with race; filters row no longer contains mode; no layout collapse.

- [ ] **Step 6: Check e2e coupling before finalizing**

Run: `rg -n "mode-tab|ModeTabs|Melee|Campaign|active-toggle-button" crates/hotkey-editor/e2e`
If any spec selects the mode control by a class/structure this task changed, update that spec in the **same commit**. If it selects by role/text (`button`, "Melee"/"Campaign"), no change is needed.
Expected: either no hits, or a known set of selectors updated alongside.

- [ ] **Step 7: Commit (only if the owner authorizes)**

```bash
git add "crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_tabs_bar/mod.rs" \
        "crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_tabs_bar/components/mode_tabs_host/style/mod.rs" \
        "crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_workspace/components/race_theme/components/shared/unit_list/components/unit_controls/mod.rs"
git commit -m "refactor(editor-controls): move mode selector into the race section, drop cross-subtree import"
```

---

### Task A3: Slim the race badges

The race carousel height is a single knob on `RaceTabsHost` (`h-40`); the badges fill it. Shorten it so the mode control fits the section without it growing too tall.

**Files:**
- Modify: `…/editor_page/components/editor_tabs_bar/components/race_tabs_host/style/mod.rs`

**Interfaces:**
- Consumes: nothing new. `RaceTabsHost` root is `@container w-full min-w-0 h-40`; the badge banners below fill this height, so lowering it slims every badge uniformly.
- Produces: shorter badges; no API change.

- [ ] **Step 1: Lower the carousel height**

Edit `…/race_tabs_host/style/mod.rs` — change `h-40` to a slimmer step (TUNE; start at `h-28`):

```rust
use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "w-full",
        "min-w-0",
        "h-28",
    ],
}
```

- [ ] **Step 2: Compile-check**

Run: `moon run :check`
Expected: `Finished` with no errors.

- [ ] **Step 3: Regenerate CSS and verify in the browser**

Run: `touch crates/hotkey-editor/tailwind.css`, reload, and confirm at **mobile / tablet / laptop / desktop / qhd / uhd**:
- The badges are visibly shorter but still legible (banner art, race label, active accent all still read well).
- The badges + mode control together form a tidy race section that isn't too tall.
- TUNE the exact height here (e.g. `h-24`…`h-32`) until it "still looks good" (handoff §1).
Expected: slimmer badges, section balanced at every band.

- [ ] **Step 4: Commit (only if the owner authorizes)**

```bash
git add "crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_tabs_bar/components/race_tabs_host/style/mod.rs"
git commit -m "style(editor-controls): slim the race badges"
```

---

## Self-review

**Spec coverage (Phase A slice of the design):**
- "mode → connected segmented control" → Task A1 (new `ActiveModeSegment`/`IdleModeSegment` leaves + connected `ModeTabs` container + `ModeTab` dispatcher). ✓
- "differentiation invariant: segmented = connected/gapless vs chips separated" → A1 removes the per-segment border/rounding and gap, adds one outer frame + divider. ✓
- "remove the `ModeTabsHost` cross-subtree mount from `unit_controls`" → Task A2. ✓
- "mode lives in the race row; badges slimmer" → A2 (placement) + A3 (height). ✓
- "`ToggleButton` not restyled; new affordances are new components" → A1 builds new leaves; `ToggleButton` untouched. ✓
- "dispatcher-of-two-looks, not class swap" → `ModeTab` `match`es. ✓

**Placeholder scan:** No TBD/TODO. `TUNE`-marked values are explicit browser-tuning of visual dimensions (`w-[16rem]`, `h-28`, above/below order) with concrete starting values and a stated acceptance ("looks good, reads as a peer") — the repo's mandated browser-tuning step, not a missing requirement.

**Type consistency:** `ActiveModeSegmentModel`/`IdleModeSegmentModel` fields (`label`, `onclick`, `onkeydown`) match what `ModeTab`'s dispatcher passes (it drops `active` after matching, and never carried `title`). `ModeTabModel` is unchanged, so `ModeTabs`' presentation (`ModeTabBinding`) still supplies exactly `label`/`active`/`onclick`/`onkeydown`. `ModeTabsView`/`ModeTabView` re-exports are preserved. `EditorTabsBar`/`UnitControls` signatures unchanged.

**Open item carried to Phase E:** After Phases B.2/C, the shared `ToggleButton` loses its remaining users and may become dead code — verify and surface for removal then (do not delete in Phase A; it is still used by search-scope and catalog toggles).
