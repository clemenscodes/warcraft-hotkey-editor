use dioxus::prelude::*;

/// The title column: the name row over the unit id.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailTitleProps {
    pub unit_name: &'static str,
    pub unit_id: String,
    pub has_hero_attributes: bool,
    pub current_level: u32,
    pub is_open: bool,
    pub selected_hero_level: Signal<u32>,
    pub level_picker_open: Signal<bool>,
}
