use super::view::HitPointsRegenGainView;
use dioxus::prelude::*;
use warcraft_keybinds::HitPointsRegen;

/// The health-regeneration gain leaf's input: the unit's resolved health regeneration.
#[derive(Props, Clone, PartialEq)]
pub struct HitPointsRegenGainProps {
    pub value: HitPointsRegen,
}

impl From<&HitPointsRegenGainView> for HitPointsRegenGainProps {
    fn from(view: &HitPointsRegenGainView) -> Self {
        let HitPointsRegenGainView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for HitPointsRegenGainProps {
    type View = HitPointsRegenGainView;
}
