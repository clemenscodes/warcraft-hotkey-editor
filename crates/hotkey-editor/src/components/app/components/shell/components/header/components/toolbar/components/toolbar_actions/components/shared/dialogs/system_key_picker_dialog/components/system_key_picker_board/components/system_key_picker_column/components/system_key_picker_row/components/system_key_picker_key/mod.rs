mod logic;
mod props;
mod state;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::tooltip::{
    Tooltip, TooltipProps,
};
use dioxus::prelude::*;
use logic::SystemKeyPickerKeyPresentation;
pub use props::SystemKeyPickerKeyProps;
pub use state::SystemKeyPickerKeyState;
use tw_macro::assert_component;
assert_component!(SystemKeyPickerKey);

/// A single key on the system keyboard board: an on-screen keyboard button that
/// assigns its `KeyCode` when clicked. Its look and behaviour arrive shaped through
/// `SystemKeyPickerKeyPresentation`; its conflict tooltip through the shared
/// `Tooltip` leaf. The body only places them.
#[component]
pub fn SystemKeyPickerKey(props: SystemKeyPickerKeyProps) -> Element {
    let SystemKeyPickerKeyPresentation {
        class,
        label,
        wide,
        onclick,
    } = SystemKeyPickerKeyPresentation::from(&props);
    let tooltip = TooltipProps::from(&props);
    rsx! {
        button {
            class,
            r#type: "button",
            "data-wide": wide,
            onclick,
            {label}
            Tooltip { ..tooltip }
        }
    }
}
