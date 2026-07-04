mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::FightRowProps;
use style::CLASS;
assert_component!(FightRow);
#[component]
pub fn FightRow(props: FightRowProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
