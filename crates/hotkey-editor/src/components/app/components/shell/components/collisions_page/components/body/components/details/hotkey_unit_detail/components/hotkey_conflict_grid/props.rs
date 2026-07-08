use super::components::hotkey_conflict_card::HotkeyConflictCardProps;
use dioxus::prelude::*;

/// The scrolling grid of shared-hotkey conflict cards.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyConflictGridProps {
    pub cards: Vec<HotkeyConflictCardProps>,
}
