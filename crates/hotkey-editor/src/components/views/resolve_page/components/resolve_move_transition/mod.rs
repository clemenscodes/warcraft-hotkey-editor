mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveMoveTransitionProps;
use style::CLASS;
assert_component!(ResolveMoveTransition);

/// The from → to grid block below a move's abilities.
#[component]
pub fn ResolveMoveTransition(props: ResolveMoveTransitionProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
