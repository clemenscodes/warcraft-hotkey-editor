use super::view::PrimaryAttributeLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PrimaryAttributeLabelModel {
    #[props(into)]
    pub text: String,
}

impl From<&PrimaryAttributeLabelView> for PrimaryAttributeLabelModel {
    fn from(view: &PrimaryAttributeLabelView) -> Self {
        let PrimaryAttributeLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for PrimaryAttributeLabelModel {
    type View = PrimaryAttributeLabelView;
}
