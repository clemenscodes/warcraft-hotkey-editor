use super::view::HotkeyDetailHeaderView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyDetailHeaderModel {
    pub unit: UnitIconView,
    pub count: usize,
}

impl From<&HotkeyDetailHeaderView> for HotkeyDetailHeaderModel {
    fn from(view: &HotkeyDetailHeaderView) -> Self {
        let HotkeyDetailHeaderView { unit, count } = view.clone();
        Self { unit, count }
    }
}

impl ddd::Model for HotkeyDetailHeaderModel {
    type View = HotkeyDetailHeaderView;
}
