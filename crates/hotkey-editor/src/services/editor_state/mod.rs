use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use warcraft_api::SearchField;
use warcraft_api::UnitKind;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

pub mod context;

/// The editor view's own UI state: the selected slot, the per-catalog toggles, and the
/// tier overrides. Provided once by the app shell and read by the editor page from
/// context, so the page is fed no god-bag of signals as props. Each field is a `Signal`,
/// so a reader subscribes only to the slice it touches; the whole struct is `Copy`, so a
/// handler captures it cheaply. The in-progress drag is its own concern and lives in
/// [`DragState`](crate::services::drag_state::DragState), not here.
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
    tier_overrides: Signal<HashMap<WarcraftObjectId, usize>>,
    search_field: Signal<SearchField>,
    collapsed_categories: Signal<HashSet<UnitKind>>,
    active_category: Signal<UnitKind>,
    show_abilityless_units: Signal<bool>,
    expand_variants: Signal<bool>,
    update_hotkeys_on_move: Signal<bool>,
}

impl EditorState {
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

    pub fn tier_overrides(&self) -> Signal<HashMap<WarcraftObjectId, usize>> {
        self.tier_overrides
    }

    pub fn search_field(&self) -> Signal<SearchField> {
        self.search_field
    }

    pub fn collapsed_categories(&self) -> Signal<HashSet<UnitKind>> {
        self.collapsed_categories
    }

    pub fn active_category(&self) -> Signal<UnitKind> {
        self.active_category
    }

    pub fn show_abilityless_units(&self) -> Signal<bool> {
        self.show_abilityless_units
    }

    pub fn expand_variants(&self) -> Signal<bool> {
        self.expand_variants
    }

    pub fn update_hotkeys_on_move(&self) -> Signal<bool> {
        self.update_hotkeys_on_move
    }
}
