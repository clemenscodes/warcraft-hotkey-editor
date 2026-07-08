use super::logic::LetterColumnInputs;
use super::props::KeyPickerProps;
use crate::components::app::components::shell::components::shared::key_picker_board::{
    KeyColumn, KeyPickerBoardProps,
};
use dioxus::prelude::*;
use warcraft_keybinds::{HotkeyToken, KeyCode};

/// The key picker's shaped view: the open signal that drives the dialog shell and the
/// fully built board (its single column of letter keys plus the pick and dismiss
/// handlers). The body only places these.
pub(super) struct KeyPickerModel {
    pub(super) open: Signal<bool>,
    pub(super) title: String,
    pub(super) board: KeyPickerBoardProps,
}

/// Composes the picker: mirrors the caller's open flag into a signal the dialog shell
/// can close (firing the caller's `on_close` when it does), shapes the caller's letter
/// cells into the shared board's single column, and adapts the board's `KeyCode` pick
/// back to the [`HotkeyToken`] the caller works in. Focus and the focus-gap keyboard
/// fallback belong to the board host, so nothing here listens or focuses.
pub(super) fn use_key_picker(props: &KeyPickerProps) -> KeyPickerModel {
    let title = props.title.clone();
    let parent_on_close = props.on_close;
    let letter_on_pick = props.on_pick;
    let mut open = use_signal(|| props.open);
    use_effect(move || {
        if !open() {
            parent_on_close.call(());
        }
    });
    let board_on_close = EventHandler::new(move |_event: ()| open.set(false));
    let column_inputs = LetterColumnInputs {
        rows: props.rows.clone(),
        allow_conflict_pick: props.allow_conflict_pick,
    };
    let column = KeyColumn::from(column_inputs);
    let columns: Vec<KeyColumn> = vec![column];
    let board_on_pick = EventHandler::new(move |code: KeyCode| {
        if let Ok(token) = HotkeyToken::try_from(code) {
            letter_on_pick.call(token);
        }
    });
    let board = KeyPickerBoardProps {
        columns,
        on_pick: board_on_pick,
        on_close: board_on_close,
    };
    KeyPickerModel { open, title, board }
}
