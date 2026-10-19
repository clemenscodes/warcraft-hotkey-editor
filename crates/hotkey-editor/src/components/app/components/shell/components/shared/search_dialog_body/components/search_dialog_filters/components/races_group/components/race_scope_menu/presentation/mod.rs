use crate::components::app::components::shell::components::shared::exit_presence::use_exit_presence;
use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;
use warcraft_api::{AllRaces, RaceSelection};

pub(super) struct RaceScopeMenuPresentation {
    pub(super) summary: String,
    pub(super) is_open: bool,
    pub(super) is_mounted: bool,
    pub(super) is_leaving: bool,
    pub(super) toggle: EventHandler<MouseEvent>,
    pub(super) dismiss: EventHandler<MouseEvent>,
}

pub(super) fn use_race_scope_menu() -> RaceScopeMenuPresentation {
    let scope_signal = use_editor_state().search_race_scope();
    let scope = scope_signal.read().clone();
    let summary = match &scope {
        RaceSelection::All => "All races".to_owned(),
        RaceSelection::Only { .. } => {
            let names: Vec<String> = AllRaces::ALL
                .iter()
                .filter(|race| scope.admits(Some(*race)))
                .map(|race| format!("{race}"))
                .collect();
            if names.is_empty() {
                "None".to_owned()
            } else {
                names.join(", ")
            }
        }
    };
    let mut panel = use_exit_presence();
    let presence = panel.presence();
    let is_open = presence.is_shown();
    let is_mounted = presence.is_mounted();
    let is_leaving = presence.is_leaving();
    let toggle = EventHandler::new(move |_event: MouseEvent| panel.toggle());
    let dismiss = EventHandler::new(move |_event: MouseEvent| panel.close());
    RaceScopeMenuPresentation {
        summary,
        is_open,
        is_mounted,
        is_leaving,
        toggle,
        dismiss,
    }
}
