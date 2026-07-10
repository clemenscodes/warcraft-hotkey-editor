use crate::components::app::components::shell::components::editor_page::components::shared::toggle_button::ToggleButtonProps;
use dioxus::prelude::*;

/// One search-field option: its label, whether it is the active field, and the
/// handler that selects it.
#[derive(Props, Clone, PartialEq)]
pub struct SearchFieldButtonProps {
    pub label: &'static str,
    pub is_active: bool,
    pub on_select: EventHandler<MouseEvent>,
}

impl From<&SearchFieldButtonProps> for ToggleButtonProps {
    fn from(props: &SearchFieldButtonProps) -> Self {
        Self {
            label: props.label,
            active: props.is_active,
            title: None,
            onclick: props.on_select,
            onkeydown: EventHandler::default(),
        }
    }
}
