pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::layout_cell::LayoutCell;
use style::CLASS;

pub use props::LayoutGridProps;

assert_component!(LayoutGrid);

/// The four-by-three grid of editable hotkey cells.
#[component]
pub fn LayoutGrid(props: LayoutGridProps) -> Element {
    let cells = props.cells;
    rsx! {
        div {
            class: CLASS,
            for cell in cells {
                LayoutCell { ..cell }
            }
        }
    }
}
