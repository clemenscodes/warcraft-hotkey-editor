mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::BreadcrumbLabelProps;
use style::CLASS;
assert_component!(BreadcrumbLabel);
#[component]
pub fn BreadcrumbLabel(props: BreadcrumbLabelProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
