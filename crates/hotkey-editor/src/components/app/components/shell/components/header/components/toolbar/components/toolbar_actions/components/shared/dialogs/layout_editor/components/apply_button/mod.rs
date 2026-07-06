mod data;
mod props;
mod style;

use dioxus::prelude::*;
pub use props::ApplyButtonProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ApplyButton);

/// The footer action that rewrites every ability hotkey to match the grid.
#[component]
pub fn ApplyButton(props: ApplyButtonProps) -> Element {
    let onclick = props.on_apply;
    let label = data::LABEL;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            {label}
        }
    }
}
