use super::view::PrimaryAttributeLabelView;
use dioxus::prelude::*;

/// An attribute label leaf's input: the attribute name it presents. The three
/// attribute rows share this look — gold that brightens under a primary-attribute
/// group — and thread only their name.
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
