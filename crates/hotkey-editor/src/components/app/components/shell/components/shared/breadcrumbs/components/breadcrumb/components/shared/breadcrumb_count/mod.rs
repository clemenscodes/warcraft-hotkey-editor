mod props;
mod view;

pub use view::BreadcrumbCountView;
mod style;
use dioxus::prelude::*;
use props::BreadcrumbCountProps;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn BreadcrumbCount(props: BreadcrumbCountProps) -> Element {
    let count = props.count;
    rsx! { span { class: CLASS, "{count}" } }
}

assert_component!(BreadcrumbCount);
