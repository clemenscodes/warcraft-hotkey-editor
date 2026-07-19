use super::view::HotkeyPagerCardView;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyPagerCardModel {
    pub unit: HotkeyUnitView,
}

impl From<&HotkeyPagerCardView> for HotkeyPagerCardModel {
    fn from(view: &HotkeyPagerCardView) -> Self {
        let HotkeyPagerCardView { unit } = view.clone();
        Self { unit }
    }
}

impl ddd::Model for HotkeyPagerCardModel {
    type View = HotkeyPagerCardView;
}
