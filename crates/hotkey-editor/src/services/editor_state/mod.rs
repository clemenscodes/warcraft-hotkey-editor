use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use warcraft_api::UnitKind;
use warcraft_database::SearchField;
use warcraft_keybinds::GridSlotId;

pub mod context;
pub mod drag;
pub mod hit_test;

pub use drag::{DragFollower, DragFollowerVisual, DraggingSlot, DropTargetTile};
pub(crate) use hit_test::{CursorPoint, HitTestPoint};

/// The editor view's own UI state: the selected slot, the drag machinery, the
/// per-catalog toggles, and the tier overrides. Provided once by the app shell and
/// read by the editor page from context, so the page is fed no god-bag of signals as
/// props. Each field is a `Signal`, so a reader subscribes only to the slice it
/// touches; the whole struct is `Copy`, so a handler captures it cheaply.
///
/// The nav-scoped signals (race, mode, selected unit, search query) live in
/// [`ViewNavigationContext`](crate::services::navigation::view_navigation::ViewNavigationContext)
/// and the document lives in [`CustomKeysService`](crate::services::customkeys::service::CustomKeysService);
/// this is only the state confined to the editor subtree.
#[derive(Clone, Copy, PartialEq)]
pub struct EditorState {
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub selected_hero_level: Signal<u32>,
    pub selected_from_research: Signal<bool>,
    pub selected_from_uprooted: Signal<bool>,
    pub hotkey_assign_request: Signal<bool>,
    pub tier_overrides: Signal<HashMap<String, usize>>,
    pub search_field: Signal<SearchField>,
    pub collapsed_categories: Signal<HashSet<UnitKind>>,
    pub show_abilityless_units: Signal<bool>,
    pub expand_variants: Signal<bool>,
    pub dragging_slot: Signal<Option<DraggingSlot>>,
    pub drop_target_tile: Signal<Option<DropTargetTile>>,
    pub drag_follower: Signal<Option<DragFollower>>,
    pub update_hotkeys_on_move: Signal<bool>,
}
