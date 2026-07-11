mod cell;
pub mod components;
mod key_event;
mod model;
mod presentation;
mod view;

pub use view::KeyPickerBoardView;
mod style;

use components::key_picker_column::KeyPickerColumn;
use dioxus::prelude::*;
use presentation::KeyPickerBoardPresentation;
use style::CLASS;
use tw_macro::assert_component;

pub use cell::{KeyCell, KeyCellState, KeyColumn, KeyWidth};
pub(crate) use key_event::BrowserKeyEvent;
use model::KeyPickerBoardModel;

/// An on-screen keyboard that assigns a key when one is clicked or typed. It is
/// dialog-agnostic and side-effect-free: a focusable board of key rows with one
/// `onkeydown` on its own element — no document listeners, no focus effect, no dialog,
/// no open flag. It behaves identically inline, in a dialog, or in the gallery. Every
/// key is a [`KeyCode`](warcraft_keybinds::KeyCode), so the board is a plain collection
/// of keys with no type parameter; the caller lays out the columns of [`KeyCell`]s and
/// decides which keys, labels, widths, and states they carry.
#[component]
pub fn KeyPickerBoard(props: KeyPickerBoardModel) -> Element {
    let presentation = KeyPickerBoardPresentation::from(&props);
    let columns = presentation.columns;
    let on_pick = presentation.on_pick;
    let onkeydown = presentation.onkeydown;
    rsx! {
        div {
            class: CLASS,
            role: "group",
            aria_label: "Available hotkeys",
            tabindex: "-1",
            onkeydown,
            for column in columns {
                {
                    let rows = column.into_rows();
                    rsx! {
                        KeyPickerColumn { rows, on_pick }
                    }
                }
            }
        }
    }
}

assert_component!(KeyPickerBoard);
