mod props;
mod state;
mod style;

use dioxus::prelude::*;
pub use props::LayoutTileProps;
pub use state::LayoutTileState;
use tw_macro::assert_component;
assert_component!(LayoutTile);

/// A single editable cell in the global hotkey grid. Draggable to swap, clickable
/// to open the key picker, and pulses while being edited.
#[component]
pub fn LayoutTile(props: LayoutTileProps) -> Element {
    let class = style::class(props.state);
    let label = props.label;
    let row = props.row;
    let column = props.column;
    let ondragstart = props.ondragstart;
    let ondragend = props.ondragend;
    let ondragover = props.ondragover;
    let ondrop = props.ondrop;
    let onclick = props.onclick;
    rsx! {
        button {
            class,
            draggable: "true",
            "data-layout-row": row,
            "data-layout-col": column,
            ondragstart,
            ondragend,
            ondragover,
            ondrop,
            onclick,
            {label}
        }
    }
}
