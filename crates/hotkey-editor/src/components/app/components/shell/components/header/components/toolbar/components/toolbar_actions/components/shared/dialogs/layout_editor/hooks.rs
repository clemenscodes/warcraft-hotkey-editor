use super::components::layout_grid::components::layout_cell::{LayoutCellProps, LayoutCellState};
use super::data::QWERTY_ROWS;
use super::props::LayoutEditorProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::{KeyPickerCell, KeyPickerCellState};
use crate::components::app::components::shell::components::toasts::{ToastOptions, use_toast};
use crate::services::customkeys::context::use_custom_keys_service;
use dioxus::prelude::*;

use warcraft_keybinds::{
    COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, ColumnIndex, GridCoordinate, HotkeyToken, RowIndex,
};

/// Everything the layout editor's markup needs, already shaped: the grid cells,
/// the key-picker state, the toggle state, and every handler. The body only
/// places these; all the work happens here.
pub(super) struct LayoutEditorModel {
    pub(super) open: Signal<bool>,
    pub(super) cells: Vec<LayoutCellProps>,
    pub(super) picker_open: bool,
    pub(super) picker_rows: Vec<Vec<KeyPickerCell>>,
    pub(super) on_pick: EventHandler<HotkeyToken>,
    pub(super) on_picker_close: EventHandler<()>,
    pub(super) on_apply: EventHandler<MouseEvent>,
    pub(super) toggle_checked: bool,
    pub(super) on_toggle: EventHandler<FormEvent>,
}

/// Composes the layout editor's state and behavior. Builds the twelve grid cells
/// with their drag/click handlers, resolves the key-picker rows from the current
/// layout, and wires the apply, pick, toggle, and guarded open-change handlers.
pub(super) fn use_layout_editor(props: &LayoutEditorProps) -> LayoutEditorModel {
    let open = props.open;
    let mut grid_layout = props.grid_layout;
    let mut editing_layout_cell = props.editing_layout_cell;
    let mut dragging_layout_cell = props.dragging_layout_cell;
    let custom_keys_service = use_custom_keys_service();
    let mut update_hotkeys_on_move = props.update_hotkeys_on_move;
    let mut layout_dialog_open = props.open;
    let layout_snapshot = *grid_layout.read();
    let editing_snapshot = *editing_layout_cell.read();
    let toast_api = use_toast();
    let mut cells: Vec<LayoutCellProps> = Vec::new();
    for row in 0..COMMAND_GRID_ROWS {
        for column in 0..COMMAND_GRID_COLUMNS {
            let column_index = ColumnIndex::try_from(column).ok();
            let row_index = RowIndex::try_from(row).ok();
            let coordinate_option = column_index
                .zip(row_index)
                .map(|(col, row_idx)| GridCoordinate::new(col, row_idx));
            let current_letter = coordinate_option
                .and_then(|coordinate| {
                    layout_snapshot.letter_at(coordinate.column(), coordinate.row())
                })
                .map(|letter| letter.to_string())
                .unwrap_or_default();
            let is_editing = editing_snapshot == coordinate_option;
            let state = if is_editing {
                LayoutCellState::Editing
            } else {
                LayoutCellState::Idle
            };
            let label = if is_editing {
                String::from("…")
            } else {
                current_letter
            };
            let ondragstart = EventHandler::new(move |_event: Event<DragData>| {
                if let Some(coordinate) = coordinate_option {
                    dragging_layout_cell.set(Some(coordinate));
                }
            });
            let ondragend = EventHandler::new(move |_event: Event<DragData>| {
                dragging_layout_cell.set(None);
            });
            let ondragover = EventHandler::new(move |event: Event<DragData>| {
                event.prevent_default();
            });
            let ondrop = EventHandler::new(move |event: Event<DragData>| {
                event.prevent_default();
                let source_option = *dragging_layout_cell.read();
                let Some(source_cell) = source_option else {
                    return;
                };
                let source_column = u8::from(source_cell.column());
                let source_row = u8::from(source_cell.row());
                if source_column == column && source_row == row {
                    dragging_layout_cell.set(None);
                    return;
                }
                let mut next_layout = *grid_layout.read();
                next_layout.swap_cells(source_column, source_row, column, row);
                grid_layout.set(next_layout);
                dragging_layout_cell.set(None);
            });
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                if let Some(coordinate) = coordinate_option {
                    editing_layout_cell.set(Some(coordinate));
                }
            });
            let cell = LayoutCellProps {
                state,
                label,
                row,
                column,
                ondragstart,
                ondragend,
                ondragover,
                ondrop,
                onclick,
            };
            cells.push(cell);
        }
    }
    let picker_open = editing_snapshot.is_some();
    let picker_rows: Vec<Vec<KeyPickerCell>> = if let Some(active_cell) = editing_snapshot {
        let current_letter = layout_snapshot
            .letter_at(active_cell.column(), active_cell.row())
            .map(|character| character.to_ascii_uppercase());
        QWERTY_ROWS
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&letter| {
                        let token = HotkeyToken::try_from(letter)
                            .expect("QWERTY layout letters are A to Z");
                        let upper_letter = letter.to_ascii_uppercase();
                        let state = if Some(upper_letter) == current_letter {
                            KeyPickerCellState::Current
                        } else if let Some(other_position) =
                            layout_snapshot.position_for_letter(upper_letter)
                        {
                            let display_row = u8::from(other_position.row()) + 1;
                            let display_column = u8::from(other_position.column()) + 1;
                            let display_name =
                                format!("row {display_row}, column {display_column}",);
                            KeyPickerCellState::Conflict { display_name }
                        } else {
                            KeyPickerCellState::Available
                        };
                        KeyPickerCell::new(token, state)
                    })
                    .collect()
            })
            .collect()
    } else {
        Vec::new()
    };
    let on_apply = EventHandler::new(move |_event: MouseEvent| {
        let snapshot = *grid_layout.read();
        let changed_count = custom_keys_service.apply_grid_layout(snapshot);
        if changed_count > 0 {
            let hotkey_word = if changed_count == 1 {
                "HOTKEY"
            } else {
                "HOTKEYS"
            };
            let message = format!("{changed_count} {hotkey_word} UPDATED");
            let options = ToastOptions::new().description(message);
            let title = "GRID APPLIED".to_string();
            toast_api.success(title, options);
        }
        layout_dialog_open.set(false);
    });
    let on_pick = EventHandler::new(move |token: HotkeyToken| {
        let Some(active_cell) = *editing_layout_cell.read() else {
            return;
        };
        let Ok(letter) = char::try_from(token) else {
            return;
        };
        let mut next_layout = *grid_layout.read();
        let active_column = u8::from(active_cell.column());
        let active_row = u8::from(active_cell.row());
        next_layout.assign_unique(active_column, active_row, letter);
        grid_layout.set(next_layout);
        editing_layout_cell.set(None);
    });
    let on_picker_close = EventHandler::new(move |_event: ()| editing_layout_cell.set(None));
    let toggle_checked = *update_hotkeys_on_move.read();
    let on_toggle = EventHandler::new(move |_event: FormEvent| {
        let current = *update_hotkeys_on_move.read();
        update_hotkeys_on_move.set(!current);
    });
    LayoutEditorModel {
        open,
        cells,
        picker_open,
        picker_rows,
        on_pick,
        on_picker_close,
        on_apply,
        toggle_checked,
        on_toggle,
    }
}
