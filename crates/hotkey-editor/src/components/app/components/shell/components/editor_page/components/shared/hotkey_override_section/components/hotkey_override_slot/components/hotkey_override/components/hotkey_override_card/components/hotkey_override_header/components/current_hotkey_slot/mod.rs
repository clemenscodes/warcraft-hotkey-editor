pub mod components;
mod model;
mod presentation;
mod view;

pub use view::CurrentHotkeySlotView;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::shared::hotkey_override_section::components::hotkey_override_slot::components::hotkey_override::components::hotkey_override_card::components::shared::override_key::OverrideKey;
use components::unbindable_note::UnbindableNote;
use presentation::{HotkeySlotKeyCell, CurrentHotkeySlotDispatch};
use tw_macro::assert_component;

use model::CurrentHotkeySlotModel;

#[component]
pub fn CurrentHotkeySlot(props: CurrentHotkeySlotModel) -> Element {
    let dispatch = CurrentHotkeySlotDispatch::from(&props);
    if let Some(key_cell) = dispatch.key_cell {
        let HotkeySlotKeyCell {
            label,
            is_editing,
            is_special,
            title,
            on_activate,
        } = key_cell;
        return rsx! {
            OverrideKey {
                label,
                is_editing,
                is_special,
                title,
                on_activate,
            }
        };
    }
    if let Some(text) = dispatch.info_text {
        return rsx! {
            UnbindableNote {
                text,
            }
        };
    }
    rsx! {}
}

assert_component!(CurrentHotkeySlot);
