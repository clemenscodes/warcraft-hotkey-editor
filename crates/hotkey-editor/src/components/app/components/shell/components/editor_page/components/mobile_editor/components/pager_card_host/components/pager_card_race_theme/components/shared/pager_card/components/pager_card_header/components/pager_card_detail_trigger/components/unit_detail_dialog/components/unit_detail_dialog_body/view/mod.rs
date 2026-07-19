use super::UnitDetailDialogBody;
use super::model::UnitDetailDialogBodyModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;
use warcraft_api::{Evasion, HeroAttributes, UnitCombat};

#[derive(Clone, PartialEq, Default)]
pub struct UnitDetailDialogBodyView {
    pub portrait_url: Option<String>,
    pub description_text: String,
    pub combat: UnitCombat,
    pub hero_attributes: Option<HeroAttributes>,
    pub evasion: Evasion,
}

impl ddd::View for UnitDetailDialogBodyView {}

impl Render for UnitDetailDialogBodyView {
    type Model = UnitDetailDialogBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let portrait_url = self.portrait_url.clone();
        let description_text = self.description_text.clone();
        let combat = self.combat;
        let hero_attributes = self.hero_attributes;
        let evasion = self.evasion;
        rsx! {
            UnitDetailDialogBody {
                portrait_url,
                description_text,
                combat,
                hero_attributes,
                evasion,
            }
        }
    }
}
