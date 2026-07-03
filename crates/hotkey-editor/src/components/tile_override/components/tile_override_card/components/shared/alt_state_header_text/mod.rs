mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::AltStateHeaderTextProps;

assert_component!(AltStateHeaderText);

/// The label column of the alt-state header.
#[component]
pub fn AltStateHeaderText(props: AltStateHeaderTextProps) -> Element {
    let children = props.children;
    rsx! {
        div { class: CLASS, {children} }
    }
}
