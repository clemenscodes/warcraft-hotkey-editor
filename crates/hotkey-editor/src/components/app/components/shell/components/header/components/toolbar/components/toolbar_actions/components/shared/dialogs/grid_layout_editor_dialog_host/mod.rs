pub mod components;
mod style;

use components::grid_layout_editor_dialog::GridLayoutEditorDialog;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// Places the global hotkey-layout editor dialog in the always-mounted toolbar, so it
/// opens from either the centered grid-layout button (laptop and up) or the burger
/// drawer. The dialog self-sources its grid, open, and preference state from context
/// and self-gates on the shared open signal; this host is only its classed container.
#[component]
pub fn GridLayoutEditorDialogHost() -> Element {
    rsx! {
        div {
            class: CLASS,
            GridLayoutEditorDialog {}
        }
    }
}

assert_component!(GridLayoutEditorDialogHost);
