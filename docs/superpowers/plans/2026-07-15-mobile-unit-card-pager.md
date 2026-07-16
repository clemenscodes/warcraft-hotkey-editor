# Mobile UnitCard Pager Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the primitive mobile editor screen with a vertically scroll-snapped pager of `UnitCard`s — one blue card (icon + name + id header over the unit's command grids) per unit, every race — that builds only the cards in or adjacent to the viewport.

**Architecture:** `MobileEditor` becomes a scroll-viewport that lists every unit (race-ordered), captures its own pixel height via `onmounted`, tracks the active card index on `onscroll`, and renders only a 3-card window (`active-1 ..= active+1`) between two height-spacer `div`s that preserve total scroll height. Each `UnitCard` re-resolves its own header (`UnitView`) and command grids (`UnitSlotContainers`) from its `unit_id` prop and reuses the existing `UnitCommandGrids` leaf. No navigation, search, filtering, or `cqi` scaling.

**Tech Stack:** Rust, Dioxus 0.7 (wasm), `web_sys` (scroll metrics), Tailwind via the project `classes!`/`tw!` macros, the pinned `warcraft-api`/`warcraft-keybinds` domain crates (git tag `v0.8.0`).

## Global Constraints

- **Only three commands exist:** `moon run :ci` (the full gate — fmt, clippy, tests, wasm build, Playwright e2e; long, user-run), `moon run :check` (fast wasm compile check — use this to iterate), `moon run :dev` (dev server at `http://localhost:8123/warcraft-hotkey-editor/`, trailing slash; bare `/` is 404). Never `cargo`/`dx`/`playwright`/narrower `moon` targets. Never run a second `moon run :ci` while one is running.
- **Renderer-only change.** No edits to `warcraft-keybinds`/`warcraft-api`. No domain logic in the renderer — only stateless catalog reads (`WarcraftApi`, `UnitSlotContainers::resolve`), exactly as the current mobile editor already does. `viewport_px`/`active_index` are UI-only signals (ARCHITECTURE R10), never persisted.
- **COMPONENTS.md:** directory == component == class; one classed root per component; body is hook-call(s) then pure RSX; children live under the component's own `components/`; every component has `assert_component!` and a `classes!` `style/mod.rs`; ddd `View`/`Model` trio per component (`#[component] fn Foo(props: FooModel)`, `Model: From<&View>`, both `impl ddd::…`). Props types are private (`mod model;`, never re-exported). Optional rendering is a guarded early-return in a leaf, never an `if let` in the body.
- **RUST_STYLE.md:** full semantic names; no tuples; no `as` casts outside `From`/`TryFrom` bodies; no inline numeric suffixes (annotate the binding: `let rank: u8 = 0`); `Self` inside impls; derive every qualifying trait.
- **No `cqi` in this batch.** Fixed tokens/bands only (the IA-first stage). The `cqi` pass is a separate, later effort.
- **Verification model:** renderer components carry no unit tests; each task is verified by `moon run :check` (compiles) plus, where the component is mounted, in-browser inspection at a mobile viewport via the Playwright MCP. The full `moon run :ci` gate is the final task.

---

## File Structure

All paths are under
`crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/mobile_editor/`
(abbreviated **`{ME}/`** below).

- `{ME}/mod.rs` — **modify.** `MobileEditor` becomes the windowed pager body.
- `{ME}/presentation/mod.rs` — **replace.** Owns the race-ordered unit-id list, the `viewport_px`/`active_index` signals, the `onmounted`/`onscroll` handlers, and the computed window + spacer heights.
- `{ME}/style/mod.rs` — **modify.** Add scroll-snap viewport classes.
- `{ME}/components/mod.rs` — **create.** `pub mod unit_card; pub mod unit_card_spacer;`
- `{ME}/components/unit_card_spacer/{mod,view,model,style}.rs` — **create.** A fixed-height flex filler (`UnitCardSpacer { height_px: i32 }`).
- `{ME}/components/unit_card/{mod,view,model,style}.rs` + `presentation/mod.rs` + `components/mod.rs` — **create.** `UnitCard { unit_id }`: the blue card; resolves header + grids; renders `UnitCardHeader` + `UnitCommandGrids`.
- `{ME}/components/unit_card/components/unit_card_header/{mod,view,model,style}.rs` + `components/mod.rs` — **create.** `UnitCardHeader { icon_url, name, unit_id }`.
- `{ME}/components/unit_card/components/unit_card_header/components/unit_card_portrait/{mod,view,model,style}.rs` — **create.** `UnitCardPortrait { src: Option<String> }` (early-returns empty when `None`).
- `{ME}/components/unit_card/components/unit_card_header/components/unit_card_name/{mod,view,model,style}.rs` — **create.** `UnitCardName { name: String }`.
- `{ME}/components/unit_card/components/unit_card_header/components/unit_card_id/{mod,view,model,style}.rs` — **create.** `UnitCardId { unit_id: WarcraftObjectId }`.

