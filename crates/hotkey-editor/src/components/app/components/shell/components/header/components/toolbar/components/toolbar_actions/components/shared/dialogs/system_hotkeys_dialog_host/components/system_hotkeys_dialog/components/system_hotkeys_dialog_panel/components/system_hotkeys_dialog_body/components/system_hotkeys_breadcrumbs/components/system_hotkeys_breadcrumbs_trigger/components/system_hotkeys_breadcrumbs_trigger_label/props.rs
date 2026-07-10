use dioxus::prelude::*;

/// The trigger's caption: the active category name.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsTriggerLabelProps {
    #[props(into)]
    pub text: String,
}
