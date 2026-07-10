pub mod components;
mod hooks;
mod props;
mod view;

pub use view::IslandConflictAbilityView;

mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability_name::ConflictAbilityName;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability_trigger::ConflictAbilityTrigger;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use crate::components::app::components::shell::components::shared::carriers_dialog_host::CarriersDialogHost;
use components::conflict_more::ConflictMore;
use dioxus::prelude::*;
use hooks::{IslandConflictAbilityModel, use_island_conflict_ability};
use props::IslandConflictAbilityProps;
use style::CLASS;
use tw_macro::assert_component;

/// One ability column of an island conflict: a clickable icon over the name and id,
/// with an optional "+N more" link. Both the icon and the link open this ability's
/// carriers dialog. The column owns the open state and mounts the dialog's host beneath
/// itself, so no ancestor knows the dialog exists.
#[component]
pub fn IslandConflictAbility(props: IslandConflictAbilityProps) -> Element {
    let IslandConflictAbilityModel {
        open_state,
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
        CarriersDialogHost { open_state }
    }
}

assert_component!(IslandConflictAbility);
