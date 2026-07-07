use super::components::apply_button::{ApplyButton, ApplyButtonProps};
use super::components::layout_editor_content::LayoutEditorContent;
use super::components::layout_grid::components::layout_tile::{LayoutTileProps, LayoutTileState};
use super::components::layout_grid::{LayoutGrid, LayoutGridProps};
use super::components::layout_intro::LayoutIntro;
use super::components::move_hotkey_toggle::{MoveHotkeyToggle, MoveHotkeyToggleProps};
use super::data::QWERTY_ROWS;
use super::hooks::LayoutEditorModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::DialogProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::{
    KeyPickerCell, KeyPickerCellState, KeyPickerProps,
};
use crate::services::grid_layout::service::GridLayoutService;
use dioxus::prelude::*;
use warcraft_keybinds::{
    COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, ColumnIndex, GridCoordinate, GridLayout, HotkeyToken,
    RowIndex,
};

impl From<&LayoutEditorModel> for DialogProps {
    fn from(model: &LayoutEditorModel) -> Self {
        let open = model.open;
        let title = String::from("Global Hotkey Layout");
        let apply = ApplyButtonProps {
            on_apply: model.on_apply,
        };
        let footer = Some(rsx! {
            ApplyButton { ..apply }
        });
        let grid = LayoutGridProps {
            cells: model.cells.clone(),
        };
        let toggle = MoveHotkeyToggleProps {
            checked: model.toggle_checked,
            on_toggle: model.on_toggle,
        };
        let children = rsx! {
            LayoutEditorContent {
                LayoutIntro {}
                LayoutGrid { ..grid }
                MoveHotkeyToggle { ..toggle }
            }
        };
        let on_open_change = Some(model.on_dialog_open_change);
        Self {
            open,
            title,
            children,
            footer,
            on_open_change,
        }
    }
}

impl From<&LayoutEditorModel> for KeyPickerProps {
    fn from(model: &LayoutEditorModel) -> Self {
        let title = String::from("Pick a grid key");
        let rows = model.picker_rows.clone();
        let open = true;
        let allow_conflict_pick = true;
        let on_pick = model.on_pick;
        let on_close = model.on_picker_close;
        Self {
            title,
            rows,
            open,
            allow_conflict_pick,
            on_pick,
            on_close,
        }
    }
}

/// The signals, service, and snapshots every grid cell resolves against. Shared by
/// all twelve cells the [`LayoutGridCells`] builder produces: the two snapshots
/// decide each cell's letter and editing state, while the signals and service are
/// what its drag/click handlers mutate.
#[derive(Clone, Copy)]
pub(super) struct GridCellContext {
    pub(super) grid_layout: Signal<GridLayout>,
    pub(super) editing_layout_tile: Signal<Option<GridCoordinate>>,
    pub(super) dragging_layout_tile: Signal<Option<GridCoordinate>>,
    pub(super) grid_layout_service: GridLayoutService,
    pub(super) layout: GridLayout,
    pub(super) editing_snapshot: Option<GridCoordinate>,
}

impl GridCellContext {
    /// Resolve one grid cell at the given address: its visual state, the letter it
    /// shows, and the five drag/click handlers. The drag-drop handler routes a cell
    /// swap through the
    /// [`GridLayoutService`](crate::services::grid_layout::service::GridLayoutService).
    fn cell(&self, row: u8, column: u8) -> LayoutTileProps {
        let mut editing_layout_tile = self.editing_layout_tile;
        let mut dragging_layout_tile = self.dragging_layout_tile;
        let grid_layout = self.grid_layout;
        let grid_layout_service = self.grid_layout_service;
        let column_index = ColumnIndex::try_from(column).ok();
        let row_index = RowIndex::try_from(row).ok();
        let coordinate_option = column_index
            .zip(row_index)
            .map(|(col, row_idx)| GridCoordinate::new(col, row_idx));
        let current_letter = coordinate_option
            .and_then(|coordinate| self.layout.letter_at(coordinate.column(), coordinate.row()))
            .map(|letter| letter.to_string())
            .unwrap_or_default();
        let is_editing = self.editing_snapshot == coordinate_option;
        let state = if is_editing {
            LayoutTileState::Editing
        } else {
            LayoutTileState::Idle
        };
        let label = if is_editing {
            String::from("…")
        } else {
            current_letter
        };
        let ondragstart = EventHandler::new(move |_event: Event<DragData>| {
            if let Some(coordinate) = coordinate_option {
                dragging_layout_tile.set(Some(coordinate));
            }
        });
        let ondragend = EventHandler::new(move |_event: Event<DragData>| {
            dragging_layout_tile.set(None);
        });
        let ondragover = EventHandler::new(move |event: Event<DragData>| {
            event.prevent_default();
        });
        let ondrop = EventHandler::new(move |event: Event<DragData>| {
            event.prevent_default();
            let source_option = *dragging_layout_tile.read();
            let Some(source_cell) = source_option else {
                return;
            };
            let source_column = u8::from(source_cell.column());
            let source_row = u8::from(source_cell.row());
            if source_column == column && source_row == row {
                dragging_layout_tile.set(None);
                return;
            }
            let mut next_layout = *grid_layout.read();
            next_layout.swap_cells(source_column, source_row, column, row);
            grid_layout_service.select(next_layout);
            dragging_layout_tile.set(None);
        });
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            if let Some(coordinate) = coordinate_option {
                editing_layout_tile.set(Some(coordinate));
            }
        });
        LayoutTileProps {
            state,
            label,
            row,
            column,
            ondragstart,
            ondragend,
            ondragover,
            ondrop,
            onclick,
        }
    }
}

/// The twelve editable grid cells, resolved from a [`GridCellContext`] by mapping
/// over the command-grid positions. Mirrors the key picker's board: one fielded
/// carrier the body places, built from data rather than twelve copy-pasted blocks.
pub(super) struct LayoutGridCells {
    pub(super) cells: Vec<LayoutTileProps>,
}

impl LayoutGridCells {
    pub(super) fn build(context: &GridCellContext) -> Self {
        let mut cells: Vec<LayoutTileProps> = Vec::new();
        for row in 0..COMMAND_GRID_ROWS {
            for column in 0..COMMAND_GRID_COLUMNS {
                let cell = context.cell(row, column);
                cells.push(cell);
            }
        }
        Self { cells }
    }

    pub(super) fn into_cells(self) -> Vec<LayoutTileProps> {
        self.cells
    }
}

/// The inputs the picker board resolves each key against: the current layout and the
/// cell being edited.
#[derive(Clone, Copy)]
pub(super) struct LayoutPickerContext {
    pub(super) layout: GridLayout,
    pub(super) active_cell: GridCoordinate,
}

/// The QWERTY keyboard laid out as picker cells, each marked current / conflicting
/// (naming the row and column it already occupies) / available. Mirrors the override
/// panel's picker board.
pub(super) struct LayoutPickerBoard {
    pub(super) rows: Vec<Vec<KeyPickerCell>>,
}

impl LayoutPickerBoard {
    pub(super) fn build(context: &LayoutPickerContext) -> Self {
        let layout = context.layout;
        let active_cell = context.active_cell;
        let current_letter = layout
            .letter_at(active_cell.column(), active_cell.row())
            .map(|character| character.to_ascii_uppercase());
        let rows = QWERTY_ROWS
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
                            layout.position_for_letter(upper_letter)
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
            .collect();
        Self { rows }
    }

    pub(super) fn into_rows(self) -> Vec<Vec<KeyPickerCell>> {
        self.rows
    }
}
