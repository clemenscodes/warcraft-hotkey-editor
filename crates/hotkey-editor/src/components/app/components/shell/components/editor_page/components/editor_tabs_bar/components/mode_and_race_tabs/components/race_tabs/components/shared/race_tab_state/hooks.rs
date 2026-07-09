use super::logic::RaceSelection;
use super::props::RaceTabStateProps;
use crate::services::focus::context::use_focus_coordinator;
use crate::services::focus::coordinator::{FocusCoordinator, FocusTarget};
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::RaceLabels;

/// A race tab's finished behaviour: whether its race is the active one, its display
/// label, and the pointer/keyboard/mount handlers. The look is not here — the dispatcher
/// picks the active or inactive component from `is_active`, and each wears its own accent
/// overlay. This is the one place the click/keyboard/focus behaviour lives, so the two
/// look components stay identical in behaviour and differ only in appearance.
pub(super) struct RaceTabBinding {
    is_active: bool,
    label: String,
    onclick: EventHandler<MouseEvent>,
    onkeydown: EventHandler<KeyboardEvent>,
    onmounted: EventHandler<Event<MountedData>>,
}

impl RaceTabBinding {
    pub(super) fn is_active(&self) -> bool {
        self.is_active
    }

    pub(super) fn label(&self) -> &str {
        &self.label
    }

    pub(super) fn onclick(&self) -> EventHandler<MouseEvent> {
        self.onclick
    }

    pub(super) fn onkeydown(&self) -> EventHandler<KeyboardEvent> {
        self.onkeydown
    }

    pub(super) fn onmounted(&self) -> EventHandler<Event<MountedData>> {
        self.onmounted
    }
}

/// Wire a race tab's shared behaviour: register it as the race-tabs focus target while
/// its race is the active one, and build its activation handlers.
pub(super) fn use_race_tab(props: RaceTabStateProps) -> RaceTabBinding {
    let focus = use_focus_coordinator();
    let mut mounted_handle = use_signal(|| None::<Rc<MountedData>>);
    let race = props.race;
    let navigation = props.navigation;
    let active_race = navigation.active_race;
    use_effect(move || {
        let current_race = *active_race.read();
        if current_race == race {
            let handle = mounted_handle.read().clone();
            focus.set_race_tabs_handle(handle);
        }
    });
    let handlers = RaceTabHandlers::build(props, focus);
    let current_race = *active_race.read();
    let is_active = current_race == race;
    let display_name = RaceLabels::display_name(race);
    let label = display_name.to_string();
    let onmounted = EventHandler::new(move |event: Event<MountedData>| {
        let data = event.data();
        mounted_handle.set(Some(data));
    });
    let onclick = handlers.onclick;
    let onkeydown = handlers.onkeydown;
    RaceTabBinding {
        is_active,
        label,
        onclick,
        onkeydown,
        onmounted,
    }
}

/// The race tab's non-styling chrome: the pointer and keyboard activation handlers.
struct RaceTabHandlers {
    onclick: EventHandler<MouseEvent>,
    onkeydown: EventHandler<KeyboardEvent>,
}

impl RaceTabHandlers {
    fn build(props: RaceTabStateProps, focus: FocusCoordinator) -> Self {
        let race = props.race;
        let navigation = props.navigation;
        let selection = RaceSelection::from(&navigation);
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            selection.apply(race);
        });
        let onkeydown = EventHandler::new(move |event: KeyboardEvent| {
            let key = event.key();
            let key_value = key.to_string();
            let is_space = key_value == " ";
            let is_enter = key_value == "Enter";
            if is_space || is_enter {
                event.prevent_default();
                selection.apply(race);
                focus.request(FocusTarget::UnitCard);
            }
        });
        Self { onclick, onkeydown }
    }
}
