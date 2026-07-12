use super::view::SystemHotkeysBreadcrumbsTriggerLabelView;
use dioxus::prelude::*;

/// The trigger's caption: the active category name.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsTriggerLabelModel {
    #[props(into)]
    pub text: String,
}

impl From<&SystemHotkeysBreadcrumbsTriggerLabelView> for SystemHotkeysBreadcrumbsTriggerLabelModel {
    fn from(view: &SystemHotkeysBreadcrumbsTriggerLabelView) -> Self {
        let SystemHotkeysBreadcrumbsTriggerLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for SystemHotkeysBreadcrumbsTriggerLabelModel {
    type View = SystemHotkeysBreadcrumbsTriggerLabelView;
}
