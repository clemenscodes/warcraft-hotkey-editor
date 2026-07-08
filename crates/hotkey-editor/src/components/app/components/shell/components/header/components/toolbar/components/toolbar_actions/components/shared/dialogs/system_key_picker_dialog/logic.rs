use super::data::BoardKey;
use super::hooks::SystemKeyPickerModel;
use super::state::BoardSection;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::DialogProps;
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use crate::components::app::components::shell::components::shared::key_picker_board::{
    KeyCell, KeyCellState, KeyColumn, KeyWidth,
};
use crate::components::app::components::shell::components::shared::key_picker_board_host::KeyPickerBoardHost;
use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_keybinds::KeyCode;

/// Everything one board column needs to resolve every one of its keys. The
/// [`BoardSection`] is the only thing that differs between the keyboard and the
/// numpad, so both columns are built from one [`SystemKeyColumn::build`] with no
/// copy-pasted loop.
pub(super) struct ColumnInputs<'a> {
    pub(super) section: BoardSection,
    pub(super) rows: &'static [&'static [BoardKey]],
    pub(super) current_code: KeyCode,
    pub(super) conflicts: &'a HashMap<KeyCode, Vec<String>>,
}

/// Resolves one board column to its [`KeyCell`] rows — each key marked current /
/// conflict / available, given its cap label, wide flag, and (for a conflict) its
/// tooltip text, placement, and anchor.
impl From<&ColumnInputs<'_>> for KeyColumn {
    fn from(inputs: &ColumnInputs) -> Self {
        let total_row_count = inputs.rows.len();
        let mut rows: Vec<Vec<KeyCell>> = Vec::new();
        for (row_index, row) in inputs.rows.iter().enumerate() {
            let is_bottom_row = row_index + 2 >= total_row_count;
            let placement = if is_bottom_row {
                TooltipPlacement::Above
            } else {
                TooltipPlacement::Below
            };
            let last_index = row.len().saturating_sub(1);
            let mut cells: Vec<KeyCell> = Vec::new();
            for (key_index, entry) in row.iter().enumerate() {
                let code = entry.code;
                let label = entry.label.to_string();
                let conflict_names = inputs.conflicts.get(&code);
                let width = match inputs.section {
                    BoardSection::Keyboard => {
                        let is_wide =
                            matches!(entry.label, "Space" | "Mouse4" | "Mouse5" | "Backspace");
                        if is_wide {
                            KeyWidth::Wide
                        } else {
                            KeyWidth::Standard
                        }
                    }
                    BoardSection::Numpad => KeyWidth::Standard,
                };
                let anchor = match inputs.section {
                    BoardSection::Keyboard => {
                        if key_index == 0 {
                            TooltipAnchor::Left
                        } else if key_index == last_index {
                            TooltipAnchor::Right
                        } else {
                            TooltipAnchor::Center
                        }
                    }
                    BoardSection::Numpad => TooltipAnchor::Right,
                };
                let state = if code == inputs.current_code {
                    KeyCellState::Current
                } else if let Some(names) = conflict_names {
                    let joined = names.join(", ");
                    let tooltip = format!("Already used by {joined}");
                    KeyCellState::Conflict {
                        tooltip,
                        placement,
                        anchor,
                    }
                } else {
                    KeyCellState::Available
                };
                let pickable = true;
                let cell = KeyCell::new(code, label, width, state, pickable);
                cells.push(cell);
            }
            rows.push(cells);
        }
        Self::new(rows)
    }
}

impl From<&SystemKeyPickerModel> for DialogProps {
    fn from(model: &SystemKeyPickerModel) -> Self {
        let open = model.open;
        let title = model.title.clone();
        let board = model.board.clone();
        let children = rsx! {
            KeyPickerBoardHost { ..board }
        };
        Self {
            open,
            title,
            children,
            on_open_change: None,
        }
    }
}
