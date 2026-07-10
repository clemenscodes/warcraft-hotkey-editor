use super::view::SystemHotkeysBreadcrumbsTriggerView;
use dioxus::prelude::*;

/// The mobile dropdown trigger's inputs: the active category caption, whether the
/// dropdown is open (for aria and the caret), and the toggle handler.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsTriggerProps {
    #[props(into)]
    pub label: String,
    pub is_open: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl From<&SystemHotkeysBreadcrumbsTriggerView> for SystemHotkeysBreadcrumbsTriggerProps {
    fn from(view: &SystemHotkeysBreadcrumbsTriggerView) -> Self {
        let SystemHotkeysBreadcrumbsTriggerView {
            label,
            is_open,
            on_toggle,
        } = view.clone();
        Self {
            label,
            is_open,
            on_toggle,
        }
    }
}

impl ddd::Props for SystemHotkeysBreadcrumbsTriggerProps {
    type View = SystemHotkeysBreadcrumbsTriggerView;
}
