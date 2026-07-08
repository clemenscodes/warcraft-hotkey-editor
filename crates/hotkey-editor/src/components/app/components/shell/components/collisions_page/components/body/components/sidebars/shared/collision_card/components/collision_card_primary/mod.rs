mod props;

use super::super::state::CollisionCardContent;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::coordinate::Coordinate;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_unit_name::ConflictUnitName;
use dioxus::prelude::*;
pub use props::CollisionCardPrimaryProps;
use tw_macro::assert_component;
assert_component!(CollisionCardPrimary);

/// The primary meta of a collision card: the unit's name and object id for a unit
/// card, or the highlighted coordinate for an island card. A dedicated switch on
/// the card content, so the card's meta column stays a flat list of children.
#[component]
pub fn CollisionCardPrimary(props: CollisionCardPrimaryProps) -> Element {
    match props.content {
        CollisionCardContent::Unit { name, unit_id, .. } => rsx! {
            ConflictUnitName { text: name }
            ConflictObjectId { text: unit_id }
        },
        CollisionCardContent::Island { coordinate } => rsx! {
            Coordinate { coordinate }
        },
    }
}
