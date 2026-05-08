use dioxus::prelude::*;
use warcraft_api::{Race, RaceLabels};
use warcraft_database::{UnitKindHelpers, UnitMode};

use crate::model::grid::GridSlotId;
use crate::services::focus::modality::FocusModality;

use super::state::RaceTabClasses;

#[component]
pub(super) fn RaceTab(
    race: Race,
    is_active: bool,
    mut active_race: Signal<Race>,
    unit_mode: Signal<UnitMode>,
    mut selected_unit_id: Signal<Option<String>>,
    mut selected_slot: Signal<Option<GridSlotId>>,
) -> Element {
    let classes = RaceTabClasses::for_race(race);
    let button_class = classes.button_class();
    let label_class = classes.label_class();
    let label_text = RaceLabels::display_name(race);
    let race_attribute = RaceLabels::data_attribute(race);
    let handle_click = move |_| {
        active_race.set(race);
        let mode_value = *unit_mode.read();
        let next_id = UnitKindHelpers::default_unit_id_for(race, mode_value);
        selected_unit_id.set(next_id);
        selected_slot.set(None);
    };
    let handle_keydown = move |event: Event<KeyboardData>| {
        let key_value = event.data().key().to_string();
        if key_value == " " || key_value == "Enter" {
            event.prevent_default();
            active_race.set(race);
            let mode_value = *unit_mode.read();
            let next_id = UnitKindHelpers::default_unit_id_for(race, mode_value);
            selected_unit_id.set(next_id);
            selected_slot.set(None);
            FocusModality::after_render(".unit-card.selected, .unit-card");
        }
    };

    rsx! {
        button {
            class: button_class,
            "data-race": race_attribute,
            "data-active": is_active,
            onclick: handle_click,
            onkeydown: handle_keydown,
            span { class: label_class, "{label_text}" }
        }
    }
}
