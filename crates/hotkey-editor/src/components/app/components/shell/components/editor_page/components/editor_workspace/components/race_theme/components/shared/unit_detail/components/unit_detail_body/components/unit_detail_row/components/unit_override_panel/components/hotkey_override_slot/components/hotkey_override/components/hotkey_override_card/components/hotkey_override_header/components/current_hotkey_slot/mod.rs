pub mod components;
mod model;
mod presentation;
mod view;

pub use view::CurrentHotkeySlotView;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::hotkey_override_slot::components::hotkey_override::components::hotkey_override_card::components::shared::override_key::OverrideKey;
use components::unbindable_note::UnbindableNote;
use presentation::{HotkeySlotKeyCell, CurrentHotkeySlotDispatch};
use tw_macro::assert_component;

use model::CurrentHotkeySlotModel;

/// The hotkey / research-hotkey / passive-note slot in the override header. Renders
/// exactly the one that applies, or nothing.
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
            OverrideKey { label, is_editing, is_special, title, on_activate }
        };
    }
    if let Some(text) = dispatch.info_text {
        return rsx! {
            UnbindableNote { text }
        };
    }
    rsx! {}
}

assert_component!(CurrentHotkeySlot);
