use dioxus::prelude::*;
use warcraft_api::{HeroAttributes, UnitCombat};
use warcraft_keybinds::Evasion;

/// The stats panel's inputs: the unit's combat block, optional hero attributes, the
/// selected hero level, and the resolved evasion chance (a domain figure).
#[derive(Props, Clone, PartialEq)]
pub struct UnitStatsPanelProps {
    pub combat: UnitCombat,
    pub hero_attributes: Option<HeroAttributes>,
    pub selected_hero_level: Signal<u32>,
    pub evasion: Evasion,
}
