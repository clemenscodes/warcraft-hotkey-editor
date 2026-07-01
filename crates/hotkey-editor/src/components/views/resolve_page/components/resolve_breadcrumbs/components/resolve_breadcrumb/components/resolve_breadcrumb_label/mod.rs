mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveBreadcrumbLabelProps;
use style::CLASS;
assert_component!(ResolveBreadcrumbLabel);
#[component]
pub fn ResolveBreadcrumbLabel(props: ResolveBreadcrumbLabelProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
