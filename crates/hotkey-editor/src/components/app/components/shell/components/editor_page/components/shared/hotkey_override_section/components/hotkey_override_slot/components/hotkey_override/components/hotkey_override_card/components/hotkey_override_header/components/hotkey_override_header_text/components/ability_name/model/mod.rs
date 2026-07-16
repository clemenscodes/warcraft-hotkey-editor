use super::view::AbilityNameView;
use dioxus::prelude::*;

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
