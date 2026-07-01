mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::CollisionBreadcrumbCountProps;
use style::CLASS;
assert_component!(CollisionBreadcrumbCount);

/// A breadcrumb tab's live collision count, parenthesised via `::before`/`::after`.
#[component]
pub fn CollisionBreadcrumbCount(props: CollisionBreadcrumbCountProps) -> Element {
    let count = props.count;
    rsx! { span { class: CLASS, "{count}" } }
}
