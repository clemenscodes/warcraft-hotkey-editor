use super::view::ConflictUnitNameView;
use dioxus::prelude::*;
/// A unit's name on a hotkey/position collision card.
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

impl ddd::Model for ConflictUnitNameModel {
    type View = ConflictUnitNameView;
}
