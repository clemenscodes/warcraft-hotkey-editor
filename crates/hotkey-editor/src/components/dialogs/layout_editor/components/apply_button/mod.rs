mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::ApplyButtonProps;
use style::CLASS;
assert_component!(ApplyButton);

/// The footer action that rewrites every ability hotkey to match the grid.
#[component]
pub fn ApplyButton(props: ApplyButtonProps) -> Element {
    let onclick = props.on_apply;
    rsx! {
        button { class: CLASS, r#type: "button", onclick, "Apply grid to all hotkeys" }
    }
}
