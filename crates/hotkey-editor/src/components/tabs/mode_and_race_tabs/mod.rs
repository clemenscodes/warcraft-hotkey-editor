mod state;

use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_database::{UnitKindHelpers, UnitMode};

use crate::components::tabs::race_tabs::RaceTabs;
use crate::model::grid::GridSlotId;
use crate::services::focus::modality::FocusModality;

use state::ModeButtonClass;

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
    let button_class = ModeButtonClass::get();
    let is_melee_active = mode_snapshot == UnitMode::Melee;
    let is_campaign_active = mode_snapshot == UnitMode::Campaign;
    rsx! {
        div {
            class: "flex items-stretch gap-10 grow min-w-0 max-[700px]:flex-col max-[700px]:gap-[0.6rem]",
            div {
                class: "flex flex-col gap-2 self-stretch flex-[0_0_var(--sidebar-column-width)] w-[var(--sidebar-column-width)] max-[1099px]:flex-[0_0_18rem] max-[1099px]:w-72 max-[700px]:flex-row max-[700px]:flex-none max-[700px]:w-full max-[700px]:gap-[0.5rem]",
                button {
                    class: "{button_class}",
                    "data-active": "{is_melee_active}",
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
                            FocusModality::after_render(".race-tab[data-active='true'], .race-tab");
                        }
                    },
                    "Melee"
                }
                button {
                    class: "{button_class}",
                    "data-active": "{is_campaign_active}",
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
                            FocusModality::after_render(".race-tab[data-active='true'], .race-tab");
                        }
                    },
                    "Campaign"
                }
            }
            RaceTabs { active_race, unit_mode, selected_unit_id, selected_slot }
        }
    }
}
