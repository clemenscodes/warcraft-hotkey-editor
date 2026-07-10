use dioxus::prelude::*;
use warcraft_api::SearchField;

/// The two search-field options, each finished with its active flag and select
/// handler.
pub(super) struct SearchFieldToggleModel {
    pub(super) unit_is_active: bool,
    pub(super) ability_is_active: bool,
    pub(super) select_unit: EventHandler<MouseEvent>,
    pub(super) select_ability: EventHandler<MouseEvent>,
}

impl From<Signal<SearchField>> for SearchFieldToggleModel {
    fn from(search_field: Signal<SearchField>) -> Self {
        let mut search_field = search_field;
        let current = *search_field.read();
        let select_unit = EventHandler::new(move |_event: MouseEvent| {
            search_field.set(SearchField::UnitName);
        });
        let select_ability = EventHandler::new(move |_event: MouseEvent| {
            search_field.set(SearchField::Ability);
        });
        let unit_is_active = current == SearchField::UnitName;
        let ability_is_active = current == SearchField::Ability;
        Self {
            unit_is_active,
            ability_is_active,
            select_unit,
            select_ability,
        }
    }
}
