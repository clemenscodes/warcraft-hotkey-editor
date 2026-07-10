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
