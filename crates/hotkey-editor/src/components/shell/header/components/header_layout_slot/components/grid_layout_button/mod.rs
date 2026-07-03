pub mod components;
mod hooks;
mod style;

use crate::assert_component;
use components::grid_layout_button_icon::GridLayoutButtonIcon;
use components::grid_layout_button_label::GridLayoutButtonLabel;
use dioxus::prelude::*;
use hooks::use_grid_layout_button;
use style::CLASS;
assert_component!(GridLayoutButton);

/// Prominent header call-to-action that opens the global grid-layout editor.
/// Deliberately styled apart from the icon-only toolbar buttons.
#[component]
pub fn GridLayoutButton() -> Element {
    let model = use_grid_layout_button();
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Edit global hotkey layout",
            aria_haspopup: "dialog",
            aria_expanded: model
                    .is_open,
            onclick: model.on_toggle,
            GridLayoutButtonIcon {}
            GridLayoutButtonLabel {}
        }
    }
}
