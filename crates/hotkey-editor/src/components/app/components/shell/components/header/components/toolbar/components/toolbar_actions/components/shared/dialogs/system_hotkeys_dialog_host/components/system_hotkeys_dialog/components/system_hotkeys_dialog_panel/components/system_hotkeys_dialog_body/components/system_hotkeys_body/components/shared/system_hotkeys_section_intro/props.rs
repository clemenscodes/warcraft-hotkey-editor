use super::view::SystemHotkeysSectionIntroView;
use dioxus::prelude::*;

/// The one-line caption introducing a category editor.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysSectionIntroProps {
    #[props(into)]
    pub text: String,
}

impl From<&SystemHotkeysSectionIntroView> for SystemHotkeysSectionIntroProps {
    fn from(view: &SystemHotkeysSectionIntroView) -> Self {
        let SystemHotkeysSectionIntroView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for SystemHotkeysSectionIntroProps {
    type View = SystemHotkeysSectionIntroView;
}
