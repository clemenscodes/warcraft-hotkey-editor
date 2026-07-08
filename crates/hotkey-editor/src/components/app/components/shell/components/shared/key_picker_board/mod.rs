mod cell;
pub mod components;
mod key_event;
mod logic;
mod props;
mod style;

use components::key_picker_column::KeyPickerColumn;
use dioxus::prelude::*;
use logic::KeyPickerBoardPresentation;
use style::CLASS;
use tw_macro::assert_component;

pub use cell::{KeyCell, KeyCellState, KeyColumn, KeyWidth};
pub(crate) use key_event::BrowserKeyEvent;
pub use props::KeyPickerBoardProps;

assert_component!(KeyPickerBoard);

/// An on-screen keyboard that assigns a key when one is clicked or typed. It is
/// dialog-agnostic and side-effect-free: a focusable board of key rows with one
/// `onkeydown` on its own element — no document listeners, no focus effect, no dialog,
/// no open flag. It behaves identically inline, in a dialog, or in the gallery. Every
/// key is a [`KeyCode`](warcraft_keybinds::KeyCode), so the board is a plain collection
/// of keys with no type parameter; the caller lays out the columns of [`KeyCell`]s and
/// decides which keys, labels, widths, and states they carry.
#[component]
pub fn KeyPickerBoard(props: KeyPickerBoardProps) -> Element {
    let presentation = KeyPickerBoardPresentation::from(&props);
    let columns = presentation.columns;
    let onkeydown = presentation.onkeydown;
    rsx! {
        div {
            class: CLASS,
            role: "group",
            aria_label: "Available hotkeys",
            tabindex: "-1",
            onkeydown,
            for column in columns {
                KeyPickerColumn { ..column }
            }
        }
    }
}
