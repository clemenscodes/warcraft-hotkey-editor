use super::view::SystemHotkeysBreadcrumbsTriggerLabelView;
use dioxus::prelude::*;

/// The trigger's caption: the active category name.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsTriggerLabelProps {
    #[props(into)]
    pub text: String,
}

impl From<&SystemHotkeysBreadcrumbsTriggerLabelView> for SystemHotkeysBreadcrumbsTriggerLabelProps {
    fn from(view: &SystemHotkeysBreadcrumbsTriggerLabelView) -> Self {
        let SystemHotkeysBreadcrumbsTriggerLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for SystemHotkeysBreadcrumbsTriggerLabelProps {
    type View = SystemHotkeysBreadcrumbsTriggerLabelView;
}
