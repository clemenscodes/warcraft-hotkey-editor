mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::SearchFieldButtonProps;

assert_component!(SearchFieldButton);

/// One button of the search-field toggle.
#[component]
pub fn SearchFieldButton(props: SearchFieldButtonProps) -> Element {
    let label = props.label;
    let is_active = props.is_active;
    let on_select = props.on_select;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "data-active": is_active,
            aria_pressed: is_active,
            onclick: on_select,
            {label}
        }
    }
}
