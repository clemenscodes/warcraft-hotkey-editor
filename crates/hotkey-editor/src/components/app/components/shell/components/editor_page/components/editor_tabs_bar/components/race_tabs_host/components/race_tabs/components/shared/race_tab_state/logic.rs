use super::components::active_race_tab::ActiveRaceTabProps;
use super::components::inactive_race_tab::InactiveRaceTabProps;
use super::components::shared::race_tab::components::race_tab_label::RaceTabLabelProps;
use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::RaceTabBinding;
use dioxus::prelude::*;

/// A race tab's finished behaviour: whether it is the active tab, its label child props,
/// and the pointer/keyboard handlers. The click and keyboard handlers arrive already
/// baked in the `RaceTabBinding` (the cascade lives behind them). The look picks the
/// active-or-inactive component from `is_active`.
pub(super) struct RaceTabBehavior {
    is_active: bool,
    label: RaceTabLabelProps,
    onclick: EventHandler<MouseEvent>,
    onkeydown: EventHandler<KeyboardEvent>,
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
}

impl From<&RaceTabBinding> for RaceTabBehavior {
    fn from(binding: &RaceTabBinding) -> Self {
        let is_active = binding.is_active;
        let label = RaceTabLabelProps {
            label: binding.label.clone(),
        };
        let onclick = binding.onclick;
        let onkeydown = binding.onkeydown;
        Self {
            is_active,
            label,
            onclick,
            onkeydown,
        }
    }
}

impl From<&RaceTabBehavior> for ActiveRaceTabProps {
    fn from(behavior: &RaceTabBehavior) -> Self {
        let label = behavior.label().clone();
        let onclick = behavior.onclick();
        let onkeydown = behavior.onkeydown();
        Self {
            label,
            onclick,
            onkeydown,
        }
    }
}

impl From<&RaceTabBehavior> for InactiveRaceTabProps {
    fn from(behavior: &RaceTabBehavior) -> Self {
        let label = behavior.label().clone();
        let onclick = behavior.onclick();
        let onkeydown = behavior.onkeydown();
        Self {
            label,
            onclick,
            onkeydown,
        }
    }
}
