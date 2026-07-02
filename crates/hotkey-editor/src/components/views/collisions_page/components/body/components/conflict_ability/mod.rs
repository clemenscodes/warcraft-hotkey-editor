mod props;
mod style;

use super::conflict_ability_icon::{ConflictAbilityIcon, ConflictAbilityIconProps};
use super::conflict_ability_name::ConflictAbilityName;
use super::conflict_ability_trigger::ConflictAbilityTrigger;
use super::conflict_object_id::ConflictObjectId;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ConflictAbilityProps;
use style::CLASS;
assert_component!(ConflictAbility);

/// One ability of a conflict: icon (deep-link button), name, and object id.
#[component]
pub fn ConflictAbility(props: ConflictAbilityProps) -> Element {
    let ability_name = props.ability_name;
    let ability_id = props.ability_id;
    let icon = ConflictAbilityIconProps {
        src: props.icon_url,
        alt: ability_name.clone(),
    };
    let unit_id = props.unit_id;
    let view_navigation = props.view_navigation;
    let onclick = EventHandler::new(move |_event: MouseEvent| view_navigation.open_unit(&unit_id));
    rsx! {
        div {
            class: CLASS,
            ConflictAbilityTrigger {
                onclick,
                ConflictAbilityIcon { ..icon }
            }
            ConflictAbilityName { text: ability_name }
            ConflictObjectId { text: ability_id }
        }
    }
}
