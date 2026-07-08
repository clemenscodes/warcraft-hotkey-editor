mod logic;
mod props;
mod state;
mod style;

use crate::components::app::components::shell::components::shared::tooltip::{
    Tooltip, TooltipProps,
};
use dioxus::prelude::*;
use logic::KeyPickerKeyPresentation;
pub use props::KeyPickerKeyProps;
use tw_macro::assert_component;
assert_component!(KeyPickerKey);

/// A single key on the picker board: an on-screen keyboard button that assigns its
/// key when clicked. Its look and behaviour arrive shaped through
/// `KeyPickerKeyPresentation`; its conflict tooltip through the shared `Tooltip`
/// leaf. The body only places them.
#[component]
pub fn KeyPickerKey(props: KeyPickerKeyProps) -> Element {
    let KeyPickerKeyPresentation {
        class,
        label,
        data_label,
        data_wide,
        disabled,
        onclick,
    } = KeyPickerKeyPresentation::from(&props);
    let tooltip = TooltipProps::from(&props);
    rsx! {
        button {
            class,
            r#type: "button",
            disabled,
            "data-wide": data_wide,
            "data-label": data_label,
            onclick,
            {label}
            Tooltip { ..tooltip }
        }
    }
}
