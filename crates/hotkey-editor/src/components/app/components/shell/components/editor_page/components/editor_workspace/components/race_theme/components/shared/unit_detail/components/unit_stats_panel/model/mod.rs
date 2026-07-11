use super::view::UnitStatsPanelView;
use dioxus::prelude::*;
use warcraft_api::Evasion;
use warcraft_api::{HeroAttributes, UnitCombat};

/// The stats panel's inputs: the unit's combat block, optional hero attributes, and the
/// resolved evasion chance (a domain figure). The selected hero level the hero columns
/// scale by is read from editor context, so it is not a prop.
#[derive(Props, Clone, PartialEq)]
pub struct UnitStatsPanelModel {
    pub combat: UnitCombat,
    pub hero_attributes: Option<HeroAttributes>,
    pub evasion: Evasion,
}

impl From<&UnitStatsPanelView> for UnitStatsPanelModel {
    fn from(view: &UnitStatsPanelView) -> Self {
        let UnitStatsPanelView {
            combat,
            hero_attributes,
            evasion,
        } = view.clone();
        Self {
            combat,
            hero_attributes,
            evasion,
        }
    }
}

impl ddd::Model for UnitStatsPanelModel {
    type View = UnitStatsPanelView;
}
