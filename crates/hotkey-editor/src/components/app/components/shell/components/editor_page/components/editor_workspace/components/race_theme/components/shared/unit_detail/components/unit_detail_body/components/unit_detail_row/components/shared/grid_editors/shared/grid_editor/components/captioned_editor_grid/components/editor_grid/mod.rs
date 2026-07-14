pub mod components;
mod model;
mod view;

pub use view::EditorGridView;
mod style;

use components::grid_editor_tile::{EditorTile, GridEditorTile};
use dioxus::prelude::*;
use model::EditorGridModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn EditorGrid(props: EditorGridModel) -> Element {
    let tiles = props.tiles;
    rsx! {
        div {
            class: CLASS,
            for EditorTile {
        coordinate,
        icon,
        label,
        hotkey,
        badge_state,
        state,
        is_dragging_source,
        is_drag_over,
        is_focusable,
        draggable,
        onkeydown,
        onpointerdown,
        onpointermove,
        onpointerup,
        onpointercancel,
        onlostpointercapture,
        onclick,
        ondoubleclick,
    } in tiles {
                GridEditorTile {
                    coordinate,
                    icon,
                    label,
                    hotkey,
                    badge_state,
                    state,
                    is_dragging_source,
                    is_drag_over,
                    is_focusable,
                    draggable,
                    onkeydown,
                    onpointerdown,
                    onpointermove,
                    onpointerup,
                    onpointercancel,
                    onlostpointercapture,
                    onclick,
                    ondoubleclick,
                }
            }
        }
    }
}

assert_component!(EditorGrid);