Build order is bottom-up so every `moon run :check` is green: header leaves → header → card → spacer → rewire `MobileEditor`.

---

### Task 1: UnitCardHeader and its three leaves

**Files:**
- Create: `{ME}/components/unit_card/components/unit_card_header/components/unit_card_portrait/mod.rs` (+ `view/mod.rs`, `model/mod.rs`, `style/mod.rs`)
- Create: `{ME}/components/unit_card/components/unit_card_header/components/unit_card_name/mod.rs` (+ `view/mod.rs`, `model/mod.rs`, `style/mod.rs`)
- Create: `{ME}/components/unit_card/components/unit_card_header/components/unit_card_id/mod.rs` (+ `view/mod.rs`, `model/mod.rs`, `style/mod.rs`)
- Create: `{ME}/components/unit_card/components/unit_card_header/components/mod.rs`
- Create: `{ME}/components/unit_card/components/unit_card_header/mod.rs` (+ `view/mod.rs`, `model/mod.rs`, `style/mod.rs`)

**Interfaces:**
- Produces: `UnitCardHeader(props: UnitCardHeaderModel)` where `UnitCardHeaderModel { icon_url: Option<String>, name: String, unit_id: WarcraftObjectId }`; leaves `UnitCardPortrait { src: Option<String> }`, `UnitCardName { name: String }`, `UnitCardId { unit_id: WarcraftObjectId }`.

- [ ] **Step 1: Create the portrait leaf.**

`unit_card_portrait/view/mod.rs`:
```rust
#[derive(Clone, PartialEq)]
pub struct UnitCardPortraitView {
    pub src: Option<String>,
}

impl ddd::View for UnitCardPortraitView {}
```

`unit_card_portrait/model/mod.rs`:
```rust
use super::view::UnitCardPortraitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCardPortraitModel {
    pub src: Option<String>,
}

impl From<&UnitCardPortraitView> for UnitCardPortraitModel {
    fn from(view: &UnitCardPortraitView) -> Self {
        let UnitCardPortraitView { src } = view.clone();
        Self { src }
    }
}

impl ddd::Model for UnitCardPortraitModel {
    type View = UnitCardPortraitView;
}
```

`unit_card_portrait/style/mod.rs`:
```rust
use tw_macro::tw;
classes! {
    base: tw![
        "size-14",
        "shrink-0",
        "border-2",
        "border-warcraft-blue",
        "rounded-control",
        "object-cover",
        "shadow-raised",
        "bg-warcraft-bg-panel/70",
        "text-transparent",
        "leading-0",
    ],
}
```

`unit_card_portrait/mod.rs` (optional render is a guarded early-return — no `if let` in a parent body):
```rust
mod model;
mod view;

pub use view::UnitCardPortraitView;
mod style;

use dioxus::prelude::*;
use model::UnitCardPortraitModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitCardPortrait(props: UnitCardPortraitModel) -> Element {
    let Some(src) = props.src else {
        return rsx! {};
    };
    rsx! {
        img {
            class: CLASS,
            src,
            alt: "",
            loading: "lazy",
            decoding: "async",
        }
    }
}

assert_component!(UnitCardPortrait);
```

- [ ] **Step 2: Create the name leaf.**

`unit_card_name/view/mod.rs`:
```rust
#[derive(Clone, PartialEq)]
pub struct UnitCardNameView {
    pub name: String,
}

impl ddd::View for UnitCardNameView {}
```

