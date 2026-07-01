use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictHotkeyKeyProps {
    #[props(into)]
    pub text: String,
}
