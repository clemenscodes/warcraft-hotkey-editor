use super::components::race_tabs::RaceTabsProps;
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_api::Race;

/// The seam: source the active race, and wire the select handler to the navigation
/// service's `select_race` cascade (set race → default unit → clear slot). The
/// presentational `RaceTabs` gets only those two inputs; the cascade never enters it.
pub(super) fn use_race_tabs_host() -> RaceTabsProps {
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let active_race = navigation.active_race();
    let selected_slot = editor.selected_slot();
    let on_select = EventHandler::new(move |race: Race| {
        navigation.select_race(race, selected_slot);
    });
    RaceTabsProps {
        active_race,
        on_select,
    }
}
