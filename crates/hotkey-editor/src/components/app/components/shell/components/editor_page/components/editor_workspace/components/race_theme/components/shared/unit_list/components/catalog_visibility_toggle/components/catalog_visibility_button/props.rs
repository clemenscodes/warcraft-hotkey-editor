use crate::components::app::components::shell::components::editor_page::components::shared::toggle_button::ToggleButtonProps;
use dioxus::prelude::*;

/// One catalog-visibility toggle: its label, tooltip, current on/off state, and the
/// handler that flips it.
#[derive(Props, Clone, PartialEq)]
pub struct CatalogVisibilityButtonProps {
    pub label: &'static str,
    pub title: &'static str,
    pub is_active: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl From<&CatalogVisibilityButtonProps> for ToggleButtonProps {
    fn from(props: &CatalogVisibilityButtonProps) -> Self {
        Self {
            label: props.label,
            active: props.is_active,
            title: Some(props.title),
            onclick: props.on_toggle,
            onkeydown: EventHandler::default(),
        }
    }
}
