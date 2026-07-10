use dioxus::prelude::*;
use warcraft_api::{HeroAttributes, UnitCombat};
use warcraft_keybinds::Evasion;

/// The stats panel's inputs: the unit's combat block, optional hero attributes, and the
/// resolved evasion chance (a domain figure). The selected hero level the hero columns
/// scale by is read from editor context, so it is not a prop.
#[derive(Props, Clone, PartialEq)]
pub struct UnitStatsPanelProps {
    pub combat: UnitCombat,
    pub hero_attributes: Option<HeroAttributes>,
    pub evasion: Evasion,
}
