use super::view::SystemHotkeysListEntryLabelView;
use dioxus::prelude::*;

/// A hotkey row's binding name.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysListEntryLabelProps {
    #[props(into)]
    pub text: String,
}

impl From<&SystemHotkeysListEntryLabelView> for SystemHotkeysListEntryLabelProps {
    fn from(view: &SystemHotkeysListEntryLabelView) -> Self {
        let SystemHotkeysListEntryLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for SystemHotkeysListEntryLabelProps {
    type View = SystemHotkeysListEntryLabelView;
}
