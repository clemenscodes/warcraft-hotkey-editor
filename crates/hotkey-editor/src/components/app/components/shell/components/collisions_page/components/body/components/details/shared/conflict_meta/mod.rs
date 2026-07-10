mod props;
mod view;

pub use view::ConflictMetaView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_unit_name::ConflictUnitName;
use dioxus::prelude::*;
use props::ConflictMetaProps;
use style::CLASS;
use tw_macro::assert_component;

/// The text meta column of a detail-pane header: the unit name, its object id, and the
/// collision count. Shared by the hotkey and unit-position detail headers.
#[component]
pub fn ConflictMeta(props: ConflictMetaProps) -> Element {
    let name = props.name;
    let unit_id = props.unit_id;
    let count = props.count;
    rsx! {
        div {
            class: CLASS,
            ConflictUnitName { text: name }
            ConflictObjectId { object_id: unit_id }
            CollisionCount { count }
        }
    }
}

assert_component!(ConflictMeta);
