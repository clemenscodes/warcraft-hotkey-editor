use super::view::PagerCardRaceThemeView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct PagerCardRaceThemeModel {
    pub unit_id: WarcraftObjectId,
}

impl From<&PagerCardRaceThemeView> for PagerCardRaceThemeModel {
    fn from(view: &PagerCardRaceThemeView) -> Self {
        let PagerCardRaceThemeView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Model for PagerCardRaceThemeModel {
    type View = PagerCardRaceThemeView;
}
