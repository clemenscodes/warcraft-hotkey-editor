use dioxus::prelude::*;
use std::collections::HashSet;
use warcraft_api::{CatalogVisibility, SearchField, UnitKindHelpers, UnitMode};
use warcraft_api::{Race, UnitKind, WarcraftObjectId};
use warcraft_keybinds::GridSlotId;

use super::components::catalog_visibility_toggle::CatalogVisibilityToggleProps;
use super::components::mobile_category_tab::MobileCategoryTabProps;
use super::components::search_field_toggle::SearchFieldToggleProps;
use super::components::unit_category_section::UnitCategorySectionProps;
use super::components::unit_list_search::UnitListSearchProps;
use super::state::UnitListState;
use crate::services::focus::coordinator::FocusCoordinator;

/// The categories the mobile tab bar shows, in display order.
pub(super) const MOBILE_CATEGORY_ORDER: [UnitKind; 4] = [
    UnitKind::Hero,
    UnitKind::Soldier,
    UnitKind::Worker,
    UnitKind::Building,
];

/// Everything the unit list reads and writes: the current race and mode, the
/// selection it drives, and the search and catalog-visibility state it owns.
#[derive(Props, Clone, PartialEq)]
pub struct UnitListProps {
    pub active_race: Signal<Race>,
    pub unit_mode: Signal<UnitMode>,
    pub selected_unit_id: Signal<Option<WarcraftObjectId>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub search_query: Signal<String>,
    pub search_field: Signal<SearchField>,
    pub show_abilityless_units: Signal<bool>,
    pub expand_variants: Signal<bool>,
    pub collapsed_categories: Signal<HashSet<UnitKind>>,
}

/// The values the search box's keydown handler captures: the immediate query it
/// reads, the clear handler, the focus coordinator, the first result to select on
/// Enter, and the selection signals it writes.
pub(super) struct SearchKeydownInputs {
    pub(super) raw_query: Signal<String>,
    pub(super) on_clear: EventHandler<()>,
    pub(super) focus: FocusCoordinator,
    pub(super) first_result_id: Option<WarcraftObjectId>,
    pub(super) first_result_kind: Option<UnitKind>,
    pub(super) selected_unit_id: Signal<Option<WarcraftObjectId>>,
    pub(super) selected_slot: Signal<Option<GridSlotId>>,
    pub(super) active_category: Signal<UnitKind>,
}

/// The unit list's shaped view: the data attributes for the panel, and the
/// finished props for every child (the two toggles, the search box, the mobile
/// category tabs, and the category sections).
pub(super) struct UnitListModel {
    pub(super) active_category_attr: &'static str,
    pub(super) search_active: bool,
    pub(super) search_field_toggle: SearchFieldToggleProps,
    pub(super) catalog_visibility_toggle: CatalogVisibilityToggleProps,
    pub(super) search: UnitListSearchProps,
    pub(super) mobile_tabs: Vec<MobileCategoryTabProps>,
    pub(super) sections: Vec<UnitCategorySectionProps>,
}

/// Every computed intermediate the unit list's child props are built from. The
/// hook wires the signals, the derived catalog state, and the two shaped handlers
/// into one of these; the whole child-props tree then derives itself through the
/// `From` impl below, so the hook never assembles a props struct by hand.
pub(super) struct UnitListInputs {
    pub(super) state: UnitListState,
    pub(super) race: Race,
    pub(super) mode: UnitMode,
    pub(super) current_search_field: SearchField,
    pub(super) visibility: CatalogVisibility,
    pub(super) selected_unit_id: Signal<Option<WarcraftObjectId>>,
    pub(super) selected_slot: Signal<Option<GridSlotId>>,
    pub(super) collapsed_categories: Signal<HashSet<UnitKind>>,
    pub(super) search_field: Signal<SearchField>,
    pub(super) show_abilityless_units: Signal<bool>,
    pub(super) expand_variants: Signal<bool>,
    pub(super) search_active: bool,
    pub(super) active_kind: UnitKind,
    pub(super) active_category: Signal<UnitKind>,
    pub(super) raw_query: Signal<String>,
    pub(super) search_placeholder: &'static str,
    pub(super) on_input: EventHandler<FormEvent>,
    pub(super) on_keydown: EventHandler<KeyboardEvent>,
}

impl From<UnitListInputs> for UnitListModel {
    fn from(inputs: UnitListInputs) -> Self {
        let UnitListInputs {
            state,
            race,
            mode,
            current_search_field,
            visibility,
            selected_unit_id,
            selected_slot,
            collapsed_categories,
            search_field,
            show_abilityless_units,
            expand_variants,
            search_active,
            active_kind,
            active_category,
            raw_query,
            search_placeholder,
            on_input,
            on_keydown,
        } = inputs;
        let active_category_attr = super::unit_kind_data_attr(active_kind);
        let search_field_toggle = SearchFieldToggleProps { search_field };
        let catalog_visibility_toggle = CatalogVisibilityToggleProps {
            show_abilityless_units,
            expand_variants,
        };
        let search_value: ReadSignal<String> = raw_query.into();
        let search_box = UnitListSearchProps {
            value: search_value,
            placeholder: search_placeholder,
            on_input,
            on_keydown,
        };
        let mobile_tabs = MOBILE_CATEGORY_ORDER
            .iter()
            .map(|&kind| {
                let is_active = kind == active_kind;
                MobileCategoryTabProps {
                    kind,
                    is_active,
                    race,
                    active_category,
                }
            })
            .collect();
        let sections = state
            .category_kinds()
            .iter()
            .map(|&kind| {
                let category_label = UnitKindHelpers::category_label(kind).to_owned();
                let is_collapsed = state.collapsed_snapshot().contains(&kind);
                let query = state.query_snapshot().to_owned();
                let active_unit_id = state.active_unit_id();
                UnitCategorySectionProps {
                    category_kind: kind,
                    category_label,
                    is_collapsed,
                    collapsed_categories,
                    race,
                    mode,
                    query,
                    search_field: current_search_field,
                    visibility,
                    active_unit_id,
                    selected_unit_id,
                    selected_slot,
                    active_category,
                }
            })
            .collect();
        Self {
            active_category_attr,
            search_active,
            search_field_toggle,
            catalog_visibility_toggle,
            search: search_box,
            mobile_tabs,
            sections,
        }
    }
}
