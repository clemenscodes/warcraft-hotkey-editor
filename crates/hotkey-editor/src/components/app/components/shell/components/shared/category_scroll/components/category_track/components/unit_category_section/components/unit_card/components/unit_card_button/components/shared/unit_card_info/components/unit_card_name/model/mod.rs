use super::view::UnitCardNameView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCardNameModel {
    #[props(into)]
    pub text: String,
}

impl From<&UnitCardNameView> for UnitCardNameModel {
    fn from(view: &UnitCardNameView) -> Self {
        let UnitCardNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for UnitCardNameModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for UnitCardNameModel {
    type View = UnitCardNameView;
}
