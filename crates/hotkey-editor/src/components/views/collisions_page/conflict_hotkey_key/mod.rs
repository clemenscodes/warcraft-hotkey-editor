mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ConflictHotkeyKeyProps;
use style::CLASS;
assert_component!(ConflictHotkeyKey);
#[component]
pub fn ConflictHotkeyKey(props: ConflictHotkeyKeyProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
