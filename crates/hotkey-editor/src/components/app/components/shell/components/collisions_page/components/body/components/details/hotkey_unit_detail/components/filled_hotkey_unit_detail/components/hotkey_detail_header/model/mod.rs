use super::view::HotkeyDetailHeaderView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitIconView;
use dioxus::prelude::*;

/// The detail-pane header row: the selected unit and its collision count. The header
/// builds the unit button and the text meta column from the unit view.
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
