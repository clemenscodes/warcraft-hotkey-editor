use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_api::Race;

pub(super) struct RaceTabsInputs {
    pub(super) active_race: Signal<Race>,
    pub(super) on_select: EventHandler<Race>,
}

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
