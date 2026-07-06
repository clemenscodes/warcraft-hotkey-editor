mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

pub use props::AltStateContainerProps;

assert_component!(AltStateContainer);

/// The blue-edged off-state / upgraded-form block.
#[component]
pub fn AltStateContainer(props: AltStateContainerProps) -> Element {
    let children = props.children;
    rsx! {
        div { class: CLASS, {children} }
    }
}
