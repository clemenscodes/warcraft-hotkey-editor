use dioxus::prelude::*;
use warcraft_keybinds::EffectiveHitPoints;

/// The effective hit points row's input: raw health scaled by armor.
#[derive(Props, Clone, PartialEq)]
pub struct EffectiveHitPointsRowProps {
    pub value: EffectiveHitPoints,
}
