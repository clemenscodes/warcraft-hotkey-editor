use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;
use warcraft_api::{AllRaces, RaceSelection};

pub(super) struct RaceScopeMenuPresentation {
    pub(super) summary: String,
    pub(super) is_open: bool,
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
    let mut open = use_signal::<bool>(|| false);
    let is_open = *open.read();
    let toggle = EventHandler::new(move |_event: MouseEvent| {
        let next = !*open.peek();
        open.set(next);
    });
    let dismiss = EventHandler::new(move |_event: MouseEvent| open.set(false));
    RaceScopeMenuPresentation {
        summary,
        is_open,
        toggle,
        dismiss,
    }
}
