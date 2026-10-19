mod model;
mod view;

mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::coordinate::Coordinate;
use dioxus::prelude::*;
use model::IslandConflictMetaModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IslandConflictMeta(props: IslandConflictMetaModel) -> Element {
    let coordinate = props.coordinate;
    let count = props.count;
    rsx! {
        div {
            class: CLASS,
            Coordinate {
                coordinate,
            }
            CollisionCount {
                count,
            }
        }
    }
}

assert_component!(IslandConflictMeta);
