mod props;
mod style;

use dioxus::prelude::*;
pub use props::ToggleButtonProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ToggleButton);

/// The shared labeled pill button (mode, search-field, catalog-visibility). Its look
/// is its own; its size is the parent's, flowing through the box it fills — see
/// `style.rs`.
#[component]
pub fn ToggleButton(props: ToggleButtonProps) -> Element {
    let label = props.label;
    let active = props.active;
    let title = props.title;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "data-active": active,
            aria_pressed: active,
            title,
            onclick,
            onkeydown,
            {label}
        }
    }
}
