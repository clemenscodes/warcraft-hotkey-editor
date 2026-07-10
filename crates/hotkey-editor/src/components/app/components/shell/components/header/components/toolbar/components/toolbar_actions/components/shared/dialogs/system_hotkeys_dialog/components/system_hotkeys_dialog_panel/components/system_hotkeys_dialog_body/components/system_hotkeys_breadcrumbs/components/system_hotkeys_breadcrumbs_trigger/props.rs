use super::components::system_hotkeys_breadcrumbs_trigger_caret::SystemHotkeysBreadcrumbsTriggerCaretProps;
use super::components::system_hotkeys_breadcrumbs_trigger_label::SystemHotkeysBreadcrumbsTriggerLabelProps;
use dioxus::prelude::*;

/// The mobile dropdown trigger's inputs: the active category caption, whether the
/// dropdown is open (for aria and the caret), and the toggle handler.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsTriggerProps {
    #[props(into)]
    pub label: String,
    pub is_open: bool,
    pub open: &'static str,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl From<&SystemHotkeysBreadcrumbsTriggerProps> for SystemHotkeysBreadcrumbsTriggerLabelProps {
    fn from(props: &SystemHotkeysBreadcrumbsTriggerProps) -> Self {
        let text = props.label.clone();
        Self { text }
    }
}

impl From<&SystemHotkeysBreadcrumbsTriggerProps> for SystemHotkeysBreadcrumbsTriggerCaretProps {
    fn from(props: &SystemHotkeysBreadcrumbsTriggerProps) -> Self {
        let open = props.open;
        Self { open }
    }
}
