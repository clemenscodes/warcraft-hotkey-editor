use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, use_toast};
use warcraft_keybinds::CustomKeysFile;

use crate::components::key_picker::{KeyPicker, KeyPickerCell, KeyPickerCellState};
use crate::domain::grid_layout::{
    COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, EditingCell, GridLayout,
};
use crate::domain::hotkey_token::HotkeyToken;
use crate::domain::positions::Positions;

const QWERTY_ROWS: &[&[char]] = &[
    &['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
    &['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'],
    &['Z', 'X', 'C', 'V', 'B', 'N', 'M'],
];

#[component]
pub(crate) fn LayoutEditor(
    mut grid_layout: Signal<GridLayout>,
    mut editing_layout_cell: Signal<Option<EditingCell>>,
    mut dragging_layout_cell: Signal<Option<EditingCell>>,
    mut loaded_keys: Signal<Option<CustomKeysFile>>,
) -> Element {
    let layout_snapshot = *grid_layout.read();
    let editing_snapshot = *editing_layout_cell.read();
    let toast_api = use_toast();

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
                        let token = HotkeyToken::from(letter);
                        let upper_letter = letter.to_ascii_uppercase();
                        let state = if Some(upper_letter) == current_letter {
                            KeyPickerCellState::Current
                        } else if let Some(other_position) =
                            layout_snapshot.position_for_letter(upper_letter)
                        {
                            let display_name = format!(
                                "row {row}, column {column}",
                                row = other_position.1 + 1,
                                column = other_position.0 + 1,
                            );
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

    rsx! {
        div { class: "layout-editor",
            div { class: "layout-editor-grid",
                for row in 0..COMMAND_GRID_ROWS {
                    for column in 0..COMMAND_GRID_COLUMNS {
                        {
                            let current_letter = layout_snapshot
                                .letter_at(column, row)
                                .map(|letter| letter.to_string())
                                .unwrap_or_default();
                            let is_editing = editing_snapshot
                                == Some(EditingCell::new(column, row));
                            let class_name = if is_editing {
                                "layout-cell editing"
                            } else {
                                "layout-cell"
                            };
                            let cell_label = if is_editing {
                                String::from("…")
                            } else {
                                current_letter.clone()
                            };
                            rsx! {
                                button {
                                    class: "{class_name}",
                                    draggable: "true",
                                    "data-layout-row": "{row}",
                                    "data-layout-col": "{column}",
                                    ondragstart: move |_| {
                                        let cell = EditingCell::new(column, row);
                                        dragging_layout_cell.set(Some(cell));
                                    },
                                    ondragend: move |_| {
                                        dragging_layout_cell.set(None);
                                    },
                                    ondragover: move |event| {
                                        event.prevent_default();
                                    },
                                    ondrop: move |event| {
                                        event.prevent_default();
                                        let source_option = *dragging_layout_cell.read();
                                        let Some(source_cell) = source_option else {
                                            return;
                                        };
                                        if source_cell.column() == column && source_cell.row() == row {
                                            dragging_layout_cell.set(None);
                                            return;
                                        }
                                        let mut next_layout = *grid_layout.read();
                                        next_layout.swap_cells(
                                            source_cell.column(),
                                            source_cell.row(),
                                            column,
                                            row,
                                        );
                                        grid_layout.set(next_layout);
                                        dragging_layout_cell.set(None);
                                    },
                                    onclick: move |_| {
                                        let cell = EditingCell::new(column, row);
                                        editing_layout_cell.set(Some(cell));
                                    },
                                    "{cell_label}"
                                }
                            }
                        }
                    }
                }
            }
            button {
                class: "layout-editor-apply-button",
                r#type: "button",
                onclick: move |_| {
                    let snapshot = *grid_layout.read();
                    let changed_count =
                        Positions::apply_grid_to_all_known_objects(&mut loaded_keys, snapshot);
                    if changed_count > 0 {
                        let hotkey_word = if changed_count == 1 { "HOTKEY" } else { "HOTKEYS" };
                        let message =
                            format!("{changed_count} {hotkey_word} UPDATED");
                        toast_api.success(
                            "GRID APPLIED".to_string(),
                            ToastOptions::new().description(message),
                        );
                    }
                },
                "Apply grid to all hotkeys"
            }
        }
        if picker_open {
            KeyPicker {
                title: "Pick a grid key".to_string(),
                rows: picker_rows,
                open: true,
                allow_conflict_pick: true,
                on_pick: move |token: HotkeyToken| {
                    let Some(active_cell) = *editing_layout_cell.read() else {
                        return;
                    };
                    let Ok(letter) = char::try_from(token) else {
                        return;
                    };
                    let mut next_layout = *grid_layout.read();
                    next_layout.assign_unique(active_cell.column(), active_cell.row(), letter);
                    grid_layout.set(next_layout);
                    editing_layout_cell.set(None);
                },
                on_close: move |_| editing_layout_cell.set(None),
            }
        }
    }
}
