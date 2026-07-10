pub mod components;
mod props;
mod style;

use components::grid_layout_button_icon::GridLayoutButtonIcon;
use components::grid_layout_button_label::GridLayoutButtonLabel;
use dioxus::prelude::*;
use props::GridLayoutButtonProps;
use style::CLASS;
use tw_macro::assert_component;

/// Prominent call-to-action that opens the global grid-layout editor. Deliberately
/// styled apart from the icon-only toolbar buttons. Presentational: its open state
/// and toggle handler arrive as props.
#[component]
pub fn GridLayoutButton(props: GridLayoutButtonProps) -> Element {
    let is_open = props.is_open;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Edit global hotkey layout",
            aria_haspopup: "dialog",
            aria_expanded: is_open,
            onclick,
            GridLayoutButtonIcon {}
            GridLayoutButtonLabel {}
        }
    }
}

assert_component!(GridLayoutButton);
