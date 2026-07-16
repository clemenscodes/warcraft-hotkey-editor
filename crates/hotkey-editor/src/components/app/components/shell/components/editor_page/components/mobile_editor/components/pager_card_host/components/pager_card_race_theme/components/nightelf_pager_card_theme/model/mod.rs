use super::view::NightelfPagerCardThemeView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct NightelfPagerCardThemeModel {
    pub unit_id: WarcraftObjectId,
}

impl From<&NightelfPagerCardThemeView> for NightelfPagerCardThemeModel {
    fn from(view: &NightelfPagerCardThemeView) -> Self {
        let NightelfPagerCardThemeView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Model for NightelfPagerCardThemeModel {
    type View = NightelfPagerCardThemeView;
}
