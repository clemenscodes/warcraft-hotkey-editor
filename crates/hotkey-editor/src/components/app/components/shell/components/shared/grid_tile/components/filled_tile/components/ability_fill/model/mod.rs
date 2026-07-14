use super::view::AbilityFillView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AbilityFillModel {
    pub active: bool,
}

impl From<&AbilityFillView> for AbilityFillModel {
    fn from(view: &AbilityFillView) -> Self {
        let AbilityFillView { active } = view.clone();
        Self { active }
    }
}

impl ddd::Model for AbilityFillModel {
    type View = AbilityFillView;
}
