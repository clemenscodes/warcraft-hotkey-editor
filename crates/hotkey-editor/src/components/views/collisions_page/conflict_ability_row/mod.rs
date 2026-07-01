mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ConflictAbilityRowProps;
use style::CLASS;
assert_component!(ConflictAbilityRow);
#[component]
pub fn ConflictAbilityRow(props: ConflictAbilityRowProps) -> Element {
    let is_multi = props.is_multi;
    let children = props.children;
    rsx! { div { class: CLASS, "data-multi": is_multi, {children} } }
}
