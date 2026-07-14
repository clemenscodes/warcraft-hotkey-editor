mod model;
mod presentation;
mod view;

pub use view::ConflictAbilityView;
mod style;

use super::conflict_ability_name::ConflictAbilityName;
use super::conflict_ability_trigger::ConflictAbilityTrigger;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use presentation::{ConflictAbilityPresentation, use_conflict_ability};
use dioxus::prelude::*;
use model::ConflictAbilityModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ConflictAbility(props: ConflictAbilityModel) -> Element {
    let ConflictAbilityPresentation {
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
            ConflictAbilityName {
                text: name,
            }
            ConflictObjectId {
                object_id: ability_id,
            }
        }
    }
}

assert_component!(ConflictAbility);
