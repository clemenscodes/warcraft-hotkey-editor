use crate::components::app::components::shell::components::editor_page::components::shared::toggle_button::ToggleButtonProps;
use dioxus::prelude::*;

/// One mode button: the label it shows, whether it is the active mode, and the
/// pointer and keyboard activation handlers.
#[derive(Props, Clone, PartialEq)]
pub struct ModeTabProps {
    pub label: &'static str,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&ModeTabProps> for ToggleButtonProps {
    fn from(props: &ModeTabProps) -> Self {
        Self {
            label: props.label,
            active: props.active,
            title: None,
            onclick: props.onclick,
            onkeydown: props.onkeydown,
        }
    }
}
