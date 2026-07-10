pub mod components;
mod hooks;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::layout_editor_host::LayoutEditorHost;
use components::grid_layout_button::GridLayoutButton;
use dioxus::prelude::*;
use hooks::use_grid_layout_button;
use style::CLASS;
use tw_macro::assert_component;

/// Connects the global grid-layout button to the overlay state and gives it its
/// centered, laptop-and-up box, carrying the layout editor it opens. Hidden in the
/// compact layout, where the drawer offers the action instead — so this is the desktop
/// trigger and the burger renders its own copy of the editor for the compact layout.
#[component]
pub fn GridLayoutButtonHost() -> Element {
    let button = use_grid_layout_button();
    let is_open = button.is_open;
    let onclick = button.onclick;
    rsx! {
        div {
            class: CLASS,
            GridLayoutButton { is_open, onclick }
        }
        LayoutEditorHost {}
    }
}

assert_component!(GridLayoutButtonHost);
