use dioxus::prelude::*;
use warcraft_api::UnitKind;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

use super::state::{FirstResult, UnitListState};

/// The categories the mobile tab bar shows, in display order.
pub(super) const MOBILE_CATEGORY_ORDER: [UnitKind; 4] = [
    UnitKind::Hero,
    UnitKind::Soldier,
    UnitKind::Worker,
    UnitKind::Building,
];

/// The values the search box's keydown handler captures: the immediate query it
/// reads, the clear handler, the first result to select on Enter, and the selection
/// signals it writes.
pub(super) struct SearchKeydownInputs {
    pub(super) raw_query: Signal<String>,
    pub(super) on_clear: EventHandler<()>,
    pub(super) first_result: Option<FirstResult>,
    pub(super) selected_unit_id: Signal<Option<WarcraftObjectId>>,
    pub(super) selected_slot: Signal<Option<GridSlotId>>,
    pub(super) active_category: Signal<UnitKind>,
}

/// The unit list's shaped view: the search box's bound value, placeholder, and
/// handlers; the mobile tab row's category kinds; and the scroll region's category
/// kinds — all as domain values. The two toggles read their own context, so they are
/// rendered without any of this.
pub(super) struct UnitListModel {
    pub(super) search_value: ReadSignal<String>,
    pub(super) search_placeholder: &'static str,
    pub(super) on_input: EventHandler<FormEvent>,
    pub(super) on_keydown: EventHandler<KeyboardEvent>,
    pub(super) mobile_categories: Vec<UnitKind>,
    pub(super) category_kinds: Vec<UnitKind>,
}

/// Every computed intermediate the unit list's shaped view is built from. The hook
/// wires the derived catalog state and the two shaped handlers into one of these; the
/// model derives itself through the `From` impl below, so the hook never assembles the
/// view by hand.
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
        let category_kinds = state.category_kinds().to_vec();
        Self {
            search_value,
            search_placeholder,
            on_input,
            on_keydown,
            mobile_categories,
            category_kinds,
        }
    }
}
