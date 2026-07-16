use super::view::NeutralPagerCardThemeView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct NeutralPagerCardThemeModel {
    pub unit_id: WarcraftObjectId,
}

impl From<&NeutralPagerCardThemeView> for NeutralPagerCardThemeModel {
    fn from(view: &NeutralPagerCardThemeView) -> Self {
        let NeutralPagerCardThemeView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Model for NeutralPagerCardThemeModel {
    type View = NeutralPagerCardThemeView;
}
