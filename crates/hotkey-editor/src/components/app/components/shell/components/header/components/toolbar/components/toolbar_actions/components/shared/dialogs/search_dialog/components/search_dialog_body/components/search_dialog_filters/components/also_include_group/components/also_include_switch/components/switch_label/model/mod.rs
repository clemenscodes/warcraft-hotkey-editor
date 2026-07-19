use super::view::SwitchLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SwitchLabelModel {
    pub text: &'static str,
    pub popover_text: &'static str,
}

impl From<&SwitchLabelView> for SwitchLabelModel {
    fn from(view: &SwitchLabelView) -> Self {
        let SwitchLabelView { text, popover_text } = view.clone();
        Self { text, popover_text }
    }
}

impl ddd::Model for SwitchLabelModel {
    type View = SwitchLabelView;
}
