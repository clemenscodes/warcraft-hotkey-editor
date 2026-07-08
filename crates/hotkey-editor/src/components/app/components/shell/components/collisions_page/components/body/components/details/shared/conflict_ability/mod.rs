mod props;
mod style;

use super::conflict_ability_icon::ConflictAbilityIconProps;
use super::conflict_ability_name::ConflictAbilityName;
use super::conflict_ability_trigger::{ConflictAbilityTrigger, ConflictAbilityTriggerProps};
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use tw_macro::assert_component;
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
    let trigger = ConflictAbilityTriggerProps { onclick, icon };
    rsx! {
        div {
            class: CLASS,
            ConflictAbilityTrigger { ..trigger }
            ConflictAbilityName { text: ability_name }
            ConflictObjectId { text: ability_id }
        }
    }
}
