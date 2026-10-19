use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_api::Race;

pub(super) struct RaceTabsHostPresentation {
    pub(super) active_race: Race,
    pub(super) on_select: EventHandler<Race>,
}

pub(super) fn use_race_tabs_host() -> RaceTabsHostPresentation {
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let active_race = *navigation.active_race().read();
    let selected_slot = editor.selected_slot();
    let on_select = EventHandler::new(move |race: Race| {
        navigation.select_race(race, selected_slot);
    });
    RaceTabsHostPresentation {
        active_race,
        on_select,
    }
}
