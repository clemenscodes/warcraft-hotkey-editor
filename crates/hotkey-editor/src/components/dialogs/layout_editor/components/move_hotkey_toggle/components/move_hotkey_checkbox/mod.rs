mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::MoveHotkeyCheckboxProps;

assert_component!(MoveHotkeyCheckbox);

/// The custom-styled checkbox inside the move-hotkey toggle: a gold check on a
/// dark, gold-bordered tile matching the grid cells.
#[component]
pub fn MoveHotkeyCheckbox(props: MoveHotkeyCheckboxProps) -> Element {
    let checked = props.checked;
    let onchange = props.on_toggle;
    rsx! {
        input {
            class: CLASS,
            r#type: "checkbox",
            "aria-label": "Update hotkeys when moving abilities",
            checked,
            onchange,
        }
    }
}
