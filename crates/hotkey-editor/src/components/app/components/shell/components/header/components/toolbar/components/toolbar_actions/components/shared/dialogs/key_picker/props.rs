use super::components::key_picker_board::KeyPickerBoardProps;
use super::components::key_picker_board::components::key_picker_row::KeyPickerRowProps;
use super::components::key_picker_board::components::key_picker_row::components::key_picker_key::KeyPickerKeyProps;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// The look a board key wears, decided by the caller from the current bindings:
/// a free key, the key currently bound here, or a key already taken by another
/// binding (which names the holder so the key can explain the clash).
#[derive(Clone, PartialEq, Eq)]
pub enum KeyPickerCellState {
    Available,
    Current,
    Conflict { display_name: String },
}

/// One key on the picker board: the hotkey it offers and the state that hotkey is
/// in. Built by the caller from the domain; the picker only renders it.
#[derive(Clone, PartialEq, Eq)]
pub struct KeyPickerCell {
    token: HotkeyToken,
    state: KeyPickerCellState,
}

impl KeyPickerCell {
    pub fn new(token: HotkeyToken, state: KeyPickerCellState) -> Self {
        Self { token, state }
    }

    pub fn token(&self) -> HotkeyToken {
        self.token
    }

    pub fn state(&self) -> &KeyPickerCellState {
        &self.state
    }
}

/// What the key picker needs: the title the shell shows, the board of keys to
/// offer, the open flag that mounts it, and the handlers for a pick and a close.
/// `allow_conflict_pick` lets a conflicting key stay pickable (the layout editor
/// swaps the two), which the spell picker leaves off so a clash is flagged but
/// cannot be chosen.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerProps {
    #[props(into)]
    pub title: String,
    pub rows: Vec<Vec<KeyPickerCell>>,
    pub open: bool,
    #[props(default = false)]
    pub allow_conflict_pick: bool,
    pub on_pick: EventHandler<HotkeyToken>,
    pub on_close: EventHandler<()>,
}

/// What the keyboard-capture concern needs: the raw cells to resolve a pressed
/// letter against, the conflict gate, and the pick/close handlers a keystroke can
/// fire. The sub-hook turns these into a [`KeyCapture`].
pub(super) struct KeyCaptureInputs {
    pub(super) rows: Vec<Vec<KeyPickerCell>>,
    pub(super) allow_conflict_pick: bool,
    pub(super) on_pick: EventHandler<HotkeyToken>,
    pub(super) on_close: EventHandler<()>,
}

/// The keyboard-capture concern's output: the board's own keydown handler and the
/// `pending_key` signal both that handler and the document-level fallback listener
/// write into. Escape is handled inline inside the handler and never reaches here.
pub(super) struct KeyCapture {
    pub(super) onkeydown: EventHandler<Event<KeyboardData>>,
    pub(super) pending_key: Signal<Option<String>>,
}

/// Everything the board's props are built from: the raw cells, the conflict gate and
/// pick handler each key wears, plus the keydown handler and `pending_key` from the
/// keyboard-capture concern. The board props derive themselves through the `From`
/// impl below, so the hook never builds the rows by hand.
pub(super) struct KeyPickerBoardInputs {
    pub(super) rows: Vec<Vec<KeyPickerCell>>,
    pub(super) allow_conflict_pick: bool,
    pub(super) on_pick: EventHandler<HotkeyToken>,
    pub(super) onkeydown: EventHandler<Event<KeyboardData>>,
    pub(super) pending_key: Signal<Option<String>>,
}

impl From<KeyPickerBoardInputs> for KeyPickerBoardProps {
    fn from(inputs: KeyPickerBoardInputs) -> Self {
        let KeyPickerBoardInputs {
            rows,
            allow_conflict_pick,
            on_pick,
            onkeydown,
            pending_key,
        } = inputs;
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
        Self {
            rows: board_rows,
            onkeydown,
            pending_key,
        }
    }
}
