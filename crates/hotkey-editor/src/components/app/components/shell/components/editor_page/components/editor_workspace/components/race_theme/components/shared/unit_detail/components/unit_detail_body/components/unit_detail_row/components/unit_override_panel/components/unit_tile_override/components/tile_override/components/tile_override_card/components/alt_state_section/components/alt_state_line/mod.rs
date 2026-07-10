mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

pub use props::AltStateLineProps;

/// One description line under the alt-state header.
#[component]
pub fn AltStateLine(props: AltStateLineProps) -> Element {
    let text = props.text;
    rsx! {
        p { class: CLASS, {text} }
    }
}

assert_component!(AltStateLine);
