mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveBreadcrumbCountProps;
use style::CLASS;
assert_component!(ResolveBreadcrumbCount);
#[component]
pub fn ResolveBreadcrumbCount(props: ResolveBreadcrumbCountProps) -> Element {
    let count = props.count;
    rsx! { span { class: CLASS, "{count}" } }
}
