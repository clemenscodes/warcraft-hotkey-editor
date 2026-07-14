use super::view::HotkeyConflictCardView;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyConflictView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyConflictCardModel {
    pub conflict: HotkeyConflictView,
    pub unit_id: WarcraftObjectId,
}

impl From<&HotkeyConflictCardView> for HotkeyConflictCardModel {
    fn from(view: &HotkeyConflictCardView) -> Self {
        let HotkeyConflictCardView { conflict, unit_id } = view.clone();
        Self { conflict, unit_id }
    }
}

impl ddd::Model for HotkeyConflictCardModel {
    type View = HotkeyConflictCardView;
}
