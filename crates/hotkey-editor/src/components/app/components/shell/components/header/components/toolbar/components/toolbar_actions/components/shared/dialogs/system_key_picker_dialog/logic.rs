use super::components::system_key_picker_board::SystemKeyPickerBoard;
use super::components::system_key_picker_board::components::system_key_picker_column::SystemKeyPickerColumnProps;
use super::components::system_key_picker_board::components::system_key_picker_column::components::system_key_picker_row::SystemKeyPickerRowProps;
use super::components::system_key_picker_board::components::system_key_picker_column::components::system_key_picker_row::components::system_key_picker_key::{
    SystemKeyPickerKeyProps, SystemKeyPickerKeyState,
};
use super::data::BoardKey;
use super::hooks::SystemKeyPickerModel;
use super::state::BoardSection;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::DialogProps;
use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_keybinds::KeyCode;

/// Everything one board column needs to resolve every one of its keys. The
/// [`BoardSection`] is the only thing that differs between the keyboard and the
/// numpad, so both columns are built from one [`KeyColumn::build`] with no
/// copy-pasted loop.
#[derive(Clone)]
pub(super) struct ColumnInputs<'a> {
    pub(super) section: BoardSection,
    pub(super) rows: &'static [&'static [BoardKey]],
    pub(super) current_code: KeyCode,
    pub(super) conflicts: &'a HashMap<KeyCode, Vec<String>>,
    pub(super) on_pick: EventHandler<KeyCode>,
}

/// One board column resolved to its row props — each key marked current / conflict /
/// normal, given its tooltip, placement, anchor, and wide flag. Replaces the two
/// near-duplicate build loops the picker hook used to carry.
#[derive(Clone, PartialEq)]
pub(super) struct KeyColumn {
    props: SystemKeyPickerColumnProps,
}

impl KeyColumn {
    pub(super) fn build(inputs: &ColumnInputs) -> Self {
        let total_row_count = inputs.rows.len();
        let mut rows: Vec<SystemKeyPickerRowProps> = Vec::new();
        for (row_index, row) in inputs.rows.iter().enumerate() {
            let is_bottom_row = row_index + 2 >= total_row_count;
            let placement = if is_bottom_row { "above" } else { "below" };
            let last_index = row.len().saturating_sub(1);
            let mut keys: Vec<SystemKeyPickerKeyProps> = Vec::new();
            for (key_index, entry) in row.iter().enumerate() {
                let code = entry.code;
                let label = entry.label;
                let conflict_names = inputs.conflicts.get(&code);
                let state = if code == inputs.current_code {
                    SystemKeyPickerKeyState::Current
                } else if conflict_names.is_some() {
                    SystemKeyPickerKeyState::Conflict
                } else {
                    SystemKeyPickerKeyState::Normal
                };
                let title = conflict_names
                    .map(|names| format!("Already used by {}", names.join(", ")))
                    .unwrap_or_default();
                let anchor = match inputs.section {
                    BoardSection::Keyboard => {
                        if key_index == 0 {
                            "left"
                        } else if key_index == last_index {
                            "right"
                        } else {
                            ""
                        }
                    }
                    BoardSection::Numpad => "right",
                };
                let is_wide = match inputs.section {
                    BoardSection::Keyboard => {
                        matches!(label, "Space" | "Mouse4" | "Mouse5" | "Backspace")
                    }
                    BoardSection::Numpad => false,
                };
                let wide = if is_wide { "true" } else { "" };
                let on_pick = inputs.on_pick;
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
            rows.push(row_props);
        }
        let props = SystemKeyPickerColumnProps { rows };
        Self { props }
    }

    pub(super) fn into_props(self) -> SystemKeyPickerColumnProps {
        self.props
    }
}

impl From<&SystemKeyPickerModel> for DialogProps {
    fn from(model: &SystemKeyPickerModel) -> Self {
        let open = model.open;
        let title = model.title.clone();
        let board = model.board.clone();
        let children = rsx! {
            SystemKeyPickerBoard { ..board }
        };
        Self {
            open,
            title,
            children,
            footer: None,
            on_open_change: None,
        }
    }
}
