mod props;
mod style;
use dioxus::prelude::*;
pub use props::FightRowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(FightRow);
#[component]
pub fn FightRow(props: FightRowProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
