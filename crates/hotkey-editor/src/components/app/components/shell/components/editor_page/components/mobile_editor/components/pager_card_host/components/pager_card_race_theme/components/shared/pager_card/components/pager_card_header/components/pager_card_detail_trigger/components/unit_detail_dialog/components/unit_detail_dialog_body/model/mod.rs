use super::view::UnitDetailDialogBodyView;
use dioxus::prelude::*;
use warcraft_api::{Evasion, HeroAttributes, UnitCombat};

#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailDialogBodyModel {
    pub portrait_url: Option<String>,
    #[props(into)]
    pub description_text: String,
    pub combat: UnitCombat,
    pub hero_attributes: Option<HeroAttributes>,
    pub evasion: Evasion,
}

impl From<&UnitDetailDialogBodyView> for UnitDetailDialogBodyModel {
    fn from(view: &UnitDetailDialogBodyView) -> Self {
        let UnitDetailDialogBodyView {
            portrait_url,
            description_text,
            combat,
            hero_attributes,
            evasion,
        } = view.clone();
        Self {
            portrait_url,
            description_text,
            combat,
            hero_attributes,
            evasion,
        }
    }
}

impl ddd::Model for UnitDetailDialogBodyModel {
    type View = UnitDetailDialogBodyView;
}
