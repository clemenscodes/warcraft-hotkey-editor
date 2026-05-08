mod state;

use dioxus::prelude::*;
use warcraft_api::{Race, RaceLabels, SUPPORTED_RACES};
use warcraft_database::{UnitKindHelpers, UnitMode};

use crate::model::grid::GridSlotId;
use crate::services::focus::modality::FocusModality;

use state::RaceTabClasses;

#[component]
pub(crate) fn RaceTabs(
    mut active_race: Signal<Race>,
    unit_mode: Signal<UnitMode>,
    mut selected_unit_id: Signal<Option<String>>,
    mut selected_slot: Signal<Option<GridSlotId>>,
) -> Element {
    rsx! {
        nav { class: "flex gap-4 flex-nowrap w-full min-w-0 grow self-stretch",
            for race in SUPPORTED_RACES.iter().copied() {
                {
                    let is_active = *active_race.read() == race;
                    let classes = RaceTabClasses::for_race(race);
                    let label_text = RaceLabels::display_name(race);
                    let race_attribute = RaceLabels::data_attribute(race);
                    rsx! {
                        button {
                            key: "{race_attribute}",
                            class: "{classes.button_class()}",
                            "data-race": "{race_attribute}",
                            "data-active": "{is_active}",
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
                            span { class: "{classes.label_class()}", "{label_text}" }
                        }
                    }
                }
            }
        }
    }
}
