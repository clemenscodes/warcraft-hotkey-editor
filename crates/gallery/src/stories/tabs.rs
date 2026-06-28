use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::{ModeAndRaceTabs, RaceTab, RaceTabs};
use warcraft_api::Race;
use warcraft_database::UnitMode;
use warcraft_keybinds::GridSlotId;

pub fn stories() -> Vec<Story> {
    vec![
        Story::new(
            "Tabs",
            "ModeAndRaceTabs",
            "Melee / human",
            mode_and_race_tabs_melee_human,
        ),
        Story::new(
            "Tabs",
            "ModeAndRaceTabs",
            "Campaign / orc",
            mode_and_race_tabs_campaign_orc,
        ),
        Story::single("Tabs", "RaceTabs", race_tabs_human_active),
        Story::new("Tabs", "RaceTab", "Human active", race_tab_human_active),
        Story::new("Tabs", "RaceTab", "Orc inactive", race_tab_orc_inactive),
    ]
}

fn mode_and_race_tabs_melee_human() -> Element {
    let unit_mode = use_signal(|| UnitMode::Melee);
    let active_race = use_signal(|| Race::Human);
    let selected_unit_id = use_signal(|| None::<String>);
    let selected_slot = use_signal(|| None::<GridSlotId>);
    rsx! {
        ModeAndRaceTabs { unit_mode, active_race, selected_unit_id, selected_slot }
    }
}

fn mode_and_race_tabs_campaign_orc() -> Element {
    let unit_mode = use_signal(|| UnitMode::Campaign);
    let active_race = use_signal(|| Race::Orc);
    let selected_unit_id = use_signal(|| None::<String>);
    let selected_slot = use_signal(|| None::<GridSlotId>);
    rsx! {
        ModeAndRaceTabs { unit_mode, active_race, selected_unit_id, selected_slot }
    }
}

fn race_tabs_human_active() -> Element {
    let active_race = use_signal(|| Race::Human);
    let unit_mode = use_signal(|| UnitMode::Melee);
    let selected_unit_id = use_signal(|| None::<String>);
    let selected_slot = use_signal(|| None::<GridSlotId>);
    rsx! {
        RaceTabs { active_race, unit_mode, selected_unit_id, selected_slot }
    }
}

fn race_tab_human_active() -> Element {
    let race = Race::Human;
    let is_active = true;
    let active_race = use_signal(|| Race::Human);
    let unit_mode = use_signal(|| UnitMode::Melee);
    let selected_unit_id = use_signal(|| None::<String>);
    let selected_slot = use_signal(|| None::<GridSlotId>);
    rsx! {
        RaceTab { race, is_active, active_race, unit_mode, selected_unit_id, selected_slot }
    }
}

fn race_tab_orc_inactive() -> Element {
    let race = Race::Orc;
    let is_active = false;
    let active_race = use_signal(|| Race::Human);
    let unit_mode = use_signal(|| UnitMode::Melee);
    let selected_unit_id = use_signal(|| None::<String>);
    let selected_slot = use_signal(|| None::<GridSlotId>);
    rsx! {
        RaceTab { race, is_active, active_race, unit_mode, selected_unit_id, selected_slot }
    }
}
