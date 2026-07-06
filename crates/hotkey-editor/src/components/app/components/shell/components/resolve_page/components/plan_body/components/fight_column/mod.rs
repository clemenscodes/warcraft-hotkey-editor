mod props;
mod style;
use dioxus::prelude::*;
pub use props::FightColumnProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(FightColumn);
#[component]
pub fn FightColumn(props: FightColumnProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
