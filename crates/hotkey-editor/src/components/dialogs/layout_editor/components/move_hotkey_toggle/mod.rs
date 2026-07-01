pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::move_hotkey_checkbox::MoveHotkeyCheckbox;
use style::CLASS;

pub use props::MoveHotkeyToggleProps;

assert_component!(MoveHotkeyToggle);

/// The labeled checkbox controlling whether moving an ability rewrites its hotkey
/// to match the new cell.
#[component]
pub fn MoveHotkeyToggle(props: MoveHotkeyToggleProps) -> Element {
    let checked = props.checked;
    let on_toggle = props.on_toggle;
    rsx! {
        label {
            class: CLASS,
            MoveHotkeyCheckbox { checked, on_toggle }
            "Update hotkeys when moving abilities"
        }
    }
}
