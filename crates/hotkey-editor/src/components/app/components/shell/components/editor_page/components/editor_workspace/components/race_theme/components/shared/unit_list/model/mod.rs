use dioxus::prelude::*;
use warcraft_api::UnitCatalogGroup;
use warcraft_api::UnitKind;
use warcraft_keybinds::GridSlotId;

use super::state::{FirstResult, UnitListState};
use crate::services::navigation::view_navigation::ViewNavigationContext;

pub(super) const MOBILE_CATEGORY_ORDER: [UnitKind; 4] = [
    UnitKind::Hero,
    UnitKind::Soldier,
    UnitKind::Worker,
    UnitKind::Building,
];

pub(super) struct SearchKeydownInputs {
    pub(super) raw_query: Signal<String>,
    pub(super) on_clear: EventHandler<()>,
    pub(super) first_result: Option<FirstResult>,
    pub(super) navigation: ViewNavigationContext,
    pub(super) selected_slot: Signal<Option<GridSlotId>>,
    pub(super) active_category: Signal<UnitKind>,
}

pub(super) struct UnitListModel {
    pub(super) search_value: ReadSignal<String>,
    pub(super) search_placeholder: &'static str,
    pub(super) on_input: EventHandler<FormEvent>,
    pub(super) on_keydown: EventHandler<KeyboardEvent>,
    pub(super) mobile_categories: Vec<UnitKind>,
    pub(super) groups: Vec<UnitCatalogGroup>,
}

pub(super) struct UnitListInputs {
    pub(super) state: UnitListState,
    pub(super) raw_query: Signal<String>,
    pub(super) search_placeholder: &'static str,
    pub(super) on_input: EventHandler<FormEvent>,
    pub(super) on_keydown: EventHandler<KeyboardEvent>,
}

impl From<UnitListInputs> for UnitListModel {
    fn from(inputs: UnitListInputs) -> Self {
        let UnitListInputs {
            state,
            raw_query,
            search_placeholder,
            on_input,
            on_keydown,
        } = inputs;
        let search_value: ReadSignal<String> = raw_query.into();
        let mobile_categories = MOBILE_CATEGORY_ORDER.to_vec();
        let groups = state.groups().to_vec();
        Self {
            search_value,
            search_placeholder,
            on_input,
            on_keydown,
            mobile_categories,
            groups,
        }
    }
}
