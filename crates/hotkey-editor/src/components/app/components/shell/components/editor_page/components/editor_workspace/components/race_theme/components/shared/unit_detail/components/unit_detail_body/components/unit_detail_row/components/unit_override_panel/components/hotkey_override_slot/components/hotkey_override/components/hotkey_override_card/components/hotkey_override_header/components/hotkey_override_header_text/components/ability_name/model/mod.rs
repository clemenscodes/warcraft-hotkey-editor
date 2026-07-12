use super::view::AbilityNameView;
use dioxus::prelude::*;

/// The active ability / unit name shown in the override panel header.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityNameModel {
    #[props(into)]
    pub text: String,
}

impl From<&AbilityNameView> for AbilityNameModel {
    fn from(view: &AbilityNameView) -> Self {
        let AbilityNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for AbilityNameModel {
    type View = AbilityNameView;
}
