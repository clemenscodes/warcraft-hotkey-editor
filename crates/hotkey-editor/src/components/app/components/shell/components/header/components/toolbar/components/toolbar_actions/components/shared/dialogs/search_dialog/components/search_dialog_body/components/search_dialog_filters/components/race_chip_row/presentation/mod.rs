use std::collections::BTreeSet;

use dioxus::prelude::*;
use warcraft_api::{AllRaces, Race, RaceSelection};

use crate::services::editor_state::context::use_editor_state;

pub(super) struct RaceChoice {
    pub(super) race: Race,
    pub(super) is_active: bool,
    pub(super) on_pick: EventHandler<MouseEvent>,
}

/// The race chips are the search scope, not navigation. Each one toggles whether
/// the search covers that race, writing the `search_race_scope` editor signal.
/// The default scope is every race, so every chip starts active. The last active
/// race stays on, a floor like the mode toggles, because a search over no race
/// lists nothing. Navigating to a race and editing it happens by tapping a
/// result, not from a chip.
pub(super) fn use_race_chip_row() -> Vec<RaceChoice> {
    let editor = use_editor_state();
    let scope_signal = editor.search_race_scope();
    let current_scope = scope_signal.read().clone();
    AllRaces::ALL
        .iter()
        .map(|race| {
            let is_active = current_scope.admits(Some(race));
            let mut scope_signal = scope_signal;
            let on_pick = EventHandler::new(move |_event: MouseEvent| {
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
            });
            RaceChoice {
                race,
                is_active,
                on_pick,
            }
        })
        .collect()
}
