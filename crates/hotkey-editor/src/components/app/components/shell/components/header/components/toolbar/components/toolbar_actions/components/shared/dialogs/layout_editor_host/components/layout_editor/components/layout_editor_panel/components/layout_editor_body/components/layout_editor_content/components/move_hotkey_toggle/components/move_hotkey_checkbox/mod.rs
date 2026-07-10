mod props;
mod view;

pub use view::MoveHotkeyCheckboxView;
mod style;

use dioxus::prelude::*;
use props::MoveHotkeyCheckboxProps;
use style::CLASS;
use tw_macro::assert_component;

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

assert_component!(MoveHotkeyCheckbox);
