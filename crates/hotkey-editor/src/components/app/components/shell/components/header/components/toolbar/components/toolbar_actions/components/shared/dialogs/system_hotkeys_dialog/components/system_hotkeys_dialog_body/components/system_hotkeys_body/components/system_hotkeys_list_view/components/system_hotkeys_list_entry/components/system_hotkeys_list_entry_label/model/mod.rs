use super::view::SystemHotkeysListEntryLabelView;
use dioxus::prelude::*;

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

impl ddd::Layered for SystemHotkeysListEntryLabelModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for SystemHotkeysListEntryLabelModel {
    type View = SystemHotkeysListEntryLabelView;
}
