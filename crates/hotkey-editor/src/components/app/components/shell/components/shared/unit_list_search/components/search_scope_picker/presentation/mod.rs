use super::data;
use super::model::SearchScopePickerModel;
use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;
use warcraft_api::SearchField;

pub(super) struct SearchScopePickerPresentation {
    pub(super) is_open: bool,
    pub(super) current_label: String,
    pub(super) toggle: EventHandler<MouseEvent>,
    pub(super) dismiss: EventHandler<MouseEvent>,
    pub(super) unit_is_active: bool,
    pub(super) ability_is_active: bool,
    pub(super) select_unit: EventHandler<MouseEvent>,
    pub(super) select_ability: EventHandler<MouseEvent>,
}

pub(super) fn use_search_scope_picker() -> SearchScopePickerPresentation {
    let mut open = use_signal::<bool>(|| false);
    let mut search_field = use_editor_state().search_field();
    let current = *search_field.read();
    let is_open = *open.read();
    let current_label = match current {
        SearchField::UnitName => data::UNIT,
        SearchField::Ability => data::ABILITY,
    }
    .to_owned();
    let toggle = EventHandler::new(move |_event: MouseEvent| {
        let next = !open();
        open.set(next);
    });
    let dismiss = EventHandler::new(move |_event: MouseEvent| open.set(false));
    let select_unit = EventHandler::new(move |_event: MouseEvent| {
        search_field.set(SearchField::UnitName);
        open.set(false);
    });
    let select_ability = EventHandler::new(move |_event: MouseEvent| {
        search_field.set(SearchField::Ability);
        open.set(false);
    });
    let unit_is_active = current == SearchField::UnitName;
    let ability_is_active = current == SearchField::Ability;
    SearchScopePickerPresentation {
        is_open,
        current_label,
        toggle,
        dismiss,
        unit_is_active,
        ability_is_active,
        select_unit,
        select_ability,
    }
}

impl ddd::Presentation for SearchScopePickerPresentation {
    type Model = SearchScopePickerModel;
}
