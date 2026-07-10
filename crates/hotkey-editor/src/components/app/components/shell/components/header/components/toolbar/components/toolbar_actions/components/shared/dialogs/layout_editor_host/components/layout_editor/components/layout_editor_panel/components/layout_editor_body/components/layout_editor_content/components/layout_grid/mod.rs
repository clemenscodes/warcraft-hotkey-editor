pub mod components;
mod props;
mod view;

pub use view::LayoutGridView;
mod style;

use components::layout_tile::LayoutTile;
use dioxus::prelude::*;
use props::LayoutGridProps;
use style::CLASS;
use tw_macro::assert_component;

/// The four-by-three grid of editable hotkey cells.
#[component]
pub fn LayoutGrid(props: LayoutGridProps) -> Element {
    let cells = props.cells;
    rsx! {
        div { class: CLASS,
            for cell in cells {
                LayoutTile {
                    state: cell.state,
                    label: cell.label,
                    coordinate: cell.coordinate,
                    ondragstart: cell.ondragstart,
                    ondragend: cell.ondragend,
                    ondragover: cell.ondragover,
                    ondrop: cell.ondrop,
                    onclick: cell.onclick,
                }
            }
        }
    }
}

assert_component!(LayoutGrid);
