pub mod components;
mod hooks;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::key_picker_row::KeyPickerRow;
use hooks::use_board_focus;
use style::CLASS;

pub use props::KeyPickerBoardProps;

assert_component!(KeyPickerBoard);

/// The on-screen keyboard the picker offers: a focusable group of key rows. Owns
/// the focus side effect; the keydown handler that turns a physical keypress into
/// a pick arrives shaped as a prop.
#[component]
pub fn KeyPickerBoard(props: KeyPickerBoardProps) -> Element {
    use_board_focus();
    let rows = props.rows;
    let onkeydown = props.onkeydown;
    rsx! {
        div {
            class: CLASS,
            role: "group",
            aria_label: "Available hotkeys",
            tabindex: "-1",
            onkeydown,
            for row in rows {
                KeyPickerRow { ..row }
            }
        }
    }
}
