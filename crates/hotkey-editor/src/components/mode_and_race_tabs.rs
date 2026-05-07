use dioxus::prelude::*;
use warcraft_api::Race;

use crate::components::race_tabs::RaceTabs;
use crate::focus::modality::FocusModality;
use crate::grid_slot::GridSlotId;
use warcraft_database::{UnitKindHelpers, UnitMode};

#[component]
pub(crate) fn ModeAndRaceTabs(
    mut unit_mode: Signal<UnitMode>,
    active_race: Signal<Race>,
    mut selected_unit_id: Signal<Option<String>>,
    mut selected_slot: Signal<Option<GridSlotId>>,
) -> Element {
    let mode_snapshot = *unit_mode.read();
    let active_race_for_melee = *active_race.read();
    let active_race_for_campaign = *active_race.read();
    rsx! {
        div { class: "mode-strip",
            div { class: "mode-toggle",
                button {
                    class: if mode_snapshot == UnitMode::Melee { "mode-toggle-button active" } else { "mode-toggle-button" },
                    onclick: move |_| {
                        unit_mode.set(UnitMode::Melee);
                        let next_id = UnitKindHelpers::default_unit_id_for(active_race_for_melee, UnitMode::Melee);
                        selected_unit_id.set(next_id);
                        selected_slot.set(None);
                    },
                    onkeydown: move |event| {
                        let key_value = event.data().key().to_string();
                        if key_value == " " || key_value == "Enter" {
                            event.prevent_default();
                            unit_mode.set(UnitMode::Melee);
                            let next_id = UnitKindHelpers::default_unit_id_for(active_race_for_melee, UnitMode::Melee);
                            selected_unit_id.set(next_id);
                            selected_slot.set(None);
                            FocusModality::after_render(".race-tab.active, .race-tab");
                        }
                    },
                    "Melee"
                }
                button {
                    class: if mode_snapshot == UnitMode::Campaign { "mode-toggle-button active" } else { "mode-toggle-button" },
                    onclick: move |_| {
                        unit_mode.set(UnitMode::Campaign);
                        let next_id = UnitKindHelpers::default_unit_id_for(active_race_for_campaign, UnitMode::Campaign);
                        selected_unit_id.set(next_id);
                        selected_slot.set(None);
                    },
                    onkeydown: move |event| {
                        let key_value = event.data().key().to_string();
                        if key_value == " " || key_value == "Enter" {
                            event.prevent_default();
                            unit_mode.set(UnitMode::Campaign);
                            let next_id = UnitKindHelpers::default_unit_id_for(active_race_for_campaign, UnitMode::Campaign);
                            selected_unit_id.set(next_id);
                            selected_slot.set(None);
                            FocusModality::after_render(".race-tab.active, .race-tab");
                        }
                    },
                    "Campaign"
                }
            }
            RaceTabs { active_race, unit_mode, selected_unit_id, selected_slot }
        }
    }
}
