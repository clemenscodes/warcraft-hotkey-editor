use dioxus::prelude::*;
use warcraft_keybinds::HitPointsRegen;

/// The hit-points regeneration row's input: the unit's resolved health regeneration,
/// which carries its own conditional (at night, on blight) and its rate.
#[derive(Props, Clone, PartialEq)]
pub struct HitPointsRegenRowProps {
    pub value: HitPointsRegen,
}
