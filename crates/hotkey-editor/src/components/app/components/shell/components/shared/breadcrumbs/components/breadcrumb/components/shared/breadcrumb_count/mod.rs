mod props;
mod style;
use dioxus::prelude::*;
pub use props::BreadcrumbCountProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(BreadcrumbCount);
#[component]
pub fn BreadcrumbCount(props: BreadcrumbCountProps) -> Element {
    let count = props.count;
    rsx! { span { class: CLASS, "{count}" } }
}
