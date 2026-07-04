mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::FightColumnProps;
use style::CLASS;
assert_component!(FightColumn);
#[component]
pub fn FightColumn(props: FightColumnProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
