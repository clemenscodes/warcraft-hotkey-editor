use super::view::MutedHitPointsRegenGainView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MutedHitPointsRegenGainModel {
    #[props(into)]
    pub text: String,
    pub condition: Option<&'static str>,
}

impl From<&MutedHitPointsRegenGainView> for MutedHitPointsRegenGainModel {
    fn from(view: &MutedHitPointsRegenGainView) -> Self {
        let MutedHitPointsRegenGainView { text, condition } = view.clone();
        Self { text, condition }
    }
}

impl ddd::Layered for MutedHitPointsRegenGainModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for MutedHitPointsRegenGainModel {
    type View = MutedHitPointsRegenGainView;
}
