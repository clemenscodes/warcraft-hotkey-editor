use dioxus::prelude::*;
use warcraft_api::{AllRaces, Race};

use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use crate::services::unit_catalog::context::use_unit_catalog;

pub(super) struct RaceChoice {
    pub(super) race: Race,
    pub(super) is_active: bool,
    pub(super) on_pick: EventHandler<MouseEvent>,
}

/// `select_race` resolves the race's default unit and pushes it, so a chip is the
/// jump as well as the filter. On a phone it is the only race switch there is:
/// the pager is one flat list of every unit in the game and the race tabs live in
/// the desktop tree, which is not mounted below 768px.
///
/// While a search runs the domain drops the race entirely — a hit may be any race
/// — so no chip claims to be filtering by one.
pub(super) fn use_race_chip_row() -> Vec<RaceChoice> {
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let catalog = use_unit_catalog();
    let selected_slot = editor.selected_slot();
    let active_race = navigation.active_race();
    let current_race = *active_race.read();
    let is_searching = catalog.filter().is_searching();
    let all_races = AllRaces::ALL;
    all_races
        .iter()
        .map(|race| {
            let is_active = !is_searching && race == current_race;
            let on_pick = EventHandler::new(move |_event: MouseEvent| {
                navigation.select_race(race, selected_slot);
            });
            RaceChoice {
                race,
                is_active,
                on_pick,
            }
        })
        .collect()
}
