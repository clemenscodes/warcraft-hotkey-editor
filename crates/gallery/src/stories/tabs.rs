use dioxus::prelude::*;
use dioxus_gallery::Story;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::mode_and_race_tabs::ModeAndRaceTabs;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::mode_and_race_tabs::components::race_tabs::RaceTabs;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::mode_and_race_tabs::components::race_tabs::components::human_race_tab::HumanRaceTab;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::mode_and_race_tabs::components::race_tabs::components::orc_race_tab::OrcRaceTab;
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
        Story::new("Tabs", "HumanRaceTab", "Active", human_race_tab_active),
        Story::new("Tabs", "OrcRaceTab", "Inactive", orc_race_tab_inactive),
    ]
}

fn mode_and_race_tabs_melee_human() -> Element {
    let unit_mode = use_signal(|| UnitMode::Melee);
    let active_race = use_signal(|| Race::Human);
    let selected_unit_id = use_signal(|| None::<String>);
    let selected_slot = use_signal(|| None::<GridSlotId>);
    rsx! {
        ModeAndRaceTabs {
            unit_mode,
            active_race,
            selected_unit_id,
            selected_slot,
        }
    }
}

fn mode_and_race_tabs_campaign_orc() -> Element {
    let unit_mode = use_signal(|| UnitMode::Campaign);
    let active_race = use_signal(|| Race::Orc);
    let selected_unit_id = use_signal(|| None::<String>);
    let selected_slot = use_signal(|| None::<GridSlotId>);
    rsx! {
        ModeAndRaceTabs {
            unit_mode,
            active_race,
            selected_unit_id,
            selected_slot,
        }
    }
}

fn race_tabs_human_active() -> Element {
    let active_race = use_signal(|| Race::Human);
    let unit_mode = use_signal(|| UnitMode::Melee);
    let selected_unit_id = use_signal(|| None::<String>);
    let selected_slot = use_signal(|| None::<GridSlotId>);
    rsx! {
        RaceTabs {
            active_race,
            unit_mode,
            selected_unit_id,
            selected_slot,
        }
    }
}

fn human_race_tab_active() -> Element {
    let active_race = use_signal(|| Race::Human);
    let unit_mode = use_signal(|| UnitMode::Melee);
    let selected_unit_id = use_signal(|| None::<String>);
    let selected_slot = use_signal(|| None::<GridSlotId>);
    rsx! {
        HumanRaceTab {
            active_race,
            unit_mode,
            selected_unit_id,
            selected_slot,
        }
    }
}

fn orc_race_tab_inactive() -> Element {
    let active_race = use_signal(|| Race::Human);
    let unit_mode = use_signal(|| UnitMode::Melee);
    let selected_unit_id = use_signal(|| None::<String>);
    let selected_slot = use_signal(|| None::<GridSlotId>);
    rsx! {
        OrcRaceTab {
            active_race,
            unit_mode,
            selected_unit_id,
            selected_slot,
        }
    }
}
