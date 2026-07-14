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
                        KeyPickerColumn {
                            rows,
                            on_pick,
                        }
                    }
                }
            }
        }
    }
}

assert_component!(KeyPickerBoard);
