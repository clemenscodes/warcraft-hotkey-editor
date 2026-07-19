use super::view::ActiveHitPointsRegenGainView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveHitPointsRegenGainModel {
    #[props(into)]
    pub text: String,
}

impl From<&ActiveHitPointsRegenGainView> for ActiveHitPointsRegenGainModel {
    fn from(view: &ActiveHitPointsRegenGainView) -> Self {
        let ActiveHitPointsRegenGainView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for ActiveHitPointsRegenGainModel {
    type View = ActiveHitPointsRegenGainView;
}
