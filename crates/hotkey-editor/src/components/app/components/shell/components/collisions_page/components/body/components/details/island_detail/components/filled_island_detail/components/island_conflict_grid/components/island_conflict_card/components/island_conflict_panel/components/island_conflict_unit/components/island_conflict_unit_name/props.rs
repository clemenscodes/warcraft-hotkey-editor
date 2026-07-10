use super::view::IslandConflictUnitNameView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictUnitNameProps {
    #[props(into)]
    pub text: String,
}

impl From<&IslandConflictUnitNameView> for IslandConflictUnitNameProps {
    fn from(view: &IslandConflictUnitNameView) -> Self {
        let IslandConflictUnitNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for IslandConflictUnitNameProps {
    type View = IslandConflictUnitNameView;
}
