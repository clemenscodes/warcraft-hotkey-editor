use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SwitchItemLabelView {
    pub text: &'static str,
}

impl ddd::View for SwitchItemLabelView {}
