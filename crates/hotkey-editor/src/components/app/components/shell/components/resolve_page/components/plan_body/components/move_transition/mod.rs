mod props;
mod style;
use dioxus::prelude::*;
pub use props::MoveTransitionProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MoveTransition);

/// The from → to grid block below a move's abilities.
#[component]
pub fn MoveTransition(props: MoveTransitionProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
