use dioxus::prelude::*;

/// A hotkey row's binding name.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysListEntryLabelProps {
    #[props(into)]
    pub text: String,
}
