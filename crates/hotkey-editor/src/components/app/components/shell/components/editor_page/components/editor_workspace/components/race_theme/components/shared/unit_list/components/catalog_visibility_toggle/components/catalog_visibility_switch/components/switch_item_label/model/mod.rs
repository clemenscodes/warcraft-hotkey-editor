use super::view::SwitchItemLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SwitchItemLabelModel {
    pub text: &'static str,
}

impl From<&SwitchItemLabelView> for SwitchItemLabelModel {
    fn from(view: &SwitchItemLabelView) -> Self {
        let SwitchItemLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for SwitchItemLabelModel {
    type View = SwitchItemLabelView;
}
