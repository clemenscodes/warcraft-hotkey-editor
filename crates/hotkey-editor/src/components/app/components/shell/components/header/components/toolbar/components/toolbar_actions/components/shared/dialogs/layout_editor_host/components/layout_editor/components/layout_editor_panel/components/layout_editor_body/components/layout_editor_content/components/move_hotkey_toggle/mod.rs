pub mod components;
mod data;
mod props;
mod view;

pub use view::MoveHotkeyToggleView;
mod style;

use components::move_hotkey_checkbox::MoveHotkeyCheckbox;
use dioxus::prelude::*;
use props::MoveHotkeyToggleProps;
use style::CLASS;
use tw_macro::assert_component;

/// The labeled checkbox controlling whether moving an ability rewrites its hotkey
/// to match the new cell.
#[component]
pub fn MoveHotkeyToggle(props: MoveHotkeyToggleProps) -> Element {
    let checked = props.checked;
    let on_toggle = props.on_toggle;
    let label = data::LABEL;
    rsx! {
        label {
            class: CLASS,
            MoveHotkeyCheckbox { checked, on_toggle }
            {label}
        }
    }
}

assert_component!(MoveHotkeyToggle);
