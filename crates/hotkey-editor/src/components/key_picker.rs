use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::components::dialog_header::DialogHeader;
use crate::domain::hotkey_token::HotkeyToken;

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum KeyPickerCellState {
    Available,
    Current,
    Conflict { display_name: String },
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct KeyPickerCell {
    token: HotkeyToken,
    state: KeyPickerCellState,
}

impl KeyPickerCell {
    pub(crate) fn new(token: HotkeyToken, state: KeyPickerCellState) -> Self {
        Self { token, state }
    }

    pub(crate) fn token(&self) -> HotkeyToken {
        self.token
    }

    pub(crate) fn state(&self) -> &KeyPickerCellState {
        &self.state
    }
}

#[component]
pub(crate) fn KeyPicker(
    title: String,
    rows: Vec<Vec<KeyPickerCell>>,
    open: bool,
    // When true, conflict cells stay clickable and forward `on_pick` — used
    // by the grid layout editor where clicking a conflict swaps the two
    // cells. The spell hotkey picker leaves this off so a binding collision
    // is visually flagged but cannot be selected.
    #[props(default = false)] allow_conflict_pick: bool,
    on_pick: EventHandler<HotkeyToken>,
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
                        let key_value = event.data().key().to_string();
                        if key_value == "Escape" {
                            event.prevent_default();
                            on_close.call(());
                        }
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
                                        KeyPickerKey {
                                            cell: cell.clone(),
                                            allow_conflict_pick,
                                            on_pick,
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
}

#[component]
fn KeyPickerKey(
    cell: KeyPickerCell,
    allow_conflict_pick: bool,
    on_pick: EventHandler<HotkeyToken>,
) -> Element {
    let token = cell.token();
    let label_text = token.display_label();
    let (state_class, conflict_title) = match cell.state() {
        KeyPickerCellState::Available => ("available", None),
        KeyPickerCellState::Current => ("current", None),
        KeyPickerCellState::Conflict { display_name } => {
            let prefix = if allow_conflict_pick {
                "Pick to swap with"
            } else {
                "Already used by"
            };
            ("conflict", Some(format!("{prefix} {display_name}")))
        }
    };
    let is_conflict = matches!(cell.state(), KeyPickerCellState::Conflict { .. });
    let is_disabled = is_conflict && !allow_conflict_pick;
    let is_special = char::try_from(token).is_err();
    let class_name = format!("key-picker-key {state_class}");
    let title_attribute = conflict_title.unwrap_or_default();
    let special_flag = if is_special { "true" } else { "false" };
    let token_for_click = token;
    rsx! {
        button {
            class: "{class_name}",
            r#type: "button",
            disabled: is_disabled,
            title: "{title_attribute}",
            "data-label": "{label_text}",
            "data-special": "{special_flag}",
            onclick: move |_| {
                if !is_disabled {
                    on_pick.call(token_for_click);
                }
            },
            "{label_text}"
        }
    }
}
