mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ConflictHotkeyBadgeProps;
use style::CLASS;
assert_component!(ConflictHotkeyBadge);
#[component]
pub fn ConflictHotkeyBadge(props: ConflictHotkeyBadgeProps) -> Element {
    let children = props.children;
    rsx! { span { class: CLASS, {children} } }
}
