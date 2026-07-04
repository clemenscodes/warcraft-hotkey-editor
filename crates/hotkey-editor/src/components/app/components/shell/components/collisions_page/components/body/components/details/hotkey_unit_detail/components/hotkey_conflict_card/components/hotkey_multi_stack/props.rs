use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability::ConflictAbilityProps;
use dioxus::prelude::*;

/// The multi-way clash layout: the badge stacked above every clashing ability, or
/// nothing when the clash is a two-ability pair.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyMultiStackProps {
    pub abilities: Vec<ConflictAbilityProps>,
    #[props(into)]
    pub hotkey_label: String,
}
