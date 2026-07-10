use dioxus::prelude::*;
use warcraft_keybinds::HitPointsRegen;

/// The health-regeneration gain leaf's input: the unit's resolved health regeneration.
#[derive(Props, Clone, PartialEq)]
pub struct HitPointsRegenGainProps {
    pub value: HitPointsRegen,
}
