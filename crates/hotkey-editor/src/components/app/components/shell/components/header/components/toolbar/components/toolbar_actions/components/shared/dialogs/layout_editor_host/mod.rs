pub mod components;
mod hooks;
mod style;

use components::layout_editor::LayoutEditor;
use dioxus::prelude::*;
use hooks::use_layout_editor_host;
use style::CLASS;
use tw_macro::assert_component;

/// Connects the global hotkey-layout editor to app state and places it in the
/// always-mounted toolbar, so it opens from either the centered grid-layout button
/// (laptop and up) or the burger drawer. The dialog self-gates on the shared open
/// signal.
#[component]
pub fn LayoutEditorHost() -> Element {
    let inputs = use_layout_editor_host();
    let grid_layout = inputs.grid_layout;
    let editing_layout_tile = inputs.editing_layout_tile;
    let dragging_layout_tile = inputs.dragging_layout_tile;
    let update_hotkeys_on_move = inputs.update_hotkeys_on_move;
    let open = inputs.open;
    rsx! {
        div {
            class: CLASS,
            LayoutEditor {
                grid_layout,
                editing_layout_tile,
                dragging_layout_tile,
                update_hotkeys_on_move,
                open,
            }
        }
    }
}

assert_component!(LayoutEditorHost);
