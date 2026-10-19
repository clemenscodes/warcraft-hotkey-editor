pub mod components;
mod data;
mod model;
mod view;

pub use view::HotkeyOverrideSlotView;

use components::hotkey_override::HotkeyOverride;
use components::hotkey_override_empty::HotkeyOverrideEmpty;
use dioxus::prelude::*;
use model::HotkeyOverrideSlotModel;
use tw_macro::assert_component;

#[component]
pub fn HotkeyOverrideSlot(props: HotkeyOverrideSlotModel) -> Element {
    let HotkeyOverrideSlotModel {
        detail,
        active_container_slots,
    } = props;
    let Some(detail) = detail else {
        let message = data::EMPTY_PROMPT.to_string();
        return rsx! {
            HotkeyOverrideEmpty {
                message,
            }
        };
    };
    rsx! {
        HotkeyOverride {
            detail,
            active_container_slots,
        }
    }
}

assert_component!(HotkeyOverrideSlot);
