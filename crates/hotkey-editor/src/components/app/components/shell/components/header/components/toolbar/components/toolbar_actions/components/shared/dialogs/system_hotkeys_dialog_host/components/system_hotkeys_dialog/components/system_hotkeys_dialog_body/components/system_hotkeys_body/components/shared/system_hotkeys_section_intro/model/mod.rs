use super::view::SystemHotkeysSectionIntroView;
use dioxus::prelude::*;

/// The one-line caption introducing a category editor.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysSectionIntroModel {
    #[props(into)]
    pub text: String,
}

impl From<&SystemHotkeysSectionIntroView> for SystemHotkeysSectionIntroModel {
    fn from(view: &SystemHotkeysSectionIntroView) -> Self {
        let SystemHotkeysSectionIntroView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for SystemHotkeysSectionIntroModel {
    type View = SystemHotkeysSectionIntroView;
}
