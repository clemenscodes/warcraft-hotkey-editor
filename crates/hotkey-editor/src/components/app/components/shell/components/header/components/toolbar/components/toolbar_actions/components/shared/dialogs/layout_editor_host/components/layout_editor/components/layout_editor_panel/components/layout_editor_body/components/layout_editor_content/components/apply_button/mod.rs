mod data;
mod props;
mod style;

use dioxus::prelude::*;
pub use props::ApplyButtonProps;
use style::CLASS;
use tw_macro::assert_component;

/// The action that rewrites every ability hotkey to match the grid, centered
/// in the editor body below the grid.
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

assert_component!(ApplyButton);
