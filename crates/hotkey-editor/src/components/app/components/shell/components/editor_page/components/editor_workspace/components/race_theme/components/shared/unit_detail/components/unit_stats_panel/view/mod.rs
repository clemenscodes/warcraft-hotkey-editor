use warcraft_api::Evasion;
use warcraft_api::{HeroAttributes, UnitCombat};

/// The published `View` contract mirroring [`UnitStatsPanelModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitStatsPanelView {
    pub combat: UnitCombat,
    pub hero_attributes: Option<HeroAttributes>,
    pub evasion: Evasion,
}

impl ddd::View for UnitStatsPanelView {}
