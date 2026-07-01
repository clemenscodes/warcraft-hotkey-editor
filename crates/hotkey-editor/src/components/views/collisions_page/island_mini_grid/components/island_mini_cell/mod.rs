mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::IslandMiniCellProps;
use style::CLASS;
assert_component!(IslandMiniCell);

/// One cell of the island mini grid; highlighted when it is the collision cell.
#[component]
pub fn IslandMiniCell(props: IslandMiniCellProps) -> Element {
    let is_collision = props.is_collision;
    rsx! {
        div {
            class: CLASS,
            "data-collision": is_collision,
        }
    }
}
