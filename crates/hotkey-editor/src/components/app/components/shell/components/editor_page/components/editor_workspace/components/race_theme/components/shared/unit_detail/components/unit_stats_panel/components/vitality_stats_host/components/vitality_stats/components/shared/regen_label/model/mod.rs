use super::view::RegenLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RegenLabelModel {
    #[props(into)]
    pub text: String,
}

impl From<&RegenLabelView> for RegenLabelModel {
    fn from(view: &RegenLabelView) -> Self {
        let RegenLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for RegenLabelModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for RegenLabelModel {
    type View = RegenLabelView;
}
