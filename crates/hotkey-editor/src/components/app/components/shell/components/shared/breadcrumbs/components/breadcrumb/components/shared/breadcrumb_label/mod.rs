mod props;
mod style;
use dioxus::prelude::*;
pub use props::BreadcrumbLabelProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(BreadcrumbLabel);
#[component]
pub fn BreadcrumbLabel(props: BreadcrumbLabelProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
