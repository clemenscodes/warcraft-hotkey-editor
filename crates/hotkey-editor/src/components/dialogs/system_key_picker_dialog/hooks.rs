use dioxus::prelude::*;
use super::browser_event::BrowserKeyEvent;
use super::components::system_key_picker_board::SystemKeyPickerBoardProps;
use super::components::system_key_picker_board::components::system_key_picker_column::SystemKeyPickerColumnProps;
use super::components::system_key_picker_board::components::system_key_picker_column::components::system_key_picker_row::SystemKeyPickerRowProps;

use super::components::system_key_picker_board::components::system_key_picker_column::components::system_key_picker_row::components::system_key_picker_key::{
    SystemKeyPickerKeyProps, SystemKeyPickerKeyState,
};

use super::data::{KEYBOARD_ROWS, NUMPAD_ROWS};
use super::props::SystemKeyPickerDialogProps;

/// The system key picker's shaped view: the open signal that drives the shell and
/// the fully built board (both columns of keys plus the keydown handler).
pub(super) struct SystemKeyPickerModel {
    pub(super) open: Signal<bool>,
    pub(super) board: SystemKeyPickerBoardProps,
}

/// Composes the picker's state and behaviour: mirrors the open flag into a signal
/// the shell can close, fires `on_close` when it does, builds the keyboard handler
/// that maps a physical keypress to a pick, and shapes both boards' keys with their
/// state, tooltip, anchor, and wide flags.
pub(super) fn use_system_key_picker(props: &SystemKeyPickerDialogProps) -> SystemKeyPickerModel {
    let current_code = props.current_code;
    let conflicts = props.conflicts.clone();
    let on_pick = props.on_pick;
    let on_close = props.on_close;
    let open = use_signal(|| props.open);
    use_effect(move || {
        if !open() {
            on_close.call(());
        }
    });
    let onkeydown = EventHandler::new(move |event: Event<KeyboardData>| {
        event.stop_propagation();
        let key_value = event.data().key().to_string();
        if key_value == "Escape" {
            event.prevent_default();
            on_close.call(());
            return;
        }
        let code_value = event.data().code().to_string();
        let browser_event = BrowserKeyEvent::new(&key_value, &code_value);
        let Some(code) = browser_event.key_code() else {
            return;
        };
        let is_offered = KEYBOARD_ROWS
            .iter()
            .chain(NUMPAD_ROWS.iter())
            .flat_map(|row| row.iter())
            .any(|entry| entry.code == code);
        if !is_offered {
            return;
        }
        event.prevent_default();
        on_pick.call(code);
    });
    let mut columns: Vec<SystemKeyPickerColumnProps> = Vec::new();
    let mut main_rows: Vec<SystemKeyPickerRowProps> = Vec::new();
    let keyboard_total = KEYBOARD_ROWS.len();
    for (row_index, row) in KEYBOARD_ROWS.iter().enumerate() {
        let is_bottom_row = row_index + 2 >= keyboard_total;
        let placement = if is_bottom_row { "above" } else { "below" };
        let last_index = row.len().saturating_sub(1);
        let mut keys: Vec<SystemKeyPickerKeyProps> = Vec::new();
        for (key_index, entry) in row.iter().enumerate() {
            let code = entry.code;
            let label = entry.label;
            let conflict_names = conflicts.get(&code);
            let state = if code == current_code {
                SystemKeyPickerKeyState::Current
            } else if conflict_names.is_some() {
                SystemKeyPickerKeyState::Conflict
            } else {
                SystemKeyPickerKeyState::Normal
            };
            let title = conflict_names
                .map(|names| format!("Already used by {}", names.join(", ")))
                .unwrap_or_default();
            let anchor = if key_index == 0 {
                "left"
            } else if key_index == last_index {
                "right"
            } else {
                ""
            };
            let is_wide = matches!(label, "Space" | "Mouse4" | "Mouse5" | "Backspace");
            let wide = if is_wide { "true" } else { "" };
            let key = SystemKeyPickerKeyProps {
                label,
                code,
                state,
                title,
                placement,
                anchor,
                wide,
                on_pick,
            };
            keys.push(key);
        }
        let row_props = SystemKeyPickerRowProps { keys };
        main_rows.push(row_props);
    }
    let main_column = SystemKeyPickerColumnProps { rows: main_rows };
    columns.push(main_column);
    let mut numpad_rows: Vec<SystemKeyPickerRowProps> = Vec::new();
    let numpad_total = NUMPAD_ROWS.len();
    for (row_index, row) in NUMPAD_ROWS.iter().enumerate() {
        let is_bottom_row = row_index + 2 >= numpad_total;
        let placement = if is_bottom_row { "above" } else { "below" };
        let mut keys: Vec<SystemKeyPickerKeyProps> = Vec::new();
        for entry in row.iter() {
            let code = entry.code;
            let label = entry.label;
            let conflict_names = conflicts.get(&code);
            let state = if code == current_code {
                SystemKeyPickerKeyState::Current
            } else if conflict_names.is_some() {
                SystemKeyPickerKeyState::Conflict
            } else {
                SystemKeyPickerKeyState::Normal
            };
            let title = conflict_names
                .map(|names| format!("Already used by {}", names.join(", ")))
                .unwrap_or_default();
            let anchor = "right";
            let wide = "";
            let key = SystemKeyPickerKeyProps {
                label,
                code,
                state,
                title,
                placement,
                anchor,
                wide,
                on_pick,
            };
            keys.push(key);
        }
        let row_props = SystemKeyPickerRowProps { keys };
        numpad_rows.push(row_props);
    }
    let numpad_column = SystemKeyPickerColumnProps { rows: numpad_rows };
    columns.push(numpad_column);
    let board = SystemKeyPickerBoardProps { columns, onkeydown };
    SystemKeyPickerModel { open, board }
}
