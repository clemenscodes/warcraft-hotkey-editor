use super::view::IslandConflictUnitNameView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictUnitNameModel {
    #[props(into)]
    pub text: String,
}

impl From<&IslandConflictUnitNameView> for IslandConflictUnitNameModel {
    fn from(view: &IslandConflictUnitNameView) -> Self {
        let IslandConflictUnitNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for IslandConflictUnitNameModel {
    type View = IslandConflictUnitNameView;
}
