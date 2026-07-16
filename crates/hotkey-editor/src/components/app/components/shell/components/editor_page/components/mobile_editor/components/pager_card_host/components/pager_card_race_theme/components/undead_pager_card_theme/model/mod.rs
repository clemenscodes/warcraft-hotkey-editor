use super::view::UndeadPagerCardThemeView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct UndeadPagerCardThemeModel {
    pub unit_id: WarcraftObjectId,
}

impl From<&UndeadPagerCardThemeView> for UndeadPagerCardThemeModel {
    fn from(view: &UndeadPagerCardThemeView) -> Self {
        let UndeadPagerCardThemeView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Model for UndeadPagerCardThemeModel {
    type View = UndeadPagerCardThemeView;
}
