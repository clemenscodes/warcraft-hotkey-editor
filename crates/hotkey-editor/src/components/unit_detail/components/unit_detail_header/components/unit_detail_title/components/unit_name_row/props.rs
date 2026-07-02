use dioxus::prelude::*;

/// The name row: the unit name, and — for heroes — the level picker beside it.
#[derive(Props, Clone, PartialEq)]
pub struct UnitNameRowProps {
    pub unit_name: &'static str,
    pub has_hero_attributes: bool,
    pub current_level: u32,
    pub is_open: bool,
    pub selected_hero_level: Signal<u32>,
    pub level_picker_open: Signal<bool>,
}
