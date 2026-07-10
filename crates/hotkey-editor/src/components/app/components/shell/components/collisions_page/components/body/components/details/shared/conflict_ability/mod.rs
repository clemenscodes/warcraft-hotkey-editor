mod hooks;
mod props;
mod style;

use super::conflict_ability_name::ConflictAbilityName;
use super::conflict_ability_trigger::ConflictAbilityTrigger;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use hooks::{ConflictAbilityModel, use_conflict_ability};
use dioxus::prelude::*;
use props::ConflictAbilityProps;
use style::CLASS;
use tw_macro::assert_component;

/// One ability of a conflict: icon (deep-link button), name, and object id. The icon
/// button opens the owning unit through the navigation read from context.
#[component]
pub fn ConflictAbility(props: ConflictAbilityProps) -> Element {
    let ConflictAbilityModel {
        onclick,
        icon_src,
        icon_alt,
        name,
        ability_id,
    } = use_conflict_ability(&props);
    rsx! {
        div {
            class: CLASS,
            ConflictAbilityTrigger {
                onclick,
                icon_src,
                icon_alt,
            }
            ConflictAbilityName { text: name }
            ConflictObjectId { object_id: ability_id }
        }
    }
}

assert_component!(ConflictAbility);
