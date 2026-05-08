use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, use_toast};

const LAYOUT_EDITOR_STYLES: Asset =
    asset!("/src/components/dialogs/layout_editor/layout_editor.css");
use warcraft_keybinds::{ColumnIndex, CustomKeys, RowIndex};

use crate::components::shared::key_picker::{KeyPicker, KeyPickerCell, KeyPickerCellState};
use warcraft_keybinds::HotkeyToken;

use crate::model::grid::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, EditingCell, GridLayout};
use crate::services::customkeys::positions::Positions;

const QWERTY_ROWS: &[&[char]] = &[
    &['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
    &['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'],
    &['Z', 'X', 'C', 'V', 'B', 'N', 'M'],
];

const LAYOUT_EDITOR: &str = "flex flex-col gap-[2rem] items-center justify-center flex-none \
    max-[1099px]:w-full max-[1099px]:max-w-full max-[1099px]:p-[0.75rem] max-[1099px]:gap-[0.5rem]";

const LAYOUT_EDITOR_GRID: &str = "grid \
    [grid-template-columns:repeat(4,clamp(7rem,9vh,12rem))] \
    [grid-auto-rows:clamp(7rem,9vh,12rem)] \
    gap-[1.25rem] mx-auto \
    max-[1099px]:justify-center \
    max-[1099px]:[grid-template-columns:repeat(4,clamp(52px,18vw,72px))] \
    max-[1099px]:[grid-auto-rows:clamp(52px,18vw,72px)] \
    max-[1099px]:gap-[8px]";

const LAYOUT_CELL: &str = "w-[clamp(7rem,9vh,12rem)] h-[clamp(7rem,9vh,12rem)] \
    bg-[rgba(40,30,8,0.75)] border-2 border-warcraft-gold rounded-[10px] \
    text-warcraft-gold font-friz-quadrata text-[clamp(3.5rem,5vh,6rem)] leading-none uppercase \
    flex items-center justify-center p-0 \
    [text-shadow:1px_1px_0_#000,-1px_1px_0_#000,1px_-1px_0_#000,-1px_-1px_0_#000] \
    hover:[box-shadow:0_0_8px_rgba(255,206,99,0.5)] hover:bg-[rgba(255,206,99,0.12)] \
    focus:outline-none \
    [body[data-kb-modality]_&]:focus:outline-none \
    [body[data-kb-modality]_&]:focus:border-white \
    [body[data-kb-modality]_&]:focus:bg-[rgba(255,255,255,0.12)] \
    [body[data-kb-modality]_&]:focus:[box-shadow:0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)] \
    [@media(hover:none)]:[body[data-kb-modality]_&]:focus-visible:border-warcraft-gold \
    [@media(hover:none)]:[body[data-kb-modality]_&]:focus-visible:bg-[rgba(40,30,8,0.75)] \
    [@media(hover:none)]:[body[data-kb-modality]_&]:focus-visible:[box-shadow:none] \
    [@media(hover:none)]:[body[data-kb-modality]_&]:focus-visible:text-warcraft-gold \
    max-[1099px]:w-[clamp(52px,18vw,72px)] max-[1099px]:h-[clamp(52px,18vw,72px)] \
    max-[1099px]:text-[clamp(22px,7vw,34px)]";

const LAYOUT_CELL_EDITING: &str = "[background:linear-gradient(135deg,rgba(255,206,99,0.3)_0%,rgba(255,171,1,0.18)_100%)] \
    border-warcraft-gold \
    [box-shadow:0_0_16px_rgba(255,206,99,0.65),inset_0_0_12px_rgba(255,206,99,0.25)] \
    text-warcraft-gold \
    [animation:pulse-editing_1s_ease-in-out_infinite_alternate]";

const LAYOUT_APPLY_BUTTON: &str = "mt-[0.85rem] px-[3rem] py-[1.4rem] \
    [background:linear-gradient(180deg,rgba(40,30,8,0.65)_0%,rgba(15,12,4,0.65)_100%)] \
    border border-[#6c5a1f] rounded-[10px] \
    text-warcraft-gold font-friz-quadrata text-[2rem] tracking-[0.06em] uppercase cursor-pointer \
    [text-shadow:1px_1px_0_#000] \
    [transition:border-color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease] \
    hover:border-warcraft-gold \
    hover:[background:linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.65)_100%)] \
    hover:[box-shadow:0_0_10px_rgba(255,206,99,0.35)] \
    active:translate-y-[1px] \
    max-[1099px]:w-full max-[1099px]:min-h-[44px] max-[1099px]:text-[16px] \
    max-[1099px]:py-[12px] max-[1099px]:px-[24px]";

#[component]
pub(crate) fn LayoutEditor(
    mut grid_layout: Signal<GridLayout>,
    mut editing_layout_cell: Signal<Option<EditingCell>>,
    mut dragging_layout_cell: Signal<Option<EditingCell>>,
    mut loaded_keys: Signal<Option<CustomKeys>>,
) -> Element {
    let layout_snapshot = *grid_layout.read();
    let editing_snapshot = *editing_layout_cell.read();
    let toast_api = use_toast();

    let picker_open = editing_snapshot.is_some();
    let picker_rows: Vec<Vec<KeyPickerCell>> = if let Some(active_cell) = editing_snapshot {
        let active_column = ColumnIndex::try_from(active_cell.column()).ok();
        let active_row = RowIndex::try_from(active_cell.row()).ok();
        let current_letter = active_column
            .zip(active_row)
            .and_then(|(col, row)| layout_snapshot.letter_at(col, row))
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
                            let display_row = other_position.row().as_u8() + 1;
                            let display_column = other_position.column().as_u8() + 1;
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

    let apply_grid = move |_| {
        let snapshot = *grid_layout.read();
        let changed_count = Positions::apply_grid_to_all_known_objects(&mut loaded_keys, snapshot);
        if changed_count > 0 {
            let hotkey_word = if changed_count == 1 {
                "HOTKEY"
            } else {
                "HOTKEYS"
            };
            let message = format!("{changed_count} {hotkey_word} UPDATED");
            toast_api.success(
                "GRID APPLIED".to_string(),
                ToastOptions::new().description(message),
            );
        }
    };
    let handle_pick = move |token: HotkeyToken| {
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
    };
    let handle_picker_close = move |_| editing_layout_cell.set(None);

    rsx! {
        document::Stylesheet { href: LAYOUT_EDITOR_STYLES }
        div { class: LAYOUT_EDITOR,
            div { class: LAYOUT_EDITOR_GRID,
                for row in 0..COMMAND_GRID_ROWS {
                    for column in 0..COMMAND_GRID_COLUMNS {
                        {
                            let column_index = ColumnIndex::try_from(column).ok();
                            let row_index = RowIndex::try_from(row).ok();
                            let current_letter = column_index
                                .zip(row_index)
                                .and_then(|(col, row_idx)| layout_snapshot.letter_at(col, row_idx))
                                .map(|letter| letter.to_string())
                                .unwrap_or_default();
                            let is_editing = editing_snapshot
                                == Some(EditingCell::new(column, row));
                            let cell_class = if is_editing {
                                format!("{LAYOUT_CELL} {LAYOUT_CELL_EDITING}")
                            } else {
                                LAYOUT_CELL.to_string()
                            };
                            let cell_label = if is_editing {
                                String::from("…")
                            } else {
                                current_letter.clone()
                            };
                            let handle_drag_start = move |_| {
                                let cell = EditingCell::new(column, row);
                                dragging_layout_cell.set(Some(cell));
                            };
                            let handle_drag_end = move |_| {
                                dragging_layout_cell.set(None);
                            };
                            let handle_drag_over = move |event: Event<DragData>| {
                                event.prevent_default();
                            };
                            let handle_drop = move |event: Event<DragData>| {
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
                            };
                            let handle_click = move |_| {
                                let cell = EditingCell::new(column, row);
                                editing_layout_cell.set(Some(cell));
                            };
                            rsx! {
                                button {
                                    class: cell_class,
                                    draggable: "true",
                                    "data-layout-row": row,
                                    "data-layout-col": column,
                                    ondragstart: handle_drag_start,
                                    ondragend: handle_drag_end,
                                    ondragover: handle_drag_over,
                                    ondrop: handle_drop,
                                    onclick: handle_click,
                                    "{cell_label}"
                                }
                            }
                        }
                    }
                }
            }
            button {
                class: LAYOUT_APPLY_BUTTON,
                r#type: "button",
                onclick: apply_grid,
                "Apply grid to all hotkeys"
            }
        }
        if picker_open {
            KeyPicker {
                title: "Pick a grid key".to_string(),
                rows: picker_rows,
                open: true,
                allow_conflict_pick: true,
                on_pick: handle_pick,
                on_close: handle_picker_close,
            }
        }
    }
}
