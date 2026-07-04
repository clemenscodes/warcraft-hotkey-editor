mod logic;
mod props;
mod state;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
use logic::KeyPickerKeyPresentation;
pub use props::KeyPickerKeyProps;
assert_component!(KeyPickerKey);

/// A single key on the picker board: an on-screen keyboard button that assigns
/// its hotkey when clicked. Its whole look and behaviour arrive shaped through
/// `KeyPickerKeyPresentation`; the body only places them.
#[component]
pub fn KeyPickerKey(props: KeyPickerKeyProps) -> Element {
    let KeyPickerKeyPresentation {
        class,
        label,
        data_label,
        title,
        disabled,
        special,
        onclick,
    } = KeyPickerKeyPresentation::from(&props);
    rsx! {
        button {
            class,
            r#type: "button",
            disabled,
            title,
            "data-special": special,
            "data-label": data_label,
            onclick,
            {label}
        }
    }
}
