use dioxus::prelude::*;

/// One selectable hero level in the dropdown: which level it offers, whether it is
/// the current level, and the signals it writes when chosen.
#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelOptionProps {
    pub level_index: u32,
    pub current_level: u32,
    pub selected_hero_level: Signal<u32>,
    pub level_picker_open: Signal<bool>,
}
