mod model;
mod view;

pub use view::CollisionCardPrimaryView;

use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::coordinate::Coordinate;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_unit_name::ConflictUnitName;
use dioxus::prelude::*;
use tw_macro::assert_component;
use model::CollisionCardPrimaryModel;

/// The primary meta of a collision card: the unit's name and object id for a unit
/// card, or the highlighted coordinate for an island card. A dedicated switch on
/// the card content, so the card's meta column stays a flat list of children.
#[component]
pub fn CollisionCardPrimary(props: CollisionCardPrimaryModel) -> Element {
    match props.content {
        CollisionCardContent::Unit { name, unit_id, .. } => rsx! {
            ConflictUnitName {
                text: name,
            }
            ConflictObjectId {
                object_id: unit_id,
            }
        },
        CollisionCardContent::Island { coordinate } => rsx! {
            Coordinate {
                coordinate,
            }
        },
    }
}

assert_component!(CollisionCardPrimary);
