use super::view::OrcPagerCardThemeView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct OrcPagerCardThemeModel {
    pub unit_id: WarcraftObjectId,
}

impl From<&OrcPagerCardThemeView> for OrcPagerCardThemeModel {
    fn from(view: &OrcPagerCardThemeView) -> Self {
        let OrcPagerCardThemeView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Model for OrcPagerCardThemeModel {
    type View = OrcPagerCardThemeView;
}
