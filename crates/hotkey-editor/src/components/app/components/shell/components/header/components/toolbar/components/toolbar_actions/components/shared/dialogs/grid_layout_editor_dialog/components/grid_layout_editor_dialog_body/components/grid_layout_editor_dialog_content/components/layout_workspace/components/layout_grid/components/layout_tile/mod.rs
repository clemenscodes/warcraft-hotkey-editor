mod model;
mod presentation;
mod view;
pub use view::LayoutTileView;
mod state;
mod style;

use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycap;
use dioxus::prelude::*;
use model::LayoutTileModel;
use presentation::LayoutTilePresentation;
pub use state::LayoutTileState;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn LayoutTile(props: LayoutTileModel) -> Element {
    let LayoutTilePresentation {
        label,
        keycap_state,
        ondragstart,
        ondragend,
        ondragover,
        ondrop,
        onclick,
    } = LayoutTilePresentation::from(&props);
    rsx! {
        button {
            class: CLASS,
            draggable: "true",
            ondragstart,
            ondragend,
            ondragover,
            ondrop,
            onclick,
            EditableKeycap {
                label,
                state: keycap_state,
            }
        }
    }
}

assert_component!(LayoutTile);
