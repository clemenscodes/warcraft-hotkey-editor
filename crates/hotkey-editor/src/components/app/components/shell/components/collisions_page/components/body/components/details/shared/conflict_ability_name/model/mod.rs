use super::view::ConflictAbilityNameView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictAbilityNameModel {
    #[props(into)]
    pub text: String,
}

impl From<&ConflictAbilityNameView> for ConflictAbilityNameModel {
    fn from(view: &ConflictAbilityNameView) -> Self {
        let ConflictAbilityNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for ConflictAbilityNameModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ConflictAbilityNameModel {
    type View = ConflictAbilityNameView;
}
