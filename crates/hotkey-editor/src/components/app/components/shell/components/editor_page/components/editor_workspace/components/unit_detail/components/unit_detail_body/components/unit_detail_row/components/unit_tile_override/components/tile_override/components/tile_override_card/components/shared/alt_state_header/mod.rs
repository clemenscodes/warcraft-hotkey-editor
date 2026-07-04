mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::AltStateHeaderProps;

assert_component!(AltStateHeader);

/// The top row of the alt-state block.
#[component]
pub fn AltStateHeader(props: AltStateHeaderProps) -> Element {
    let children = props.children;
    rsx! {
        div { class: CLASS, {children} }
    }
}
