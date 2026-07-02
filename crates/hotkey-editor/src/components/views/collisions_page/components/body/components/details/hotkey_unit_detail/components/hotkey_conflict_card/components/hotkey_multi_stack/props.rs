use crate::components::views::collisions_page::components::body::components::conflict_ability::ConflictAbilityProps;
use dioxus::prelude::*;

/// The multi-way clash layout: the badge stacked above every clashing ability, or
/// nothing when the clash is a two-ability pair.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyMultiStackProps {
    pub abilities: Vec<ConflictAbilityProps>,
    #[props(into)]
    pub hotkey_label: String,
}
