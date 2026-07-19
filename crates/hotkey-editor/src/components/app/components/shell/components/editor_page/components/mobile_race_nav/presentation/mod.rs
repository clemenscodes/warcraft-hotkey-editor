use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_api::{AllRaces, Race};

pub(super) struct RaceNavItem {
    pub(super) key: String,
    pub(super) race: Race,
    pub(super) label: String,
    pub(super) is_active: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) onkeydown: EventHandler<KeyboardEvent>,
}

pub(super) fn use_mobile_race_nav_items() -> Vec<RaceNavItem> {
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let active_race = navigation.active_race();
    let selected_slot = editor.selected_slot();
    let current_race = *active_race.read();
    let on_select = EventHandler::new(move |race: Race| {
        navigation.select_race(race, selected_slot);
    });
    AllRaces::ALL
        .iter()
        .map(|race| {
            let is_active = race == current_race;
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                on_select.call(race);
            });
            let onkeydown = EventHandler::new(move |event: KeyboardEvent| {
                let key = event.key().to_string();
                if key == "Enter" || key == " " {
                    event.prevent_default();
                    on_select.call(race);
                }
            });
            RaceNavItem {
                key: race.slug().to_owned(),
                race,
                label: race.to_string(),
                is_active,
                onclick,
                onkeydown,
            }
        })
        .collect()
}
