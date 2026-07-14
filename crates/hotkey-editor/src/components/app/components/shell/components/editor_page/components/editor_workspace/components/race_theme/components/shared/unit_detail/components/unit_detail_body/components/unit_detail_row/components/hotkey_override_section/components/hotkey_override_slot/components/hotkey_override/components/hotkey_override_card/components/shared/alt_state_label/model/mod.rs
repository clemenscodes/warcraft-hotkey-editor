use super::view::AltStateLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AltStateLabelModel {
    pub text: Option<String>,
}

impl From<&AltStateLabelView> for AltStateLabelModel {
    fn from(view: &AltStateLabelView) -> Self {
        let AltStateLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for AltStateLabelModel {
    type View = AltStateLabelView;
}
