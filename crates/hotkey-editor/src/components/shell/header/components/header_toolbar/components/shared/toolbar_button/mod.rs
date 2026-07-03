pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::toolbar_button_icon::{ToolbarButtonIcon, ToolbarButtonIconProps};
use dioxus::prelude::*;
pub use props::ToolbarButtonProps;
use style::CLASS;
assert_component!(ToolbarButton);

/// The single source of truth for how a toolbar action button looks. Consumers swap
/// only the icon, the click handler, and aria/disabled state.
#[component]
pub fn ToolbarButton(props: ToolbarButtonProps) -> Element {
    let glyph = ToolbarButtonIconProps::from(&props);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: props.aria_label,
            aria_haspopup: props.aria_haspopup,
            aria_expanded: props.aria_expanded,
            aria_pressed: props.aria_pressed,
            "data-action": props.data_action,
            disabled: props.disabled,
            onclick: props.onclick,
            ToolbarButtonIcon { ..glyph }
        }
    }
}
