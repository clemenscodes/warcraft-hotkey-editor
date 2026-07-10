use dioxus::prelude::*;
use dioxus_gallery::Story;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::components::shared::race_tab_state::components::active_race_tab::{ActiveRaceTab, ActiveRaceTabProps};
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::components::shared::race_tab_state::components::inactive_race_tab::{InactiveRaceTab, InactiveRaceTabProps};
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::components::shared::race_tab_state::components::shared::race_tab::components::race_tab_label::RaceTabLabelProps;

pub fn stories() -> Vec<Story> {
    vec![
        Story::new("Tabs", "RaceTab", "Active", active_race_tab_story),
        Story::new("Tabs", "RaceTab", "Inactive", inactive_race_tab_story),
    ]
}

fn active_race_tab_story() -> Element {
    let label = RaceTabLabelProps {
        label: "Human".to_string(),
    };
    let onclick = EventHandler::new(|_event: MouseEvent| {});
    let onkeydown = EventHandler::new(|_event: KeyboardEvent| {});
    let props = ActiveRaceTabProps {
        label,
        onclick,
        onkeydown,
    };
    rsx! {
        ActiveRaceTab { ..props }
    }
}

fn inactive_race_tab_story() -> Element {
    let label = RaceTabLabelProps {
        label: "Orc".to_string(),
    };
    let onclick = EventHandler::new(|_event: MouseEvent| {});
    let onkeydown = EventHandler::new(|_event: KeyboardEvent| {});
    let props = InactiveRaceTabProps {
        label,
        onclick,
        onkeydown,
    };
    rsx! {
        InactiveRaceTab { ..props }
    }
}
