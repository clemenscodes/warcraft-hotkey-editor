mod model;
mod view;
pub use view::LayoutTileView;
mod state;
mod style;

use crate::components::app::components::shell::components::shared::editable_keycap::{
    EditableKeycap, EditableKeycapState,
};
use dioxus::prelude::*;
use model::LayoutTileModel;
pub use state::LayoutTileState;
use style::CLASS;
use tw_macro::assert_component;

/// A single editable cell in the global hotkey grid. Draggable to swap, clickable
/// to open the key picker, and pulses while being edited. The focusable, keyboard-
/// navigable host: it owns the drag/click behaviour, the grid address, focus, and the
/// font size, and wraps the shared `EditableKeycap` that draws the gold cap and pulse.
#[component]
pub fn LayoutTile(props: LayoutTileModel) -> Element {
    let ondragstart = props.ondragstart;
    let ondragend = props.ondragend;
    let ondragover = props.ondragover;
    let ondrop = props.ondrop;
    let onclick = props.onclick;
    let label = props.label;
    let keycap_state = match props.state {
        LayoutTileState::Idle => EditableKeycapState::Idle,
        LayoutTileState::Editing => EditableKeycapState::Editing,
    };
    rsx! {
        button {
            class: CLASS,
            draggable: "true",
            ondragstart,
            ondragend,
            ondragover,
            ondrop,
            onclick,
            EditableKeycap { label, state: keycap_state }
        }
    }
}

assert_component!(LayoutTile);
