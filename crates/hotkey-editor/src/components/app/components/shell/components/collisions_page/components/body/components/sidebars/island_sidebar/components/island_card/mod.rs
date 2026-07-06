mod props;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::coordinate::Coordinate;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::{MiniGrid, MiniGridProps};
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::row_meta::RowMeta;
use dioxus::prelude::*;
pub use props::IslandCardProps;
use style::CLASS;
assert_component!(IslandCard);

/// A selectable island-collision card: mini grid plus the coordinate and
/// collision-count meta.
#[component]
pub fn IslandCard(props: IslandCardProps) -> Element {
    let is_selected = props.is_selected;
    let collision_key = props.collision_key;
    let onclick = props.onclick;
    let coordinate = props.coordinate;
    let count = props.count;
    let grid = MiniGridProps { coordinate };
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "data-selected": is_selected,
            "data-collision-key": collision_key,
            onclick,
            MiniGrid { ..grid }
            RowMeta {
                Coordinate { coordinate }
                CollisionCount { count }
            }
        }
    }
}
