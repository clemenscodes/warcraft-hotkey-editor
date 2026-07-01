mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::CollisionBreadcrumbLabelProps;
use style::CLASS;
assert_component!(CollisionBreadcrumbLabel);
#[component]
pub fn CollisionBreadcrumbLabel(props: CollisionBreadcrumbLabelProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
