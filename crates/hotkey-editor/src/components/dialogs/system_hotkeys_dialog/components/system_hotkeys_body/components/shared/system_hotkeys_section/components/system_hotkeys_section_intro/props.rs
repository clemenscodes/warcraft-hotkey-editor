use dioxus::prelude::*;

/// The one-line caption introducing a category editor.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysSectionIntroProps {
    #[props(into)]
    pub text: String,
}
