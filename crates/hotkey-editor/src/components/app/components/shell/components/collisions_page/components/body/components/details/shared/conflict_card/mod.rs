mod props;
mod style;
use dioxus::prelude::*;
pub use props::ConflictCardProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictCard);
#[component]
pub fn ConflictCard(props: ConflictCardProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
