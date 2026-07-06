mod props;
mod style;
use dioxus::prelude::*;
pub use props::ConflictHotkeyBadgeProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictHotkeyBadge);
#[component]
pub fn ConflictHotkeyBadge(props: ConflictHotkeyBadgeProps) -> Element {
    let is_top = props.is_top;
    let children = props.children;
    rsx! { span { class: CLASS, "data-top": is_top, {children} } }
}
