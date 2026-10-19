use std::collections::BTreeSet;

use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;
use warcraft_api::{AllRaces, Race, RaceSelection};

pub(super) struct RaceBannerChoice {
    pub(super) key: String,
    pub(super) race: Race,
    pub(super) label: String,
    pub(super) is_active: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) onkeydown: EventHandler<KeyboardEvent>,
}

pub(super) fn use_race_scope_panel() -> Vec<RaceBannerChoice> {
    let editor = use_editor_state();
    let scope_signal = editor.search_race_scope();
    let current_scope = scope_signal.read().clone();
    AllRaces::ALL
        .iter()
        .map(|race| {
            let is_active = current_scope.admits(Some(race));
            let toggle = move || {
                let mut scope_signal = scope_signal;
                let scope = scope_signal.peek().clone();
                let mut races: BTreeSet<Race> = match &scope {
                    RaceSelection::All => AllRaces::ALL.iter().collect(),
                    RaceSelection::Only { races } => races.clone(),
                };
                let was_present = races.contains(&race);
                if was_present {
                    races.remove(&race);
                } else {
                    races.insert(race);
                }
                if races.is_empty() {
                    return;
                }
                let total = AllRaces::ALL.iter().count();
                let next = if races.len() == total {
                    RaceSelection::All
                } else {
                    RaceSelection::Only { races }
                };
                scope_signal.set(next);
            };
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                toggle();
            });
            let onkeydown = EventHandler::new(move |event: KeyboardEvent| {
                let key = event.key().to_string();
                if key == "Enter" || key == " " {
                    event.prevent_default();
                    toggle();
                }
            });
            RaceBannerChoice {
                key: race.slug().to_owned(),
                race,
                label: format!("{race}"),
                is_active,
                onclick,
                onkeydown,
            }
        })
        .collect()
}
