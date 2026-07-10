use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::RaceTabBinding;
use dioxus::prelude::*;

/// A race tab's finished behaviour: whether it is the active tab, its display name, and
/// the pointer/keyboard handlers. The click and keyboard handlers arrive already baked in
/// the `RaceTabBinding` (the cascade lives behind them). The look picks the
/// active-or-inactive component from `is_active`.
pub(super) struct RaceTabBehavior {
    is_active: bool,
    label: String,
    onclick: EventHandler<MouseEvent>,
    onkeydown: EventHandler<KeyboardEvent>,
}

impl RaceTabBehavior {
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
}

impl From<&RaceTabBinding> for RaceTabBehavior {
    fn from(binding: &RaceTabBinding) -> Self {
        let is_active = binding.is_active;
        let label = binding.label.clone();
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
