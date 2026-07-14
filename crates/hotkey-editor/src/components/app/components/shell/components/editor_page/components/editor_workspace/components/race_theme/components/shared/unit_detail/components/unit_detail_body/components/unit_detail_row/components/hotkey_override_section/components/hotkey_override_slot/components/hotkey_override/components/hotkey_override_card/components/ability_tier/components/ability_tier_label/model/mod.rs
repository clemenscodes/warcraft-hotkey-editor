use super::view::AbilityTierLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AbilityTierLabelModel {
    #[props(into)]
    pub text: String,
}

impl From<&AbilityTierLabelView> for AbilityTierLabelModel {
    fn from(view: &AbilityTierLabelView) -> Self {
        let AbilityTierLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for AbilityTierLabelModel {
    type View = AbilityTierLabelView;
}
