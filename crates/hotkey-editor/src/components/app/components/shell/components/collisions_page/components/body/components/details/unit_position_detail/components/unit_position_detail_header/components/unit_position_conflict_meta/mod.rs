mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_unit_name::ConflictUnitName;
use dioxus::prelude::*;
pub use props::UnitPositionConflictMetaProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitPositionConflictMeta);

/// The text meta column of the position-collision detail header: the unit's name, its
/// object id, and its collision count.
#[component]
pub fn UnitPositionConflictMeta(props: UnitPositionConflictMetaProps) -> Element {
    let name = props.name;
    let unit_id_label = props.unit_id_label;
    let count = props.count;
    rsx! {
        div {
            class: CLASS,
            ConflictUnitName { text: name }
            ConflictObjectId { text: unit_id_label }
            CollisionCount { count }
        }
    }
}
