pub mod components;
mod hooks;
mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use components::conflict_unit_icon::ConflictUnitIcon;
use components::island_conflict_unit_name::IslandConflictUnitName;
use dioxus::prelude::*;
use hooks::use_island_conflict_unit;
pub use props::IslandConflictUnitProps;
use style::CLASS;
use tw_macro::assert_component;

/// The affected unit heading a conflict card: a clickable icon over the name and
/// object id, all deep-linking into the editor focused on that unit through the
/// navigation read from context.
#[component]
pub fn IslandConflictUnit(props: IslandConflictUnitProps) -> Element {
    let model = use_island_conflict_unit(&props);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick: model.onclick,
            ConflictUnitIcon { ..model.icon }
            IslandConflictUnitName { text: model.name }
            ConflictObjectId { object_id: model.unit_id }
        }
    }
}

assert_component!(IslandConflictUnit);
