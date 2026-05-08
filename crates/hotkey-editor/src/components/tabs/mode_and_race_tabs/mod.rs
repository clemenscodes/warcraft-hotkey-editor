mod state;

use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_database::{UnitKindHelpers, UnitMode};

use crate::components::tabs::race_tabs::RaceTabs;
use crate::model::grid::GridSlotId;
use crate::services::focus::modality::FocusModality;

use state::ModeButtonClass;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct ModeAndRaceTabsProps {
    pub(crate) unit_mode: Signal<UnitMode>,
    pub(crate) active_race: Signal<Race>,
    pub(crate) selected_unit_id: Signal<Option<String>>,
    pub(crate) selected_slot: Signal<Option<GridSlotId>>,
}

#[component]
pub(crate) fn ModeAndRaceTabs(props: ModeAndRaceTabsProps) -> Element {
    let mut unit_mode = props.unit_mode;
    let active_race = props.active_race;
    let mut selected_unit_id = props.selected_unit_id;
    let mut selected_slot = props.selected_slot;
    let mode_snapshot = *unit_mode.read();
    let active_race_for_melee = *active_race.read();
    let active_race_for_campaign = *active_race.read();
    let button_class = ModeButtonClass::get();
    let is_melee_active = mode_snapshot == UnitMode::Melee;
    let is_campaign_active = mode_snapshot == UnitMode::Campaign;
    let select_melee = move |_| {
        unit_mode.set(UnitMode::Melee);
        let next_id = UnitKindHelpers::default_unit_id_for(active_race_for_melee, UnitMode::Melee);
        selected_unit_id.set(next_id);
        selected_slot.set(None);
    };
    let melee_keydown = move |event: Event<KeyboardData>| {
        let key_value = event.data().key().to_string();
        if key_value == " " || key_value == "Enter" {
            event.prevent_default();
            unit_mode.set(UnitMode::Melee);
            let next_id =
                UnitKindHelpers::default_unit_id_for(active_race_for_melee, UnitMode::Melee);
            selected_unit_id.set(next_id);
            selected_slot.set(None);
            FocusModality::after_render(".race-tab[data-active='true'], .race-tab");
        }
    };
    let select_campaign = move |_| {
        unit_mode.set(UnitMode::Campaign);
        let next_id =
            UnitKindHelpers::default_unit_id_for(active_race_for_campaign, UnitMode::Campaign);
        selected_unit_id.set(next_id);
        selected_slot.set(None);
    };
    let campaign_keydown = move |event: Event<KeyboardData>| {
        let key_value = event.data().key().to_string();
        if key_value == " " || key_value == "Enter" {
            event.prevent_default();
            unit_mode.set(UnitMode::Campaign);
            let next_id =
                UnitKindHelpers::default_unit_id_for(active_race_for_campaign, UnitMode::Campaign);
            selected_unit_id.set(next_id);
            selected_slot.set(None);
            FocusModality::after_render(".race-tab[data-active='true'], .race-tab");
        }
    };
    rsx! {
        div {
            class: "flex items-stretch gap-10 grow min-w-0 max-[700px]:flex-col max-[700px]:gap-[0.6rem]",
            div {
                class: "flex flex-col gap-2 self-stretch flex-[0_0_var(--sidebar-column-width)] w-[var(--sidebar-column-width)] max-[1099px]:flex-[0_0_18rem] max-[1099px]:w-72 max-[700px]:flex-row max-[700px]:flex-none max-[700px]:w-full max-[700px]:gap-[0.5rem]",
                button {
                    class: button_class,
                    "data-active": is_melee_active,
                    onclick: select_melee,
                    onkeydown: melee_keydown,
                    "Melee"
                }
                button {
                    class: button_class,
                    "data-active": is_campaign_active,
                    onclick: select_campaign,
                    onkeydown: campaign_keydown,
                    "Campaign"
                }
            }
            RaceTabs { active_race, unit_mode, selected_unit_id, selected_slot }
        }
    }
}
