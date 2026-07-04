mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::BreadcrumbCountProps;
use style::CLASS;
assert_component!(BreadcrumbCount);
#[component]
pub fn BreadcrumbCount(props: BreadcrumbCountProps) -> Element {
    let count = props.count;
    rsx! { span { class: CLASS, "{count}" } }
}
