use dioxus::prelude::*;
use warcraft_api::UnitKind;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

use super::components::mobile_category_tab::MobileCategoryTabProps;
use super::components::unit_category_section::UnitCategorySectionProps;
use super::components::unit_list_search::UnitListSearchProps;
use super::state::{FirstResult, UnitListState};
use crate::services::focus::coordinator::FocusCoordinator;

/// The categories the mobile tab bar shows, in display order.
pub(super) const MOBILE_CATEGORY_ORDER: [UnitKind; 4] = [
    UnitKind::Hero,
    UnitKind::Soldier,
    UnitKind::Worker,
    UnitKind::Building,
];

/// The values the search box's keydown handler captures: the immediate query it
/// reads, the clear handler, the focus coordinator, the first result to select on
/// Enter, and the selection signals it writes.
pub(super) struct SearchKeydownInputs {
    pub(super) raw_query: Signal<String>,
    pub(super) on_clear: EventHandler<()>,
    pub(super) focus: FocusCoordinator,
    pub(super) first_result: Option<FirstResult>,
    pub(super) selected_unit_id: Signal<Option<WarcraftObjectId>>,
    pub(super) selected_slot: Signal<Option<GridSlotId>>,
    pub(super) active_category: Signal<UnitKind>,
}

/// The unit list's shaped view: the data attributes for the panel, and the finished
/// props for the search box, the mobile category tabs, and the category sections. The
/// two toggles read their own context, so they are rendered without props.
pub(super) struct UnitListModel {
    pub(super) active_category_attr: &'static str,
    pub(super) search_active: bool,
    pub(super) search: UnitListSearchProps,
    pub(super) mobile_tabs: Vec<MobileCategoryTabProps>,
    pub(super) sections: Vec<UnitCategorySectionProps>,
}

/// Every computed intermediate the unit list's child props are built from. The hook
/// wires the derived catalog state and the two shaped handlers into one of these; the
/// child-props tree then derives itself through the `From` impl below, so the hook
/// never assembles a props struct by hand.
pub(super) struct UnitListInputs {
    pub(super) state: UnitListState,
    pub(super) active_kind: UnitKind,
    pub(super) search_active: bool,
    pub(super) raw_query: Signal<String>,
    pub(super) search_placeholder: &'static str,
    pub(super) on_input: EventHandler<FormEvent>,
    pub(super) on_keydown: EventHandler<KeyboardEvent>,
}

impl From<UnitListInputs> for UnitListModel {
    fn from(inputs: UnitListInputs) -> Self {
        let UnitListInputs {
            state,
            active_kind,
            search_active,
            raw_query,
            search_placeholder,
            on_input,
            on_keydown,
        } = inputs;
        let active_category_attr = super::unit_kind_data_attr(active_kind);
        let search_value: ReadSignal<String> = raw_query.into();
        let search = UnitListSearchProps {
            value: search_value,
            placeholder: search_placeholder,
            on_input,
            on_keydown,
        };
        let mobile_tabs = MOBILE_CATEGORY_ORDER
            .iter()
            .map(|&kind| MobileCategoryTabProps { kind })
            .collect();
        let sections = state
            .category_kinds()
            .iter()
            .map(|&category_kind| UnitCategorySectionProps { category_kind })
            .collect();
        Self {
            active_category_attr,
            search_active,
            search,
            mobile_tabs,
            sections,
        }
    }
}
