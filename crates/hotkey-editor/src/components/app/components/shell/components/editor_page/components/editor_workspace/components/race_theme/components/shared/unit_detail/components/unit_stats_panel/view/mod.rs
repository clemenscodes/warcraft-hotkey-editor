use warcraft_api::Evasion;
use warcraft_api::{HeroAttributes, UnitCombat};

#[derive(Clone, PartialEq)]
pub struct UnitStatsPanelView {
    pub combat: UnitCombat,
    pub hero_attributes: Option<HeroAttributes>,
    pub evasion: Evasion,
}

impl ddd::Layered for UnitStatsPanelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitStatsPanelView {}
