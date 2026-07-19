use super::view::MutedHitPointsRegenGainView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MutedHitPointsRegenGainModel {
    #[props(into)]
    pub text: String,
}

impl From<&MutedHitPointsRegenGainView> for MutedHitPointsRegenGainModel {
    fn from(view: &MutedHitPointsRegenGainView) -> Self {
        let MutedHitPointsRegenGainView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for MutedHitPointsRegenGainModel {
    type View = MutedHitPointsRegenGainView;
}
