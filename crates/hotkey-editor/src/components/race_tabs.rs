use dioxus::prelude::*;
use warcraft_api::Race;

use crate::focus::modality::FocusModality;
use crate::grid_slot::GridSlotId;
use warcraft_api::{RaceLabels, SUPPORTED_RACES};
use warcraft_database::{UnitKindHelpers, UnitMode};

#[component]
pub(crate) fn RaceTabs(
    mut active_race: Signal<Race>,
    unit_mode: Signal<UnitMode>,
    mut selected_unit_id: Signal<Option<String>>,
    mut selected_slot: Signal<Option<GridSlotId>>,
) -> Element {
    rsx! {
        nav { class: "race-tabs",
            for race in SUPPORTED_RACES.iter().copied() {
                {
                    let is_active = *active_race.read() == race;
                    let class_name = if is_active { "race-tab active" } else { "race-tab" };
                    let label_text = RaceLabels::display_name(race);
                    let race_attribute = RaceLabels::data_attribute(race);
                    rsx! {
                        button {
                            class: "{class_name}",
                            "data-race": "{race_attribute}",
                            onclick: move |_| {
                                active_race.set(race);
                                let mode_value = *unit_mode.read();
                                let next_id = UnitKindHelpers::default_unit_id_for(race, mode_value);
                                selected_unit_id.set(next_id);
                                selected_slot.set(None);
                            },
                            onkeydown: move |event| {
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
                            },
                            span { class: "race-tab-label", "{label_text}" }
                        }
                    }
                }
            }
        }
    }
}
