mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

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
