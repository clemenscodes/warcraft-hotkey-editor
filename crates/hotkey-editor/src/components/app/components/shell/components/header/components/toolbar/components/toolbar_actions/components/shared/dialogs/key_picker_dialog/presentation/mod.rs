use super::model::KeyPickerDialogModel;
use super::state::{KeyPickerCell, KeyPickerCellState};
use crate::components::app::components::shell::components::shared::key_picker_board::{
    KeyCell, KeyCellState, KeyColumn, KeyWidth,
};
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use dioxus::prelude::*;
use warcraft_keybinds::{HotkeyToken, KeyCode};

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

pub(super) struct KeyPickerDialogPresentation {
    pub(super) open: bool,
    pub(super) title: String,
    pub(super) columns: Vec<KeyColumn>,
    pub(super) on_pick: EventHandler<KeyCode>,
    pub(super) on_close: EventHandler<()>,
    pub(super) on_open_change: Callback<bool>,
}

impl ddd::Presentation for KeyPickerDialogPresentation {
    type Model = KeyPickerDialogModel;
}

pub(super) fn use_key_picker_dialog_presentation(
    model: &KeyPickerDialogModel,
) -> KeyPickerDialogPresentation {
    let parent_on_close = model.on_close;
    let mut open_signal = use_signal(|| model.open);
    use_effect(move || {
        if !open_signal() {
            parent_on_close.call(());
        }
    });
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    let on_close = EventHandler::new(move |()| open_signal.set(false));
    let column_inputs = LetterColumnInputs {
        rows: model.rows.clone(),
        allow_conflict_pick: model.allow_conflict_pick,
    };
    let column = KeyColumn::from(column_inputs);
    let columns: Vec<KeyColumn> = vec![column];
    let letter_on_pick = model.on_pick;
    let on_pick = EventHandler::new(move |code: KeyCode| {
        if let Ok(token) = HotkeyToken::try_from(code) {
            letter_on_pick.call(token);
        }
    });
    let title = model.title.clone();
    let current_open = open_signal();
    KeyPickerDialogPresentation {
        open: current_open,
        title,
        columns,
        on_pick,
        on_close,
        on_open_change,
    }
}
