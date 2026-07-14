pub mod components;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::grid_layout_editor_dialog::GridLayoutEditorDialog;
use components::grid_layout_editor_button::GridLayoutEditorButton;
use dioxus::prelude::*;
use presentation::use_grid_layout_editor_button;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn GridLayoutEditorButtonHost() -> Element {
    let button = use_grid_layout_editor_button();
    let is_open = button.is_open;
    let onclick = button.onclick;
    let on_open_change = button.on_open_change;
    rsx! {
        div {
            class: CLASS,
            GridLayoutEditorButton {
                is_open,
                onclick,
            }
            GridLayoutEditorDialog {
                open: is_open,
                on_open_change,
            }
        }
    }
}

assert_component!(GridLayoutEditorButtonHost);
