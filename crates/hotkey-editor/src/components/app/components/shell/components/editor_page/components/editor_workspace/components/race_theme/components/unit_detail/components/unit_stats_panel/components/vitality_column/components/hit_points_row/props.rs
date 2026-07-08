use dioxus::prelude::*;
use warcraft_keybinds::HitPoints;

/// The hit points row's input: the unit's resolved hit points at the selected level.
#[derive(Props, Clone, PartialEq)]
pub struct HitPointsRowProps {
    pub value: HitPoints,
}
