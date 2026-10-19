mod components;
mod model;
mod presentation;
mod view;

pub use view::GridEditorTileView;
mod style;

use crate::components::app::components::shell::components::shared::tile_face::TileFace;
use components::draggable_marker::DraggableMarker;
use dioxus::prelude::*;
use model::GridEditorTileModel;
pub(crate) use presentation::EditorTile;
use presentation::EditorTileChrome;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn GridEditorTile(props: GridEditorTileModel) -> Element {
    let coordinate = props.coordinate;
    let icon = props.icon.clone();
    let label = props.label.clone();
    let hotkey = props.hotkey;
    let badge_state = props.badge_state;
    let state = props.state;
    let is_dragging_source = props.is_dragging_source;
    let is_drag_over = props.is_drag_over;
    let active = props.draggable;
    let EditorTileChrome {
        tabindex,
        onkeydown,
        onpointerdown,
        onpointermove,
        onpointerup,
        onpointercancel,
        onlostpointercapture,
        onclick,
        ondoubleclick,
    } = EditorTileChrome::from(&props);
    rsx! {
        div {
            class: CLASS,
            tabindex,
            onkeydown,
            onpointerdown,
            onpointermove,
            onpointerup,
            onpointercancel,
            onlostpointercapture,
            onclick,
            ondoubleclick,
            TileFace {
                coordinate,
                icon,
                label,
                hotkey,
                badge_state,
                state,
                is_dragging_source,
                is_drag_over,
            }
            DraggableMarker {
                active,
            }
        }
    }
}

assert_component!(GridEditorTile);
