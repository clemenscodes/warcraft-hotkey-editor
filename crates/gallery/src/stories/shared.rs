use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::{KeyPicker, KeyPickerCell, KeyPickerCellState};
use warcraft_keybinds::HotkeyToken;

pub fn stories() -> Vec<Story> {
    vec![
        Story::new("Shared", "Key picker — closed", key_picker_closed),
        Story::new("Shared", "Key picker — open, all states", key_picker_open),
    ]
}

fn key_picker_closed() -> Element {
    let title = "Pick a hotkey".to_string();
    let rows: Vec<Vec<KeyPickerCell>> = Vec::new();
    let open = false;
    rsx! {
        KeyPicker {
            title,
            rows,
            open,
            on_pick: move |_| {},
            on_close: move |_| {},
        }
    }
}

fn key_picker_open() -> Element {
    let title = "Pick a hotkey".to_string();

    let q_token = HotkeyToken::from('Q');
    let q_state = KeyPickerCellState::Available;
    let q_cell = KeyPickerCell::new(q_token, q_state);

    let w_token = HotkeyToken::from('W');
    let w_state = KeyPickerCellState::Current;
    let w_cell = KeyPickerCell::new(w_token, w_state);

    let e_token = HotkeyToken::from('E');
    let conflict_name = "Some Other Ability".to_string();
    let e_state = KeyPickerCellState::Conflict {
        display_name: conflict_name,
    };
    let e_cell = KeyPickerCell::new(e_token, e_state);

    let first_row = vec![q_cell, w_cell, e_cell];
    let rows = vec![first_row];
    let open = true;

    rsx! {
        KeyPicker {
            title,
            rows,
            open,
            on_pick: move |_| {},
            on_close: move |_| {},
        }
    }
}
