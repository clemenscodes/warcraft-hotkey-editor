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
