pub mod components;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::grid_layout_editor_dialog::GridLayoutEditorDialog;
use components::grid_layout_editor_button::GridLayoutEditorButton;
use dioxus::prelude::*;
use presentation::use_grid_layout_editor_button;
use style::CLASS;
use tw_macro::assert_component;

/// Connects the global grid-layout button to the overlay state and gives it its
/// centered, laptop-and-up box, carrying the layout editor it opens. Hidden in the
/// compact layout, where the drawer offers the action instead — so this is the desktop
/// trigger and the burger renders its own copy of the editor for the compact layout.
#[component]
pub fn GridLayoutEditorButtonHost() -> Element {
    let button = use_grid_layout_editor_button();
    let is_open = button.is_open;
    let onclick = button.onclick;
    rsx! {
        div {
            class: CLASS,
            GridLayoutEditorButton { is_open, onclick }
            GridLayoutEditorDialog {}
        }
    }
}

assert_component!(GridLayoutEditorButtonHost);