`unit_card_name/model/mod.rs`:
```rust
use super::view::UnitCardNameView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCardNameModel {
    #[props(into)]
    pub name: String,
}

impl From<&UnitCardNameView> for UnitCardNameModel {
    fn from(view: &UnitCardNameView) -> Self {
        let UnitCardNameView { name } = view.clone();
        Self { name }
    }
}

impl ddd::Model for UnitCardNameModel {
    type View = UnitCardNameView;
}
```

`unit_card_name/style/mod.rs`:
```rust
use tw_macro::tw;
classes! {
    base: tw![
        "min-w-0",
        "flex-1",
        "truncate",
        "text-xl",
        "font-semibold",
        "text-warcraft-gold",
        "text-shadow-drop",
    ],
}
```

`unit_card_name/mod.rs`:
```rust
mod model;
mod view;

pub use view::UnitCardNameView;
mod style;

use dioxus::prelude::*;
use model::UnitCardNameModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitCardName(props: UnitCardNameModel) -> Element {
    let name = props.name;
    rsx! {
        span {
            class: CLASS,
            {name}
        }
    }
}

assert_component!(UnitCardName);
```

- [ ] **Step 3: Create the id leaf** (mirrors the desktop `UnitId`, which renders `props.unit_id.value()`).

`unit_card_id/view/mod.rs`:
```rust
use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct UnitCardIdView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for UnitCardIdView {}
```

`unit_card_id/model/mod.rs`:
```rust
use super::view::UnitCardIdView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCardIdModel {
    pub unit_id: WarcraftObjectId,
}

impl From<&UnitCardIdView> for UnitCardIdModel {
    fn from(view: &UnitCardIdView) -> Self {
        let UnitCardIdView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Model for UnitCardIdModel {
    type View = UnitCardIdView;
}
```

`unit_card_id/style/mod.rs`:
```rust
use tw_macro::tw;
classes! {
    base: tw![
        "shrink-0",
        "text-sm",
        "text-warcraft-text-secondary",
    ],
}
```

`unit_card_id/mod.rs`:
```rust
mod model;
mod view;

pub use view::UnitCardIdView;
mod style;

use dioxus::prelude::*;
use model::UnitCardIdModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitCardId(props: UnitCardIdModel) -> Element {
    rsx! {
        code {
            class: CLASS,
            {props.unit_id.value()}
        }
    }
}

assert_component!(UnitCardId);
```

- [ ] **Step 4: Wire the header's `components/mod.rs`.**

`unit_card_header/components/mod.rs`:
```rust
pub mod unit_card_id;
pub mod unit_card_name;
pub mod unit_card_portrait;
```

- [ ] **Step 5: Create `UnitCardHeader`.**

`unit_card_header/view/mod.rs`:
```rust
use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct UnitCardHeaderView {
    pub icon_url: Option<String>,
    pub name: String,
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for UnitCardHeaderView {}
```

`unit_card_header/model/mod.rs`:
```rust
use super::view::UnitCardHeaderView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCardHeaderModel {
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
    pub unit_id: WarcraftObjectId,
}

impl From<&UnitCardHeaderView> for UnitCardHeaderModel {
    fn from(view: &UnitCardHeaderView) -> Self {
        let UnitCardHeaderView {
            icon_url,
            name,
            unit_id,
        } = view.clone();
        Self {
            icon_url,
            name,
            unit_id,
        }
    }
}

impl ddd::Model for UnitCardHeaderModel {
    type View = UnitCardHeaderView;
}
```

`unit_card_header/style/mod.rs`:
```rust
use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-row",
        "items-center",
        "gap-3",
        "shrink-0",
    ],
}
```

`unit_card_header/mod.rs`:
```rust
pub mod components;
mod model;
mod view;

pub use view::UnitCardHeaderView;
mod style;

use components::unit_card_id::UnitCardId;
use components::unit_card_name::UnitCardName;
use components::unit_card_portrait::UnitCardPortrait;
use dioxus::prelude::*;
use model::UnitCardHeaderModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitCardHeader(props: UnitCardHeaderModel) -> Element {
    let icon_url = props.icon_url;
    let name = props.name;
    let unit_id = props.unit_id;
    rsx! {
        div {
            class: CLASS,
            UnitCardPortrait {
                src: icon_url,
            }
            UnitCardName {
                name,
            }
            UnitCardId {
                unit_id,
            }
        }
    }
}

assert_component!(UnitCardHeader);
```

