mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::MiniCellProps;
use style::CLASS;
assert_component!(MiniCell);

/// One cell of the island mini grid; highlighted when it is the collision cell.
#[component]
pub fn MiniCell(props: MiniCellProps) -> Element {
    let is_collision = props.is_collision;
    rsx! {
        div {
            class: CLASS,
            "data-collision": is_collision,
        }
    }
}
