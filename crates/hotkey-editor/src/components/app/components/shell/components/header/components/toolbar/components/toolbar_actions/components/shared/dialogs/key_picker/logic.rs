use super::components::key_picker_body::KeyPickerBodyProps;
use super::hooks::KeyPickerModel;
use super::props::{KeyPickerCell, KeyPickerCellState};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use crate::components::app::components::shell::components::shared::key_picker_board::{
    KeyCell, KeyCellState, KeyColumn, KeyWidth,
};
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// The letter cells a caller handed the picker, plus the conflict gate, shaped into one
/// board column. The letter keyboard is a single column, so the picker builds exactly
/// one; each cell becomes a [`KeyCell`] carrying its `KeyCode`, cap label, wide flag,
/// visual state, whether it may be picked, and (for a conflict) the composed hover text
/// with its placement and anchor.
pub(super) struct LetterColumnInputs {
    pub(super) rows: Vec<Vec<KeyPickerCell>>,
    pub(super) allow_conflict_pick: bool,
}

impl From<LetterColumnInputs> for KeyColumn {
    fn from(inputs: LetterColumnInputs) -> Self {
        let mut column_rows: Vec<Vec<KeyCell>> = Vec::new();
        for row in &inputs.rows {
            let placement = TooltipPlacement::Above;
            let last_index = row.len().saturating_sub(1);
            let mut cells: Vec<KeyCell> = Vec::new();
            for (key_index, cell) in row.iter().enumerate() {
                let token = cell.token();
                let key_code = KeyCode::from(token);
                let label = token.to_string();
                let single_character = char::try_from(token);
                let width = match single_character {
                    Ok(_) => KeyWidth::Standard,
                    Err(_) => KeyWidth::Wide,
                };
                let anchor = if key_index == 0 {
                    TooltipAnchor::Left
                } else if key_index == last_index {
                    TooltipAnchor::Right
                } else {
                    TooltipAnchor::Center
                };
                let cell_state = cell.state();
                let is_conflict = matches!(cell_state, KeyPickerCellState::Conflict { .. });
                let state = match cell_state {
                    KeyPickerCellState::Available => KeyCellState::Available,
                    KeyPickerCellState::Current => KeyCellState::Current,
                    KeyPickerCellState::Conflict { display_name } => {
                        let prefix = if inputs.allow_conflict_pick {
                            "Pick to swap with"
                        } else {
                            "Already used by"
                        };
                        let tooltip = format!("{prefix} {display_name}");
                        KeyCellState::Conflict {
                            tooltip,
                            placement,
                            anchor,
                        }
                    }
                };
                let pickable = !is_conflict || inputs.allow_conflict_pick;
                let board_cell = KeyCell::new(key_code, label, width, state, pickable);
                cells.push(board_cell);
            }
            column_rows.push(cells);
        }
        Self::new(column_rows)
    }
}

/// The key picker's own shell, shaped from its model: the open value driving the
/// backdrop, the change handler that writes the open signal, the header props, and
/// the scroll-region body props. Every dialog owns its shell now — there is no base.
pub(super) struct KeyPickerShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) header: DialogHeaderProps,
    pub(super) body: KeyPickerBodyProps,
}

impl From<&KeyPickerModel> for KeyPickerShell {
    fn from(model: &KeyPickerModel) -> Self {
        let mut open_signal = model.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = model.open;
        let title = model.title.clone();
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let header = DialogHeaderProps { title, on_close };
        let board = model.board.clone();
        let body = KeyPickerBodyProps { board };
        Self {
            open,
            on_open_change,
            header,
            body,
        }
    }
}
