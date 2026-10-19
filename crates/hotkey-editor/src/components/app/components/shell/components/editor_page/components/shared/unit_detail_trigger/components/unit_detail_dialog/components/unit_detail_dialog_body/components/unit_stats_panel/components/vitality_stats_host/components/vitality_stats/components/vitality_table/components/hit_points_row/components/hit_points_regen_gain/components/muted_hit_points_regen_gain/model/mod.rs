use super::view::MutedHitPointsRegenGainView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MutedHitPointsRegenGainModel {
    #[props(into)]
    pub text: String,
    pub mark: Option<&'static str>,
    pub condition: Option<&'static str>,
}

impl From<&MutedHitPointsRegenGainView> for MutedHitPointsRegenGainModel {
    fn from(view: &MutedHitPointsRegenGainView) -> Self {
        let MutedHitPointsRegenGainView {
            text,
            mark,
            condition,
        } = view.clone();
        Self {
            text,
            mark,
            condition,
        }
    }
}

impl ddd::Layered for MutedHitPointsRegenGainModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for MutedHitPointsRegenGainModel {
    type View = MutedHitPointsRegenGainView;
}
