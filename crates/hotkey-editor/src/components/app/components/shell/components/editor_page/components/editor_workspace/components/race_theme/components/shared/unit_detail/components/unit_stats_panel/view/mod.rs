use warcraft_api::Evasion;
use warcraft_api::{HeroAttributes, UnitCombat};

#[derive(Clone, PartialEq)]
pub struct UnitStatsPanelView {
    pub combat: UnitCombat,
    pub hero_attributes: Option<HeroAttributes>,
    pub evasion: Evasion,
}

impl ddd::View for UnitStatsPanelView {}
