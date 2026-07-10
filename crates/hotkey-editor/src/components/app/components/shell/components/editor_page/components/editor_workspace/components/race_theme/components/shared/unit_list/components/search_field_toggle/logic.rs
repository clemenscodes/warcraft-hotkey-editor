use super::components::search_field_button::SearchFieldButtonProps;
use super::data;
use dioxus::prelude::*;
use warcraft_api::SearchField;

/// The two search-field options, each finished with its active flag and select
/// handler.
pub(super) struct SearchFieldToggleModel {
    pub(super) unit_button: SearchFieldButtonProps,
    pub(super) ability_button: SearchFieldButtonProps,
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
        let unit_button = SearchFieldButtonProps {
            label: data::UNIT,
            is_active: current == SearchField::UnitName,
            on_select: select_unit,
        };
        let ability_button = SearchFieldButtonProps {
            label: data::ABILITY,
            is_active: current == SearchField::Ability,
            on_select: select_ability,
        };
        Self {
            unit_button,
            ability_button,
        }
    }
}
