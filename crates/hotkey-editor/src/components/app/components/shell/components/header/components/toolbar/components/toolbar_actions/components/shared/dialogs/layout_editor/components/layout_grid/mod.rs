pub mod components;
mod props;
mod style;

use components::layout_cell::LayoutCell;
use dioxus::prelude::*;
pub use props::LayoutGridProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(LayoutGrid);

/// The four-by-three grid of editable hotkey cells.
#[component]
pub fn LayoutGrid(props: LayoutGridProps) -> Element {
    let cells = props.cells;
    rsx! {
        div { class: CLASS,
            for cell in cells {
                LayoutCell { ..cell }
            }
        }
    }
}
