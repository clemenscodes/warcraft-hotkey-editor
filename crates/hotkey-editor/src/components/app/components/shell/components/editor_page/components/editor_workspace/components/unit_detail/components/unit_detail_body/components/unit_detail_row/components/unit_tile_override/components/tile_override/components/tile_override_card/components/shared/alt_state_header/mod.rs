mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

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
