use super::super::conflict_ability_icon::ConflictAbilityIconProps;
use dioxus::prelude::*;

/// The icon button that opens the carrying unit: the ability icon it shows and the
/// click handler that navigates to that unit.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictAbilityTriggerProps {
    pub onclick: EventHandler<MouseEvent>,
    pub icon: ConflictAbilityIconProps,
}
