use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

use super::components::key_picker_board::KeyPickerBoardProps;
use super::components::key_picker_board::components::key_picker_row::KeyPickerRowProps;
use super::components::key_picker_board::components::key_picker_row::components::key_picker_key::KeyPickerKeyProps;
use super::props::{KeyPickerCellState, KeyPickerProps};

/// The key picker's shaped view: the open signal that drives the shell and the
/// fully built board (its rows of keys and the keydown handler). The body only
/// places these; every derivation the body may not do happens here.
pub(super) struct KeyPickerModel {
    pub(super) open: Signal<bool>,
    pub(super) board: KeyPickerBoardProps,
}

/// Composes the picker's state and behaviour: mirrors the open flag into a signal
/// the shell can close, fires `on_close` when it does, builds the keyboard handler
/// that maps a pressed letter to a pick, and shapes the raw cells into the board's
/// per-key props.
pub(super) fn use_key_picker(props: &KeyPickerProps) -> KeyPickerModel {
    let rows = props.rows.clone();
    let allow_conflict_pick = props.allow_conflict_pick;
    let on_pick = props.on_pick;
    let on_close = props.on_close;
    let open = use_signal(|| props.open);

    use_effect(move || {
        if !open() {
            on_close.call(());
        }
    });

    let rows_for_keydown = rows.clone();
    let onkeydown = EventHandler::new(move |event: Event<KeyboardData>| {
        event.stop_propagation();
        let key_value = event.data().key().to_string();
        if key_value == "Escape" {
            event.prevent_default();
            on_close.call(());
            return;
        }
        // Only single ASCII letters map to a board key. These are exactly the keys
        // the picker already offers via click; digits, the mouse-only tokens, and
        // any other physical key are ignored so the keyboard can never select a
        // hotkey the game cannot bind.
        let mut key_characters = key_value.chars();
        let Some(first_character) = key_characters.next() else {
            return;
        };
        let is_single_character = key_characters.next().is_none();
        if !is_single_character || !first_character.is_ascii_alphabetic() {
            return;
        }
        let Ok(pressed_token) = HotkeyToken::try_from(first_character) else {
            return;
        };
        let matching_cell = rows_for_keydown
            .iter()
            .flatten()
            .find(|cell| cell.token() == pressed_token);
        let Some(cell) = matching_cell else {
            return;
        };
        // Apply the same selectability rule as a click: a conflict cell is only
        // pickable when conflict swaps are allowed (the grid layout editor).
        let is_conflict = matches!(cell.state(), KeyPickerCellState::Conflict { .. });
        let is_pickable = !is_conflict || allow_conflict_pick;
        if !is_pickable {
            return;
        }
        event.prevent_default();
        on_pick.call(pressed_token);
    });

    let mut board_rows: Vec<KeyPickerRowProps> = Vec::new();
    for row_cells in rows {
        let mut keys: Vec<KeyPickerKeyProps> = Vec::new();
        for cell in row_cells {
            let key = KeyPickerKeyProps {
                cell,
                allow_conflict_pick,
                on_pick,
            };
            keys.push(key);
        }
        let row = KeyPickerRowProps { keys };
        board_rows.push(row);
    }

    let board = KeyPickerBoardProps {
        rows: board_rows,
        onkeydown,
    };
    KeyPickerModel { open, board }
}
