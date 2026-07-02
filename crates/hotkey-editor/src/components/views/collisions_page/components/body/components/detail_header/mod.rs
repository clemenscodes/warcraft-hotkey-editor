mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::DetailHeaderProps;
use style::CLASS;
assert_component!(DetailHeader);
#[component]
pub fn DetailHeader(props: DetailHeaderProps) -> Element {
    let children = props.children;
    rsx! { header { class: CLASS, {children} } }
}
