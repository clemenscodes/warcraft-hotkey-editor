use super::view::ActiveHitPointsRegenGainView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveHitPointsRegenGainModel {
    #[props(into)]
    pub text: String,
    pub mark: Option<&'static str>,
    pub condition: Option<&'static str>,
}

impl From<&ActiveHitPointsRegenGainView> for ActiveHitPointsRegenGainModel {
    fn from(view: &ActiveHitPointsRegenGainView) -> Self {
        let ActiveHitPointsRegenGainView {
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

impl ddd::Layered for ActiveHitPointsRegenGainModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ActiveHitPointsRegenGainModel {
    type View = ActiveHitPointsRegenGainView;
}
