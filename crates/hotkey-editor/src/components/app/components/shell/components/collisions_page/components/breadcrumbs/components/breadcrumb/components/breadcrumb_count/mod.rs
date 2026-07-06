mod props;
mod style;
use dioxus::prelude::*;
pub use props::BreadcrumbCountProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(BreadcrumbCount);

/// A breadcrumb tab's live collision count, parenthesised via `::before`/`::after`.
#[component]
pub fn BreadcrumbCount(props: BreadcrumbCountProps) -> Element {
    let count = props.count;
    rsx! { span { class: CLASS, "{count}" } }
}
