use super::components::shared::race_tab::components::race_tab_label::RaceTabLabelProps;
use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::RaceTabBinding;
use crate::services::focus::context::use_focus_coordinator;
use dioxus::prelude::*;
use std::rc::Rc;

/// A race tab's finished behaviour: whether it is the active tab, its label child props,
/// and the pointer/keyboard/mount handlers. The click and keyboard handlers arrive
/// already baked in the `RaceTabBinding` (the cascade lives behind them); this hook adds
/// only the mount handler and the focus registration, which are per-tab DOM concerns and
/// so cannot be built up in the parent. The look picks the active-or-inactive component
/// from `is_active`.
pub(super) struct RaceTabBehavior {
    is_active: bool,
    label: RaceTabLabelProps,
    onclick: EventHandler<MouseEvent>,
    onkeydown: EventHandler<KeyboardEvent>,
    onmounted: EventHandler<Event<MountedData>>,
}

impl RaceTabBehavior {
    pub(super) fn is_active(&self) -> bool {
        self.is_active
    }

    pub(super) fn label(&self) -> &RaceTabLabelProps {
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

/// Add a tab's per-DOM behaviour to its baked binding: register it as the race-tabs
/// focus target while it is the active tab (re-registering whenever the active tab
/// changes), and capture its mounted handle. The click/keyboard handlers pass through
/// from the binding unchanged.
pub(super) fn use_race_tab(binding: RaceTabBinding) -> RaceTabBehavior {
    let focus = use_focus_coordinator();
    let mut mounted_handle = use_signal(|| None::<Rc<MountedData>>);
    let is_active = binding.is_active;
    use_effect(use_reactive!(|is_active| {
        if is_active {
            let handle = mounted_handle.read().clone();
            focus.set_race_tabs_handle(handle);
        }
    }));
    let onmounted = EventHandler::new(move |event: Event<MountedData>| {
        let data = event.data();
        mounted_handle.set(Some(data));
    });
    let label = RaceTabLabelProps {
        label: binding.label.clone(),
    };
    let onclick = binding.onclick;
    let onkeydown = binding.onkeydown;
    RaceTabBehavior {
        is_active,
        label,
        onclick,
        onkeydown,
        onmounted,
    }
}
