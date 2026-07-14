use super::view::SystemSlotLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SystemSlotLabelModel {
    #[props(into)]
    pub text: String,
}

impl From<&SystemSlotLabelView> for SystemSlotLabelModel {
    fn from(view: &SystemSlotLabelView) -> Self {
        let SystemSlotLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for SystemSlotLabelModel {
    type View = SystemSlotLabelView;
}
