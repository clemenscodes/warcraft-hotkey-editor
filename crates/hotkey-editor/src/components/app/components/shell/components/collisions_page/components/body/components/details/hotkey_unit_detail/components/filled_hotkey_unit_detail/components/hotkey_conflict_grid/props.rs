use super::view::HotkeyConflictGridView;
use crate::components::app::components::shell::components::collisions_page::logic::HotkeyConflictView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The scrolling grid of shared-hotkey conflict cards: one card per conflict, all
/// deep-linking through the owning unit id.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyConflictGridProps {
    pub conflicts: Vec<HotkeyConflictView>,
    pub unit_id: WarcraftObjectId,
}

impl From<&HotkeyConflictGridView> for HotkeyConflictGridProps {
    fn from(view: &HotkeyConflictGridView) -> Self {
        let HotkeyConflictGridView { conflicts, unit_id } = view.clone();
        Self { conflicts, unit_id }
    }
}

impl ddd::Props for HotkeyConflictGridProps {
    type View = HotkeyConflictGridView;
}
