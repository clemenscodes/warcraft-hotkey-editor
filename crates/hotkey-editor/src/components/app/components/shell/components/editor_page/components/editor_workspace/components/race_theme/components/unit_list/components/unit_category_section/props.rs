use dioxus::prelude::*;
use std::collections::HashSet;
use warcraft_api::{CatalogVisibility, SearchField, UnitMode};
use warcraft_api::{Race, UnitKind, WarcraftObjectId};
use warcraft_keybinds::GridSlotId;

/// One collapsible category of units in the sidebar list: which kind it is, its
/// label and collapsed state, the catalog query that fills it, and the selection
/// signals its cards drive.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCategorySectionProps {
    pub category_kind: UnitKind,
    pub category_label: String,
    pub is_collapsed: bool,
    pub collapsed_categories: Signal<HashSet<UnitKind>>,
    pub race: Race,
    pub mode: UnitMode,
    pub query: String,
    pub search_field: SearchField,
    pub visibility: CatalogVisibility,
    pub active_unit_id: Option<WarcraftObjectId>,
    pub selected_unit_id: Signal<Option<WarcraftObjectId>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub active_category: Signal<UnitKind>,
}
