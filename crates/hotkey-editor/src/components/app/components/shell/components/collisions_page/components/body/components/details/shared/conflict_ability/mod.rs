mod hooks;
mod props;
mod style;

use super::conflict_ability_name::ConflictAbilityName;
use super::conflict_ability_trigger::ConflictAbilityTrigger;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use hooks::use_conflict_ability;
use tw_macro::assert_component;
use dioxus::prelude::*;
pub use props::ConflictAbilityProps;
use style::CLASS;
assert_component!(ConflictAbility);

/// One ability of a conflict: icon (deep-link button), name, and object id. The icon
/// button opens the owning unit through the navigation read from context.
#[component]
pub fn ConflictAbility(props: ConflictAbilityProps) -> Element {
    let model = use_conflict_ability(&props);
    rsx! {
        div {
            class: CLASS,
            ConflictAbilityTrigger { ..model.trigger }
            ConflictAbilityName { text: model.name }
            ConflictObjectId { object_id: model.ability_id }
        }
    }
}
