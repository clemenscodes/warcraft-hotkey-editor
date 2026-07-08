use super::components::key_picker_column::KeyPickerColumnProps;
use super::components::key_picker_column::components::key_picker_row::KeyPickerRowProps;
use super::components::key_picker_column::components::key_picker_row::components::key_picker_key::KeyPickerKeyProps;
use super::key_event::BrowserKeyEvent;
use super::props::KeyPickerBoardProps;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// The board's fully shaped view: the column props to render and the one keydown
/// handler the focusable board wires. Built by `From` so the body only places these.
pub(super) struct KeyPickerBoardPresentation {
    pub(super) columns: Vec<KeyPickerColumnProps>,
    pub(super) onkeydown: EventHandler<Event<KeyboardData>>,
}

impl From<&KeyPickerBoardProps> for KeyPickerBoardPresentation {
    fn from(props: &KeyPickerBoardProps) -> Self {
        let on_pick = props.on_pick;
        let on_close = props.on_close;
        let mut columns: Vec<KeyPickerColumnProps> = Vec::new();
        let mut pickable_codes: Vec<KeyCode> = Vec::new();
        for column in &props.columns {
            let column_codes = column.pickable_codes();
            pickable_codes.extend(column_codes);
            let mut column_rows: Vec<KeyPickerRowProps> = Vec::new();
            for row in column.rows() {
                let mut row_keys: Vec<KeyPickerKeyProps> = Vec::new();
                for cell in row {
                    let cell_clone = cell.clone();
                    let key = KeyPickerKeyProps {
                        cell: cell_clone,
                        on_pick,
                    };
                    row_keys.push(key);
                }
                let row_props = KeyPickerRowProps { keys: row_keys };
                column_rows.push(row_props);
            }
            let column_props = KeyPickerColumnProps { rows: column_rows };
            columns.push(column_props);
        }
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
            let Some(resolved) = browser_event.pick_among(&pickable_codes) else {
                return;
            };
            event.prevent_default();
            on_pick.call(resolved);
        });
        Self { columns, onkeydown }
    }
}
