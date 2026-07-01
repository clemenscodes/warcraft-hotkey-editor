pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::move_hotkey_checkbox::MoveHotkeyCheckbox;
use dioxus::prelude::*;
pub use props::MoveHotkeyToggleProps;
use style::CLASS;
assert_component!(MoveHotkeyToggle);

/// The labeled checkbox controlling whether moving an ability rewrites its hotkey
/// to match the new cell.
#[component]
pub fn MoveHotkeyToggle(props: MoveHotkeyToggleProps) -> Element {
    let checked = props.checked;
    let on_toggle = props.on_toggle;
    rsx! {
        label { class: CLASS,
            MoveHotkeyCheckbox { checked, on_toggle }
            "Update hotkeys when moving abilities"
        }
    }
}
