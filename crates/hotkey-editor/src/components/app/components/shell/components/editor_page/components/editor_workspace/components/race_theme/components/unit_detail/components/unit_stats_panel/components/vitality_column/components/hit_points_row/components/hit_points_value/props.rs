use dioxus::prelude::*;
use warcraft_keybinds::HitPoints;

/// The hit-points value leaf's input: the unit's resolved hit points.
#[derive(Props, Clone, PartialEq)]
pub struct HitPointsValueProps {
    pub value: HitPoints,
}
