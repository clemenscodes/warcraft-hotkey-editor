pub mod components;
mod model;
mod presentation;
mod view;

pub use view::IslandConflictAbilityView;

mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability_name::ConflictAbilityName;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability_trigger::ConflictAbilityTrigger;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use crate::components::app::components::shell::components::shared::carriers_dialog_host::CarriersDialogHost;
use components::conflict_more::ConflictMore;
use dioxus::prelude::*;
use presentation::{IslandConflictAbilityPresentation, use_island_conflict_ability};
use model::IslandConflictAbilityModel;
use style::CLASS;
use tw_macro::assert_component;

/// One ability column of an island conflict: a clickable icon over the name and id,
/// with an optional "+N more" link. Both the icon and the link open this ability's
/// carriers dialog. The column owns the open state and mounts the dialog's host beneath
/// itself, so no ancestor knows the dialog exists.
#[component]
pub fn IslandConflictAbility(props: IslandConflictAbilityModel) -> Element {
    let IslandConflictAbilityPresentation {
        mut open_state,
        icon_src,
        icon_alt,
        onclick,
        ability_name,
        ability_id,
        extra_count,
    } = use_island_conflict_ability(&props);
    rsx! {
        div {
            class: CLASS,
            ConflictAbilityTrigger { onclick, icon_src, icon_alt }
            ConflictAbilityName { text: ability_name }
            ConflictObjectId { object_id: ability_id }
            if extra_count > 0 {
                ConflictMore { count: extra_count, onclick }
            }
        }
        CarriersDialogHost {
            ability: open_state.read().clone(),
            on_close: Callback::new(move |()| open_state.set(None)),
        }
    }
}

assert_component!(IslandConflictAbility);