- [ ] **Step 6: Compile-check.**

Run: `moon run :check`
Expected: compiles clean. (Nothing renders `UnitCardHeader` yet — the check only confirms the new components type-check and satisfy `assert_component!`.)

---

### Task 2: UnitCard (the blue card)

**Files:**
- Create: `{ME}/components/unit_card/mod.rs` (+ `view/mod.rs`, `model/mod.rs`, `style/mod.rs`, `presentation/mod.rs`, `components/mod.rs`)

**Interfaces:**
- Consumes: `UnitCardHeader { icon_url, name, unit_id }` (Task 1); the existing `UnitCommandGrids { unit_id, command_card_slots, build_menu_slots, uprooted_menu_slots, research_menu_slots }`.
- Produces: `UnitCard(props: UnitCardModel)` where `UnitCardModel { unit_id: WarcraftObjectId }`.

- [ ] **Step 1: View + Model.**

`unit_card/view/mod.rs`:
```rust
use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct UnitCardView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for UnitCardView {}
```

`unit_card/model/mod.rs`:
```rust
use super::view::UnitCardView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCardModel {
    pub unit_id: WarcraftObjectId,
}

impl From<&UnitCardView> for UnitCardModel {
    fn from(view: &UnitCardView) -> Self {
        let UnitCardView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Model for UnitCardModel {
    type View = UnitCardView;
}
```

- [ ] **Step 2: Presentation** — resolve the header identity and the four slot groups from `unit_id`. Both are static catalog reads; memoize each so a card resolves once.

`unit_card/presentation/mod.rs`:
```rust
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::{WarcraftApi, WarcraftObjectId};
use warcraft_keybinds::{GridSlotId, UnitSlotContainers};

use super::model::UnitCardModel;

#[derive(Clone, PartialEq)]
struct UnitCardIdentity {
    name: String,
    icon_url: Option<String>,
}

pub(super) struct UnitCardPresentation {
    pub(super) icon_url: Option<String>,
    pub(super) name: String,
    pub(super) unit_id: WarcraftObjectId,
    pub(super) command_card_slots: Rc<[GridSlotId]>,
    pub(super) build_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) research_menu_slots: Option<Rc<[GridSlotId]>>,
}

pub(super) fn use_unit_card(props: &UnitCardModel) -> UnitCardPresentation {
    let unit_id = props.unit_id;
    let slot_data = use_memo(move || UnitSlotContainers::resolve(unit_id));
    let identity = use_memo(move || {
        let api = WarcraftApi::default();
        let unit_view = api.unit().get(unit_id);
        let display_name = unit_view
            .as_ref()
            .and_then(|unit| unit.name())
            .unwrap_or("(unnamed)")
            .to_string();
        let icon_url = unit_view
            .as_ref()
            .and_then(|unit| unit.icon())
            .map(IconUrl::from_database_path)
            .map(|icon| icon.to_string());
        UnitCardIdentity {
            name: display_name,
            icon_url,
        }
    });

    let slot_containers = slot_data.read();
    let command_card_slots = slot_containers.command_card();
    let build_menu_slots = slot_containers.build_menu();
    let uprooted_menu_slots = slot_containers.uprooted();
    let research_menu_slots = slot_containers.research();

    let resolved_identity = identity.read();
    let name = resolved_identity.name.clone();
    let icon_url = resolved_identity.icon_url.clone();

    UnitCardPresentation {
        icon_url,
        name,
        unit_id,
        command_card_slots,
        build_menu_slots,
        uprooted_menu_slots,
        research_menu_slots,
    }
}
```

- [ ] **Step 3: Style** — the blue card, mirroring the desktop `unit_detail` card *values*. Each card is one viewport tall (`h-full`), a snap page, and scrolls internally if its grids overflow.

`unit_card/style/mod.rs`:
```rust
use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "gap-4",
        "h-full",
        "shrink-0",
        "snap-start",
        "overflow-y-auto",
        "overscroll-contain",
        "p-4",
        "border",
        "border-warcraft-blue-deep",
        "rounded-card",
        "bg-panel-dark",
        "shadow-bevel",
    ],
}
```

- [ ] **Step 4: `components/mod.rs`** for the card.

