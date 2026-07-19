use super::view::HitPointsRegenGainView;
use dioxus::prelude::*;
use warcraft_api::HitPointsRegen;

#[derive(Props, Clone, PartialEq)]
pub struct HitPointsRegenGainModel {
    pub value: HitPointsRegen,
}

impl From<&HitPointsRegenGainView> for HitPointsRegenGainModel {
    fn from(view: &HitPointsRegenGainView) -> Self {
        let HitPointsRegenGainView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for HitPointsRegenGainModel {
    type View = HitPointsRegenGainView;
}
