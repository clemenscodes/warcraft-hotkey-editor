mod props;
mod view;

pub use view::UnitListSearchInputView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use props::UnitListSearchInputProps;

/// The search text field in the unit list.
#[component]
pub fn UnitListSearchInput(props: UnitListSearchInputProps) -> Element {
    let value = props.value;
    let placeholder = props.placeholder;
    let on_input = props.on_input;
    let on_keydown = props.on_keydown;
    rsx! {
        input {
            class: CLASS,
            r#type: "search",
            placeholder,
            value: value,
            oninput: move |event| on_input.call(event),
            onkeydown: move |event| on_keydown.call(event),
        }
    }
}

assert_component!(UnitListSearchInput);
