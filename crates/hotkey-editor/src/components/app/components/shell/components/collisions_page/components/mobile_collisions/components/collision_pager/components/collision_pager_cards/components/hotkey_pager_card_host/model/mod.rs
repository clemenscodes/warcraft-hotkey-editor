use super::view::HotkeyPagerCardHostView;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyPagerCardHostModel {
    pub unit: HotkeyUnitView,
}

impl From<&HotkeyPagerCardHostView> for HotkeyPagerCardHostModel {
    fn from(view: &HotkeyPagerCardHostView) -> Self {
        let HotkeyPagerCardHostView { unit } = view.clone();
        Self { unit }
    }
}

impl ddd::Model for HotkeyPagerCardHostModel {
    type View = HotkeyPagerCardHostView;
}
