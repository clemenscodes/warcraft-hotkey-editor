use dioxus::prelude::*;

/// The hero-level picker: the current level, whether the menu is open, and the
/// signals it drives (the selected level and the open flag).
#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelPickerProps {
    pub current_level: u32,
    pub is_open: bool,
    pub selected_hero_level: Signal<u32>,
    pub level_picker_open: Signal<bool>,
}
