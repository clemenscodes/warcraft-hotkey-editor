use dioxus::prelude::*;

/// The unit detail header: the portrait and title, plus the hero-level picker state
/// signals for units with hero attributes.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailHeaderProps {
    pub unit_name: &'static str,
    pub unit_id: String,
    pub portrait_url: Option<String>,
    pub has_hero_attributes: bool,
    pub selected_hero_level: Signal<u32>,
    pub level_picker_open: Signal<bool>,
}
