mod logic;
mod props;
mod state;
mod style;

use dioxus::prelude::*;
use logic::SystemKeyPickerKeyPresentation;
pub use props::SystemKeyPickerKeyProps;
pub use state::SystemKeyPickerKeyState;
use tw_macro::assert_component;
assert_component!(SystemKeyPickerKey);

/// A single key on the system keyboard board: an on-screen keyboard button that
/// assigns its `KeyCode` when clicked. Its look and behaviour arrive shaped through
/// `SystemKeyPickerKeyPresentation`; the body only places them.
#[component]
pub fn SystemKeyPickerKey(props: SystemKeyPickerKeyProps) -> Element {
    let SystemKeyPickerKeyPresentation {
        class,
        label,
        title,
        placement,
        anchor,
        wide,
        onclick,
    } = SystemKeyPickerKeyPresentation::from(&props);
    rsx! {
        button {
            class,
            r#type: "button",
            "data-tooltip": title,
            "data-tooltip-placement": placement,
            "data-tooltip-anchor": anchor,
            "data-wide": wide,
            onclick,
            {label}
        }
    }
}
