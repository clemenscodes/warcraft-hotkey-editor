use super::browser_event::BrowserKeyEvent;
use super::components::system_key_picker_board::SystemKeyPickerBoardProps;
use super::components::system_key_picker_board::components::system_key_picker_column::SystemKeyPickerColumnProps;
use dioxus::prelude::*;

use super::data::{KEYBOARD_ROWS, NUMPAD_ROWS};
use super::logic::{ColumnInputs, KeyColumn};
use super::props::SystemKeyPickerDialogProps;
use super::state::BoardSection;

/// The system key picker's shaped view: the open signal that drives the shell and
/// the fully built board (both columns of keys plus the keydown handler).
pub(super) struct SystemKeyPickerModel {
    pub(super) open: Signal<bool>,
    pub(super) title: String,
    pub(super) board: SystemKeyPickerBoardProps,
}

/// The physical-keyboard capture: the keydown handler that maps a real keypress to a
/// pick (or a close on Escape), skipping any key the board does not offer.
pub(super) struct KeyCapture {
    pub(super) onkeydown: EventHandler<Event<KeyboardData>>,
}

/// Builds the keydown handler. It closes on Escape, translates the browser event to a
/// domain [`KeyCode`](warcraft_keybinds::KeyCode), and fires a pick only for a key the
/// board actually offers.
fn use_key_capture(props: &SystemKeyPickerDialogProps) -> KeyCapture {
    let on_pick = props.on_pick;
    let on_close = props.on_close;
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
    KeyCapture { onkeydown }
}

/// Composes the picker's state and behaviour: mirrors the open flag into a signal
/// the shell can close, fires `on_close` when it does, builds the keyboard handler
/// via [`use_key_capture`], and shapes both columns of keys through one
/// [`KeyColumn`] builder.
pub(super) fn use_system_key_picker(props: &SystemKeyPickerDialogProps) -> SystemKeyPickerModel {
    let title = props.title.clone();
    let on_close = props.on_close;
    let open = use_signal(|| props.open);
    use_effect(move || {
        if !open() {
            on_close.call(());
        }
    });
    let capture = use_key_capture(props);
    let keyboard_inputs = ColumnInputs {
        section: BoardSection::Keyboard,
        rows: KEYBOARD_ROWS,
        current_code: props.current_code,
        conflicts: &props.conflicts,
        on_pick: props.on_pick,
    };
    let numpad_inputs = ColumnInputs {
        section: BoardSection::Numpad,
        rows: NUMPAD_ROWS,
        current_code: props.current_code,
        conflicts: &props.conflicts,
        on_pick: props.on_pick,
    };
    let keyboard_column = KeyColumn::build(&keyboard_inputs);
    let numpad_column = KeyColumn::build(&numpad_inputs);
    let main_column = keyboard_column.into_props();
    let numpad_column_props = numpad_column.into_props();
    let columns: Vec<SystemKeyPickerColumnProps> = vec![main_column, numpad_column_props];
    let onkeydown = capture.onkeydown;
    let board = SystemKeyPickerBoardProps { columns, onkeydown };
    SystemKeyPickerModel { open, title, board }
}
