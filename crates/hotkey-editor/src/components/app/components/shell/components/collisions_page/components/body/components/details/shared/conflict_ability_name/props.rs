use super::view::ConflictAbilityNameView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictAbilityNameProps {
    #[props(into)]
    pub text: String,
}

impl From<&ConflictAbilityNameView> for ConflictAbilityNameProps {
    fn from(view: &ConflictAbilityNameView) -> Self {
        let ConflictAbilityNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for ConflictAbilityNameProps {
    type View = ConflictAbilityNameView;
}
