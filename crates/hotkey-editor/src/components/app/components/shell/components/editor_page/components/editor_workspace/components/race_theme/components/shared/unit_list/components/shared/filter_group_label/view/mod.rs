use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct FilterGroupLabelView {
    pub text: &'static str,
}

impl ddd::View for FilterGroupLabelView {}
