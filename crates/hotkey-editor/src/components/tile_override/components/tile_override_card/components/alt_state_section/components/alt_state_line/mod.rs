mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::AltStateLineProps;

assert_component!(AltStateLine);

/// One description line under the alt-state header.
#[component]
pub fn AltStateLine(props: AltStateLineProps) -> Element {
    let text = props.text;
    rsx! {
        p { class: CLASS, {text} }
    }
}
