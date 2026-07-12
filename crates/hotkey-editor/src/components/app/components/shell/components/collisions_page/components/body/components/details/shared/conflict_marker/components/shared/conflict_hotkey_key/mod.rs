mod model;
mod view;

pub use view::ConflictHotkeyKeyView;
mod style;
use dioxus::prelude::*;
use model::ConflictHotkeyKeyModel;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn ConflictHotkeyKey(props: ConflictHotkeyKeyModel) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}

assert_component!(ConflictHotkeyKey);
