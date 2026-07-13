use super::view::SystemHotkeysListEntryLabelView;
use dioxus::prelude::*;

/// A hotkey row's binding name.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysListEntryLabelModel {
    #[props(into)]
    pub text: String,
}

impl From<&SystemHotkeysListEntryLabelView> for SystemHotkeysListEntryLabelModel {
    fn from(view: &SystemHotkeysListEntryLabelView) -> Self {
        let SystemHotkeysListEntryLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for SystemHotkeysListEntryLabelModel {
    type View = SystemHotkeysListEntryLabelView;
}
