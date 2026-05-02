use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::components::dialog_header::DialogHeader;

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum KeyPickerCellState {
    Available,
    Current,
    Conflict { display_name: String },
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct KeyPickerCell {
    pub(crate) letter: char,
    pub(crate) state: KeyPickerCellState,
}

impl KeyPickerCell {
    pub(crate) fn new(letter: char, state: KeyPickerCellState) -> Self {
        Self { letter, state }
    }
}

#[component]
pub(crate) fn KeyPicker(
    title: String,
    rows: Vec<Vec<KeyPickerCell>>,
    open: bool,
    on_pick: EventHandler<char>,
    on_close: EventHandler<()>,
) -> Element {
    let dialog_title = title.clone();
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open,
            on_open_change: move |next_open: bool| {
                if !next_open {
                    on_close.call(());
                }
            },
            DialogContent { class: "dialog-shell wc3-dialog key-picker-shell".to_string(),
                div {
                    class: "dialog-key-scope",
                    onkeydown: move |event| {
                        event.stop_propagation();
                        let key_val = event.data().key().to_string();
                        if key_val == "Escape" {
                            event.prevent_default();
                            on_close.call(());
                            return;
                        }
                        if key_val.len() != 1 {
                            return;
                        }
                        let Some(ch) = key_val.chars().next() else {
                            return;
                        };
                        if !ch.is_ascii_alphabetic() {
                            return;
                        }
                        event.prevent_default();
                        on_pick.call(ch.to_ascii_uppercase());
                    },
                    DialogHeader {
                        title: dialog_title.clone(),
                        on_close: move |_| on_close.call(()),
                    }
                    div {
                        class: "wc3-dialog-body key-picker-body",
                        tabindex: "-1",
                        autofocus: true,
                        div { class: "key-picker-board", role: "group", aria_label: "Available hotkeys",
                            for (row_index, row_cells) in rows.iter().enumerate() {
                                div {
                                    key: "{row_index}",
                                    class: "key-picker-row",
                                    for cell in row_cells.iter() {
                                        KeyPickerKey { cell: cell.clone(), on_pick }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn KeyPickerKey(cell: KeyPickerCell, on_pick: EventHandler<char>) -> Element {
    let letter_text = cell.letter.to_string();
    let (state_class, conflict_title) = match &cell.state {
        KeyPickerCellState::Available => ("available", None),
        KeyPickerCellState::Current => ("current", None),
        KeyPickerCellState::Conflict { display_name } => {
            ("conflict", Some(format!("Already used by {display_name}")))
        }
    };
    let is_conflict = matches!(cell.state, KeyPickerCellState::Conflict { .. });
    let class_name = format!("key-picker-key {state_class}");
    let title_attr = conflict_title.unwrap_or_default();
    let letter_for_click = cell.letter;
    rsx! {
        button {
            class: "{class_name}",
            r#type: "button",
            disabled: is_conflict,
            title: "{title_attr}",
            "data-letter": "{letter_text}",
            onclick: move |_| {
                if !is_conflict {
                    on_pick.call(letter_for_click);
                }
            },
            "{letter_text}"
        }
    }
}
