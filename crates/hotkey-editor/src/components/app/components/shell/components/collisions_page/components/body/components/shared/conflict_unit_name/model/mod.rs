use super::view::ConflictUnitNameView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictUnitNameModel {
    #[props(into)]
    pub text: String,
}

impl From<&ConflictUnitNameView> for ConflictUnitNameModel {
    fn from(view: &ConflictUnitNameView) -> Self {
        let ConflictUnitNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for ConflictUnitNameModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ConflictUnitNameModel {
    type View = ConflictUnitNameView;
}
