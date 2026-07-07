use super::components::key_picker_board::KeyPickerBoardProps;
use super::props::{
    KeyCapture, KeyCaptureInputs, KeyPickerBoardInputs, KeyPickerCellState, KeyPickerProps,
};
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// The key picker's shaped view: the open signal that drives the shell and the
/// fully built board (its rows of keys and the keydown handler). The body only
/// places these; every derivation the body may not do happens here.
pub(super) struct KeyPickerModel {
    pub(super) open: Signal<bool>,
    pub(super) title: String,
    pub(super) board: KeyPickerBoardProps,
}

/// Captures the keyboard: builds the board's `onkeydown` handler and the resolver
/// that maps a pressed letter to a pick. Both the board's own `onkeydown` (when it
/// holds focus) and its document-level fallback listener (for the focus gap right
/// after the portal opens) write a pressed letter into `pending_key`; the effect
/// here resolves it to a pick. That keeps one resolver whichever path the keypress
/// took, and closes the gap where a key pressed before the board gained focus was
/// silently dropped. Escape stays inline: it closes only this picker and must stop
/// propagating so the outer editor dialog does not also close.
fn use_key_capture(inputs: KeyCaptureInputs) -> KeyCapture {
    let KeyCaptureInputs {
        rows,
        allow_conflict_pick,
        on_pick,
        on_close,
    } = inputs;
    let mut pending_key = use_signal(|| Option::<String>::None);
    let onkeydown = EventHandler::new(move |event: Event<KeyboardData>| {
        event.stop_propagation();
        let key_value = event.data().key().to_string();
        if key_value == "Escape" {
            event.prevent_default();
            on_close.call(());
            return;
        }
        let mut key_characters = key_value.chars();
        let is_single_letter = key_characters
            .next()
            .map(|character| character.is_ascii_alphabetic())
            .unwrap_or(false)
            && key_characters.next().is_none();
        if is_single_letter {
            event.prevent_default();
            pending_key.set(Some(key_value));
        }
    });
    let rows_for_pick = rows;
    use_effect(move || {
        let Some(key_value) = pending_key() else {
            return;
        };
        pending_key.set(None);
        let Some(first_character) = key_value.chars().next() else {
            return;
        };
        let Ok(pressed_token) = HotkeyToken::try_from(first_character) else {
            return;
        };
        let matching_cell = rows_for_pick
            .iter()
            .flatten()
            .find(|cell| cell.token() == pressed_token);
        let Some(cell) = matching_cell else {
            return;
        };
        let is_conflict = matches!(cell.state(), KeyPickerCellState::Conflict { .. });
        let is_pickable = !is_conflict || allow_conflict_pick;
        if !is_pickable {
            return;
        }
        on_pick.call(pressed_token);
    });
    KeyCapture {
        onkeydown,
        pending_key,
    }
}

/// Composes the picker's state and behaviour: mirrors the open flag into a signal
/// the shell can close, fires `on_close` when it does, captures the keyboard into a
/// keydown handler and resolver, and shapes the raw cells into the board's per-key
/// props.
pub(super) fn use_key_picker(props: &KeyPickerProps) -> KeyPickerModel {
    let title = props.title.clone();
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
    let capture_rows = rows.clone();
    let capture_inputs = KeyCaptureInputs {
        rows: capture_rows,
        allow_conflict_pick,
        on_pick,
        on_close,
    };
    let capture = use_key_capture(capture_inputs);
    let board_inputs = KeyPickerBoardInputs {
        rows,
        allow_conflict_pick,
        on_pick,
        onkeydown: capture.onkeydown,
        pending_key: capture.pending_key,
    };
    let board = KeyPickerBoardProps::from(board_inputs);
    KeyPickerModel { open, title, board }
}
