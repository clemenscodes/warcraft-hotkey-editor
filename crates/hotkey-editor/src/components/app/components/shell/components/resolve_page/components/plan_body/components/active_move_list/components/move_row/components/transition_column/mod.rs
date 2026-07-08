mod props;
mod style;
use dioxus::prelude::*;
pub use props::TransitionColumnProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(TransitionColumn);
#[component]
pub fn TransitionColumn(props: TransitionColumnProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
