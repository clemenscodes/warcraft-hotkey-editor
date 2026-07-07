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
    selected_slot: Signal<Option<GridSlotId>>,
    selected_hero_level: Signal<u32>,
    selected_from_research: Signal<bool>,
    selected_from_uprooted: Signal<bool>,
    hotkey_assign_request: Signal<bool>,
    tier_overrides: Signal<HashMap<String, usize>>,
    search_field: Signal<SearchField>,
    collapsed_categories: Signal<HashSet<UnitKind>>,
    show_abilityless_units: Signal<bool>,
    expand_variants: Signal<bool>,
    dragging_slot: Signal<Option<DraggingSlot>>,
    drop_target_tile: Signal<Option<DropTargetTile>>,
    drag_follower: Signal<Option<DragFollower>>,
    update_hotkeys_on_move: Signal<bool>,
}

impl EditorState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        selected_slot: Signal<Option<GridSlotId>>,
        selected_hero_level: Signal<u32>,
        selected_from_research: Signal<bool>,
        selected_from_uprooted: Signal<bool>,
        hotkey_assign_request: Signal<bool>,
        tier_overrides: Signal<HashMap<String, usize>>,
        search_field: Signal<SearchField>,
        collapsed_categories: Signal<HashSet<UnitKind>>,
        show_abilityless_units: Signal<bool>,
        expand_variants: Signal<bool>,
        dragging_slot: Signal<Option<DraggingSlot>>,
        drop_target_tile: Signal<Option<DropTargetTile>>,
        drag_follower: Signal<Option<DragFollower>>,
        update_hotkeys_on_move: Signal<bool>,
    ) -> Self {
        Self {
            selected_slot,
            selected_hero_level,
            selected_from_research,
            selected_from_uprooted,
            hotkey_assign_request,
            tier_overrides,
            search_field,
            collapsed_categories,
            show_abilityless_units,
            expand_variants,
            dragging_slot,
            drop_target_tile,
            drag_follower,
            update_hotkeys_on_move,
        }
    }

    pub fn selected_slot(&self) -> Signal<Option<GridSlotId>> {
        self.selected_slot
    }

    pub fn selected_hero_level(&self) -> Signal<u32> {
        self.selected_hero_level
    }

    pub fn selected_from_research(&self) -> Signal<bool> {
        self.selected_from_research
    }

    pub fn selected_from_uprooted(&self) -> Signal<bool> {
        self.selected_from_uprooted
    }

    pub fn hotkey_assign_request(&self) -> Signal<bool> {
        self.hotkey_assign_request
    }

    pub fn tier_overrides(&self) -> Signal<HashMap<String, usize>> {
        self.tier_overrides
    }

    pub fn search_field(&self) -> Signal<SearchField> {
        self.search_field
    }

    pub fn collapsed_categories(&self) -> Signal<HashSet<UnitKind>> {
        self.collapsed_categories
    }

    pub fn show_abilityless_units(&self) -> Signal<bool> {
        self.show_abilityless_units
    }

    pub fn expand_variants(&self) -> Signal<bool> {
        self.expand_variants
    }

    pub fn dragging_slot(&self) -> Signal<Option<DraggingSlot>> {
        self.dragging_slot
    }

    pub fn drop_target_tile(&self) -> Signal<Option<DropTargetTile>> {
        self.drop_target_tile
    }

    pub fn drag_follower(&self) -> Signal<Option<DragFollower>> {
        self.drag_follower
    }

    pub fn update_hotkeys_on_move(&self) -> Signal<bool> {
        self.update_hotkeys_on_move
    }
}
