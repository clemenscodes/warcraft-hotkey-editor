use super::data::BoardKey;
use super::state::BoardSection;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyCell;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyCellState;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyWidth;
use crate::components::app::components::shell::components::shared::tooltip::TooltipAnchor;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_keybinds::KeyCode;

pub(super) struct ColumnInputs<'a> {
    pub(super) section: BoardSection,
    pub(super) rows: &'static [&'static [BoardKey]],
    pub(super) current_code: KeyCode,
    pub(super) conflicts: &'a HashMap<KeyCode, Vec<String>>,
}

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

pub(super) struct SystemKeyPickerDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: String,
    pub(super) columns: Vec<KeyColumn>,
    pub(super) on_pick: EventHandler<KeyCode>,
    pub(super) board_on_close: EventHandler<()>,
}

impl From<&SystemKeyPickerDialogPresentation> for SystemKeyPickerDialogShell {
    fn from(model: &SystemKeyPickerDialogPresentation) -> Self {
        let mut open_signal = model.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let title = model.title.clone();
        let columns = model.columns.clone();
        let on_pick = model.on_pick;
        let board_on_close = model.board_on_close;
        Self {
            open,
            on_open_change,
            title,
            columns,
            on_pick,
            board_on_close,
        }
    }
}
use super::data::KEYBOARD_ROWS;
use super::data::NUMPAD_ROWS;
use super::model::SystemKeyPickerDialogModel;

pub(super) struct SystemKeyPickerDialogPresentation {
    pub(super) open: Signal<bool>,
    pub(super) title: String,
    pub(super) columns: Vec<KeyColumn>,
    pub(super) on_pick: EventHandler<KeyCode>,
    pub(super) board_on_close: EventHandler<()>,
}

pub(super) fn use_system_key_picker(
    props: &SystemKeyPickerDialogModel,
) -> SystemKeyPickerDialogPresentation {
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
    SystemKeyPickerDialogPresentation {
        open,
        title,
        columns,
        on_pick,
        board_on_close,
    }
}

impl ddd::Presentation for SystemKeyPickerDialogPresentation {
    type Model = SystemKeyPickerDialogModel;
}
