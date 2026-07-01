mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ConflictAbilityTriggerProps;
use style::CLASS;
assert_component!(ConflictAbilityTrigger);
/// The clickable wrapper around a conflict ability's icon.
#[component]
pub fn ConflictAbilityTrigger(props: ConflictAbilityTriggerProps) -> Element {
    let onclick = props.onclick;
    let children = props.children;
    rsx! {
        button { class: CLASS, r#type: "button", onclick, {children} }
    }
}
