use dioxus::prelude::*;

/// The hero-level dropdown menu: the current level (to mark the active option) and
/// the signals its options write when chosen.
#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelMenuProps {
    pub current_level: u32,
    pub selected_hero_level: Signal<u32>,
    pub level_picker_open: Signal<bool>,
}
