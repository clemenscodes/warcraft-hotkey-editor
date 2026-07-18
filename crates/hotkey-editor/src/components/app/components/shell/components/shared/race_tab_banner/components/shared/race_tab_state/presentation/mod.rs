use crate::components::app::components::shell::components::shared::race_tab_banner::binding::RaceTabBinding;
use dioxus::prelude::*;

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
