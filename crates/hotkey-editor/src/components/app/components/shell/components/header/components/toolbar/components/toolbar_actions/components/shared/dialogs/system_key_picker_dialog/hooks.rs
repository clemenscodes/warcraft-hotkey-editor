use super::data::{KEYBOARD_ROWS, NUMPAD_ROWS};
use super::logic::ColumnInputs;
use super::props::SystemKeyPickerDialogProps;
use super::state::BoardSection;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// The system key picker's shaped view: the open signal that drives the dialog shell,
/// its title, and the raw board values (both laid-out columns plus the pick and Escape
/// handlers) the body hands to the shared board host.
pub(super) struct SystemKeyPickerModel {
    pub(super) open: Signal<bool>,
    pub(super) title: String,
    pub(super) columns: Vec<KeyColumn>,
    pub(super) on_pick: EventHandler<KeyCode>,
    pub(super) board_on_close: EventHandler<()>,
}

/// Composes the picker: mirrors the caller's open flag into a signal the dialog shell
/// can close (firing the caller's `on_close` when it does), lays out the keyboard and
/// numpad columns through one [`SystemKeyColumn`] builder, and wires the board's pick
/// and Escape handlers. The board itself owns no dialog state.
pub(super) fn use_system_key_picker(props: &SystemKeyPickerDialogProps) -> SystemKeyPickerModel {
    let title = props.title.clone();
    let parent_on_close = props.on_close;
    let on_pick = props.on_pick;
    let mut open = use_signal(|| props.open);
    use_effect(move || {
        if !open() {
            parent_on_close.call(());
        }
    });
    let board_on_close = EventHandler::new(move |_event: ()| open.set(false));
    let keyboard_inputs = ColumnInputs {
        section: BoardSection::Keyboard,
        rows: KEYBOARD_ROWS,
        current_code: props.current_code,
        conflicts: &props.conflicts,
    };
    let numpad_inputs = ColumnInputs {
        section: BoardSection::Numpad,
        rows: NUMPAD_ROWS,
        current_code: props.current_code,
        conflicts: &props.conflicts,
    };
    let keyboard = KeyColumn::from(&keyboard_inputs);
    let numpad = KeyColumn::from(&numpad_inputs);
    let columns: Vec<KeyColumn> = vec![keyboard, numpad];
    SystemKeyPickerModel {
        open,
        title,
        columns,
        on_pick,
        board_on_close,
    }
}
