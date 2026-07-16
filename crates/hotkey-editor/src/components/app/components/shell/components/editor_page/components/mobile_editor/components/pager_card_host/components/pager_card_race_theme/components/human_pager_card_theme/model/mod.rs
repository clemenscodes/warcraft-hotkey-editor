use super::view::HumanPagerCardThemeView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct HumanPagerCardThemeModel {
    pub unit_id: WarcraftObjectId,
}

impl From<&HumanPagerCardThemeView> for HumanPagerCardThemeModel {
    fn from(view: &HumanPagerCardThemeView) -> Self {
        let HumanPagerCardThemeView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Model for HumanPagerCardThemeModel {
    type View = HumanPagerCardThemeView;
}