`unit_card/components/mod.rs`:
```rust
pub mod unit_card_header;
```

- [ ] **Step 5: The card body.** Renders the header then the reused `UnitCommandGrids` (the one deliberate cross-tree reuse, per the spec — same import the mobile editor uses today).

`unit_card/mod.rs`:
```rust
pub mod components;
mod model;
mod presentation;
mod view;

pub use view::UnitCardView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_command_grids::UnitCommandGrids;
use components::unit_card_header::UnitCardHeader;
use dioxus::prelude::*;
use model::UnitCardModel;
use presentation::use_unit_card;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitCard(props: UnitCardModel) -> Element {
    let presentation = use_unit_card(&props);
    let UnitCardPresentation {
        icon_url,
        name,
        unit_id,
        command_card_slots,
        build_menu_slots,
        uprooted_menu_slots,
        research_menu_slots,
    } = presentation;
    rsx! {
        div {
            class: CLASS,
            UnitCardHeader {
                icon_url,
                name,
                unit_id,
            }
            UnitCommandGrids {
                unit_id,
                command_card_slots,
                build_menu_slots,
                uprooted_menu_slots,
                research_menu_slots,
            }
        }
    }
}

use presentation::UnitCardPresentation;

assert_component!(UnitCard);
```

- [ ] **Step 6: Compile-check.**

Run: `moon run :check`
Expected: compiles clean. `UnitCard` type-checks and reuses `UnitCommandGrids`. Still not mounted anywhere.

---

### Task 3: UnitCardSpacer

**Files:**
- Create: `{ME}/components/unit_card_spacer/mod.rs` (+ `view/mod.rs`, `model/mod.rs`, `style/mod.rs`)

**Interfaces:**
- Produces: `UnitCardSpacer(props: UnitCardSpacerModel)` where `UnitCardSpacerModel { height_px: i32 }`. A full-width, non-shrinking flex filler whose height is an inline style (a computed dimension, so it cannot be a Tailwind literal).

- [ ] **Step 1: View + Model.**

`unit_card_spacer/view/mod.rs`:
```rust
#[derive(Clone, PartialEq)]
pub struct UnitCardSpacerView {
    pub height_px: i32,
}

impl ddd::View for UnitCardSpacerView {}
```

`unit_card_spacer/model/mod.rs`:
```rust
use super::view::UnitCardSpacerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCardSpacerModel {
    pub height_px: i32,
}

impl From<&UnitCardSpacerView> for UnitCardSpacerModel {
    fn from(view: &UnitCardSpacerView) -> Self {
        let UnitCardSpacerView { height_px } = view.clone();
        Self { height_px }
    }
}

impl ddd::Model for UnitCardSpacerModel {
    type View = UnitCardSpacerView;
}
```

`unit_card_spacer/style/mod.rs`:
```rust
use tw_macro::tw;
classes! {
    base: tw![
        "w-full",
        "shrink-0",
    ],
}
```

`unit_card_spacer/mod.rs`:
```rust
mod model;
mod view;

pub use view::UnitCardSpacerView;
mod style;

use dioxus::prelude::*;
use model::UnitCardSpacerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitCardSpacer(props: UnitCardSpacerModel) -> Element {
    let height_px = props.height_px;
    let inline_height = format!("height: {height_px}px;");
    rsx! {
        div {
            class: CLASS,
            style: inline_height,
        }
    }
}

assert_component!(UnitCardSpacer);
```

- [ ] **Step 2: Compile-check.**

Run: `moon run :check`
Expected: compiles clean.

---

### Task 4: Rewire MobileEditor into the windowed pager

**Files:**
- Create: `{ME}/components/mod.rs`
- Modify: `{ME}/presentation/mod.rs` (replace entirely)
- Modify: `{ME}/mod.rs`
- Modify: `{ME}/style/mod.rs`

**Interfaces:**
- Consumes: `UnitCard { unit_id }` (Task 2), `UnitCardSpacer { height_px }` (Task 3).
- Produces: the mobile pager. `use_mobile_editor() -> MobileEditorPresentation { onmounted: EventHandler<MountedEvent>, onscroll: EventHandler<ScrollEvent>, top_spacer_px: i32, bottom_spacer_px: i32, window_unit_ids: Vec<WarcraftObjectId> }`.

