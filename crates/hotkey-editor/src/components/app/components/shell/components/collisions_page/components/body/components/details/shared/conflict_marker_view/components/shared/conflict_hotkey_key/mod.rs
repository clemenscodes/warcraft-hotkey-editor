mod props;
mod style;
use dioxus::prelude::*;
pub use props::ConflictHotkeyKeyProps;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn ConflictHotkeyKey(props: ConflictHotkeyKeyProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}

assert_component!(ConflictHotkeyKey);
