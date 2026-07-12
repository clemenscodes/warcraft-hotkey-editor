use super::view::HotkeyConflictGridView;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyConflictView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The scrolling grid of shared-hotkey conflict cards: one card per conflict, all
/// deep-linking through the owning unit id.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyConflictGridModel {
    pub conflicts: Vec<HotkeyConflictView>,
    pub unit_id: WarcraftObjectId,
}

impl From<&HotkeyConflictGridView> for HotkeyConflictGridModel {
    fn from(view: &HotkeyConflictGridView) -> Self {
        let HotkeyConflictGridView { conflicts, unit_id } = view.clone();
        Self { conflicts, unit_id }
    }
}

impl ddd::Model for HotkeyConflictGridModel {
    type View = HotkeyConflictGridView;
}