Design notes for the implementer:
- **Why `i32`, not `f64`:** `web_sys::Element::client_height()` and `scroll_top()` both return `i32`. Working in `i32` keeps the index math integer-only and avoids `as` casts (banned outside `From`/`TryFrom`). Rounded index = `(scroll_top + height/2) / height`.
- **Element capture:** store the scroll element in an `Rc<RefCell<Option<web_sys::Element>>>` via `use_hook` (mirrors `shared/drag_scroll.rs`), because `onscroll`'s event does not hand you the element — you read `scroll_top()`/`client_height()` off the captured element.
- **Re-render discipline:** update `viewport_px`/`active_index` signals only when their value actually changes (`peek()` compare before `set`), so a fast flick re-renders once per card boundary, not per scroll pixel.
- **Buffer:** `BUFFER = 1` (the card above and below). Before the first `onmounted` (`viewport_px == 0`), render an initial window of the first `2*BUFFER + 1` cards with zero spacers.

- [ ] **Step 1: Create `{ME}/components/mod.rs`.**

```rust
pub mod unit_card;
pub mod unit_card_spacer;
```

- [ ] **Step 2: Replace `{ME}/presentation/mod.rs`.**

```rust
use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use warcraft_api::{Race, WarcraftApi, WarcraftObjectId};

const CARD_WINDOW_BUFFER: usize = 1;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct UnitOrder {
    race_rank: u8,
    name: &'static str,
    unit_id: WarcraftObjectId,
}

impl UnitOrder {
    fn rank_of(race: Option<Race>) -> u8 {
        let rank: u8 = match race {
            Some(Race::Human) => 0,
            Some(Race::Orc) => 1,
            Some(Race::Nightelf) => 2,
            Some(Race::Undead) => 3,
            Some(Race::Neutral) => 4,
            None => 5,
        };
        rank
    }
}

pub(super) struct MobileEditorPresentation {
    pub(super) onmounted: EventHandler<MountedEvent>,
    pub(super) onscroll: EventHandler<ScrollEvent>,
    pub(super) top_spacer_px: i32,
    pub(super) bottom_spacer_px: i32,
    pub(super) window_unit_ids: Vec<WarcraftObjectId>,
}

pub(super) fn use_mobile_editor() -> MobileEditorPresentation {
    let unit_ids_memo = use_memo(|| {
        let api = WarcraftApi::default();
        let mut ordered: Vec<UnitOrder> = api
            .unit()
            .all()
            .map(|unit| {
                let race_rank = UnitOrder::rank_of(unit.race());
                let name = unit.name().unwrap_or("(unnamed)");
                let unit_id = unit.id();
                UnitOrder {
                    race_rank,
                    name,
                    unit_id,
                }
            })
            .collect();
        ordered.sort();
        let ids: Rc<[WarcraftObjectId]> = ordered.into_iter().map(|order| order.unit_id).collect();
        ids
    });

    let viewport_px = use_signal::<i32>(|| 0);
    let active_index = use_signal::<usize>(|| 0);
    let element_ref = use_hook(|| Rc::new(RefCell::new(None::<web_sys::Element>)));

    let unit_ids = unit_ids_memo();
    let unit_count = unit_ids.len();

    let mounted_element_ref = element_ref.clone();
    let mut mounted_viewport_px = viewport_px;
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        let Some(element) = event.data().try_as_web_event() else {
            return;
        };
        let measured_height = element.client_height();
        *mounted_element_ref.borrow_mut() = Some(element);
        if *mounted_viewport_px.peek() != measured_height {
            mounted_viewport_px.set(measured_height);
        }
    });

    let scroll_element_ref = element_ref.clone();
    let mut scroll_viewport_px = viewport_px;
    let mut scroll_active_index = active_index;
    let onscroll = EventHandler::new(move |_event: ScrollEvent| {
        let borrowed = scroll_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let measured_height = element.client_height();
        if measured_height <= 0 {
            return;
        }
        let scroll_top = element.scroll_top();
        let rounded_index = (scroll_top + measured_height / 2) / measured_height;
        let last_index = unit_count.saturating_sub(1);
        let clamped_index = usize::try_from(rounded_index).unwrap_or(0).min(last_index);
        if *scroll_viewport_px.peek() != measured_height {
            scroll_viewport_px.set(measured_height);
        }
        if *scroll_active_index.peek() != clamped_index {
            scroll_active_index.set(clamped_index);
        }
    });

    let current_viewport_px = *viewport_px.read();
    let current_active_index = *active_index.read();

    let window_start;
    let window_end;
    if current_viewport_px <= 0 {
        window_start = 0;
        window_end = unit_count.min(2 * CARD_WINDOW_BUFFER + 1);
    } else {
        window_start = current_active_index.saturating_sub(CARD_WINDOW_BUFFER);
        window_end = (current_active_index + CARD_WINDOW_BUFFER + 1).min(unit_count);
    }

    let window_unit_ids = unit_ids[window_start..window_end].to_vec();
    let leading_cards = i32::try_from(window_start).unwrap_or(0);
    let trailing_cards = i32::try_from(unit_count - window_end).unwrap_or(0);
    let top_spacer_px = leading_cards * current_viewport_px;
    let bottom_spacer_px = trailing_cards * current_viewport_px;

    MobileEditorPresentation {
        onmounted,
        onscroll,
        top_spacer_px,
        bottom_spacer_px,
        window_unit_ids,
    }
}
```

