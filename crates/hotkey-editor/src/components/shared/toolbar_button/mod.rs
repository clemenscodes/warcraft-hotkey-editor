pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::toolbar_button_icon::ToolbarButtonIcon;
use dioxus::prelude::*;
pub use props::ToolbarButtonProps;
use style::CLASS;
assert_component!(ToolbarButton);

/// The single source of truth for how a toolbar action button looks. Consumers swap
/// only the icon, the click handler, and aria/disabled state.
#[component]
pub fn ToolbarButton(props: ToolbarButtonProps) -> Element {
    let icon = props.icon;
    let onclick = props.onclick;
    let disabled = props.disabled;
    let attributes = props.attributes;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            disabled,
            onclick,
            ..attributes,
            ToolbarButtonIcon { icon }
        }
    }
}
