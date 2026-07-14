use super::view::UnitStatsPanelView;
use dioxus::prelude::*;
use warcraft_api::Evasion;
use warcraft_api::{HeroAttributes, UnitCombat};

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