Note: `MountedEvent`, `ScrollEvent`, `use_signal`, `use_memo`, `use_hook`, and `try_as_web_event` all come from `dioxus::prelude::*` (`try_as_web_event` via the `dioxus::web::WebEventExt` trait — add `use dioxus::web::WebEventExt;` if the prelude does not re-export it; `shared/drag_scroll.rs` imports it explicitly, so mirror that: add `use dioxus::web::WebEventExt;` at the top).

- [ ] **Step 3: Replace `{ME}/mod.rs`.**

```rust
pub mod components;
mod presentation;
mod style;

use components::unit_card::UnitCard;
use components::unit_card_spacer::UnitCardSpacer;
use dioxus::prelude::*;
use presentation::{MobileEditorPresentation, use_mobile_editor};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MobileEditor() -> Element {
    let presentation = use_mobile_editor();
    let MobileEditorPresentation {
        onmounted,
        onscroll,
        top_spacer_px,
        bottom_spacer_px,
        window_unit_ids,
    } = presentation;
    rsx! {
        section {
            class: CLASS,
            aria_label: "Mobile editor",
            onmounted: move |event| onmounted.call(event),
            onscroll: move |event| onscroll.call(event),
            UnitCardSpacer {
                height_px: top_spacer_px,
            }
            for unit_id in window_unit_ids {
                UnitCard {
                    key: "{unit_id}",
                    unit_id,
                }
            }
            UnitCardSpacer {
                height_px: bottom_spacer_px,
            }
        }
    }
}

assert_component!(MobileEditor);
```

- [ ] **Step 4: Add scroll-snap classes to `{ME}/style/mod.rs`.**

```rust
use tw_macro::tw;

classes! {
    base: tw![
        "hidden",
    ],
    mobile: tw![
        "mobile:flex",
        "mobile:flex-col",
        "mobile:flex-1",
        "mobile:min-h-0",
        "mobile:min-w-0",
        "mobile:overflow-y-auto",
        "mobile:overscroll-contain",
        "mobile:snap-y",
        "mobile:snap-mandatory",
        "mobile:px-4",
    ],
}
```

- [ ] **Step 5: Compile-check.**

Run: `moon run :check`
Expected: compiles clean. If `ScrollEvent` is not the correct alias, the error will name it — substitute `Event<ScrollData>` (both are in the Dioxus prelude) and re-run.

- [ ] **Step 6: Browser verification (Playwright MCP).**

Ensure the dev server is up (`moon run :dev`; it is often already running — do not squat port 8123). Then, via the Playwright MCP:
1. `browser_resize` to a mobile viewport (e.g. 390×844).
2. `browser_navigate` to `http://localhost:8123/warcraft-hotkey-editor/` (trailing slash). If the "being rebuilt" overlay shows, re-navigate — do not wait.
3. `browser_snapshot` / `browser_take_screenshot`: confirm a blue `UnitCard` fills the screen with an icon + name + rawcode header above the command grids.
4. Scroll the pager (`browser_evaluate` scrolling the `section[aria-label="Mobile editor"]`, or swipe): confirm it snaps card-to-card and the visible unit changes, walking Human → Orc → NightElf → Undead → Neutral.
5. `browser_evaluate` `document.querySelectorAll('.unit-card').length` — expect a small number (≈`2*BUFFER + 1 = 3`), not the full unit count, proving windowing. Confirm the two spacer `div`s carry a non-zero pixel height away from the list ends.

