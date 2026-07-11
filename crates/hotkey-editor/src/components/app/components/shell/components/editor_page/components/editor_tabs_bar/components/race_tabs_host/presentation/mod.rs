use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_api::Race;

/// The two inputs the presentational `RaceTabs` needs: the active race (to mark the
/// current tab) and the select handler that dispatches the domain cascade. This is the
/// host's shaped domain data — never the tabs' own props type.
pub(super) struct RaceTabsInputs {
    pub(super) active_race: Signal<Race>,
    pub(super) on_select: EventHandler<Race>,
}

/// The seam: source the active race, and wire the select handler to the navigation
/// service's `select_race` cascade (set race → default unit → clear slot). The
/// presentational `RaceTabs` gets only those two inputs; the cascade never enters it.
pub(super) fn use_race_tabs_host() -> RaceTabsInputs {
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let active_race = navigation.active_race();
    let selected_slot = editor.selected_slot();
    let on_select = EventHandler::new(move |race: Race| {
        navigation.select_race(race, selected_slot);
    });
    RaceTabsInputs {
        active_race,
        on_select,
    }
}
