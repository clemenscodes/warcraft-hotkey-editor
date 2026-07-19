use super::data;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::search_dialog::components::search_dialog_body::components::search_dialog_filters::components::shared::segmented_control::SegmentChoice;
use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;
use warcraft_api::SearchField;

pub(super) fn use_find_units_by_group() -> Vec<SegmentChoice> {
    let mut search_field = use_editor_state().search_field();
    let current = *search_field.read();
    let name_active = current == SearchField::UnitName;
    let ability_active = current == SearchField::Ability;
    let on_name = EventHandler::new(move |_event: MouseEvent| {
        search_field.set(SearchField::UnitName);
    });
    let on_ability = EventHandler::new(move |_event: MouseEvent| {
        search_field.set(SearchField::Ability);
    });
    let name_choice = SegmentChoice {
        key: "name",
        label: data::NAME,
        is_active: name_active,
        on_pick: on_name,
    };
    let ability_choice = SegmentChoice {
        key: "ability",
        label: data::ABILITY,
        is_active: ability_active,
        on_pick: on_ability,
    };
    vec![name_choice, ability_choice]
}