Expected: all of the above hold. If cards are not one-viewport tall, the pager's parent flex column is not giving `MobileEditor` a bounded height — verify the shell body is a bounded `min-h-0` flex column and fix the ancestor, not the card.

---

### Task 5: Full gate + optional e2e smoke

**Files:**
- Optional create: `crates/hotkey-editor/e2e/tests/<mirrors an existing spec name>.spec.ts`

- [ ] **Step 1 (optional): Add a mobile pager e2e smoke.**

Read one existing spec in `crates/hotkey-editor/e2e/tests/` and mirror its harness (fixtures, base-URL helper, imports) exactly. Add a test that sets a mobile viewport and asserts the pager renders and windows:

```ts
test("mobile unit-card pager renders one windowed card with header and grids", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto(BASE_URL); // reuse the existing spec's base-URL constant/helper
  const card = page.locator(".unit-card").first();
  await expect(card).toBeVisible();
  await expect(card.locator(".unit-card-header")).toBeVisible();
  // windowing: only a handful of cards exist, never the full catalog
  await expect(await page.locator(".unit-card").count()).toBeLessThan(6);
});
```

Keep selectors on identity classes (`.unit-card`, `.unit-card-header`) — they are a coupled contract; if a later task renames them, update this spec in the same commit.

- [ ] **Step 2: Run the full gate.**

Run: `moon run :ci`
Expected: green — fmt, clippy, keybinds tests, wasm build, and all Playwright e2e (the four existing smokes plus the optional new one). Do not start it if a gate is already running (`pgrep -af "moon run|playwright|dx"`; check port 8123) — coordinate with the user instead.

- [ ] **Step 3: Final in-browser sanity.**

Re-confirm the Task 4 Step 6 checklist once more against the release-mode/gate-built app if the gate rebuilt it, so the delivered screen matches the spec's §9 verification.

---

## Self-Review

**Spec coverage** (against `docs/superpowers/specs/2026-07-15-mobile-unit-card-pager-design.md`):
- §2 component structure → Tasks 1–4 create every named directory (`unit_card`, `unit_card_header`, `unit_card_portrait/name/id`, `unit_card_spacer`) with the ddd trio + `assert_component!`. ✓
- §3 blue card → Task 2 Step 3 mirrors `border-warcraft-blue-deep / rounded-card / bg-panel-dark / shadow-bevel` as the card's own values. ✓
- §4 windowing (viewport height, `round(scrollTop/viewportHeight)` index, 3-card window, spacers) → Task 4 Step 2, integer-`i32` variant. ✓
- §5 data (`unit().all()` race-ordered; per-card `UnitSlotContainers::resolve` + `UnitView`) → Task 4 (list ordering) + Task 2 (per-card). ✓
- §6 UI-only state → `viewport_px`/`active_index` are `use_signal`, never persisted. ✓
- §7 known debt (`UnitCommandGrids` cross-tree reuse kept, header leaves built fresh) → Task 2 reuses `UnitCommandGrids`; Task 1 builds fresh header leaves. ✓
- §8 risks (nested scroll+snap, re-render cost, pre-mount window) → handled in Task 4 (overscroll-contain, peek-guarded sets, `viewport_px == 0` initial window) and called out in Step 6. ✓
- §9 verification → Task 4 Step 6 + Task 5. ✓

**Placeholder scan:** no TBD/TODO; every code step shows full file contents; the one "mirror an existing spec" step (e2e harness) is explicitly optional and bounded.

**Type consistency:** `MobileEditorPresentation`, `UnitCardPresentation`, `UnitCardModel`/`View`, `UnitCardHeaderModel` field names (`icon_url`, `name`, `unit_id`) match across producer and consumer tasks; `UnitCommandGrids` field names match the extracted verbatim `UnitCommandGridsModel`; `height_px: i32` consistent between `UnitCardSpacer` and the presentation's `top_spacer_px`/`bottom_spacer_px`.
