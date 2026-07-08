mod props;
mod style;
use dioxus::prelude::*;
pub use props::DetailHeaderProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(DetailHeader);
#[component]
pub fn DetailHeader(props: DetailHeaderProps) -> Element {
    let children = props.children;
    rsx! { header { class: CLASS, {children} } }
}
