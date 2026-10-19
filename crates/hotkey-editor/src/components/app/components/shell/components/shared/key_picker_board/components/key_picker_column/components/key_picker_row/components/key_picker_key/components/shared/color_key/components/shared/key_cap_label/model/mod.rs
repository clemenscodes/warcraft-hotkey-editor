use super::view::KeyCapLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct KeyCapLabelModel {
    #[props(into)]
    pub label: String,
}

impl From<&KeyCapLabelView> for KeyCapLabelModel {
    fn from(view: &KeyCapLabelView) -> Self {
        let KeyCapLabelView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Layered for KeyCapLabelModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for KeyCapLabelModel {
    type View = KeyCapLabelView;
}
