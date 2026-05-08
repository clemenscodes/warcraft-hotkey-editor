mod key_picker_key;

use key_picker_key::KeyPickerKey;

use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::components::dialogs::dialog_header::DialogHeader;
use warcraft_keybinds::HotkeyToken;

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

#[derive(Props, Clone, PartialEq)]
pub(crate) struct KeyPickerProps {
    pub(crate) title: String,
    pub(crate) rows: Vec<Vec<KeyPickerCell>>,
    pub(crate) open: bool,
    // When true, conflict cells stay clickable and forward `on_pick` — used
    // by the grid layout editor where clicking a conflict swaps the two
    // cells. The spell hotkey picker leaves this off so a binding collision
    // is visually flagged but cannot be selected.
    #[props(default = false)]
    pub(crate) allow_conflict_pick: bool,
    pub(crate) on_pick: EventHandler<HotkeyToken>,
    pub(crate) on_close: EventHandler<()>,
}

#[component]
pub(crate) fn KeyPicker(props: KeyPickerProps) -> Element {
    let title = props.title;
    let rows = props.rows;
    let open = props.open;
    let allow_conflict_pick = props.allow_conflict_pick;
    let on_pick = props.on_pick;
    let on_close = props.on_close;
    let dialog_title = title.clone();
    let handle_open_change = move |next_open: bool| {
        if !next_open {
            on_close.call(());
        }
    };
    let handle_keydown = move |event: Event<KeyboardData>| {
        event.stop_propagation();
        let key_value = event.data().key().to_string();
        if key_value == "Escape" {
            event.prevent_default();
            on_close.call(());
        }
    };
    let handle_close = move |_| on_close.call(());
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open,
            on_open_change: handle_open_change,
            DialogContent { class: "dialog-shell wc3-dialog key-picker-shell".to_string(),
                div {
                    class: "dialog-key-scope",
                    onkeydown: handle_keydown,
                    DialogHeader {
                        title: dialog_title.clone(),
                        on_close: handle_close,
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
