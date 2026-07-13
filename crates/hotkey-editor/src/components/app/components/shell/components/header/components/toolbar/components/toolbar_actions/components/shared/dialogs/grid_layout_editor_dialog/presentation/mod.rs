use super::components::grid_layout_editor_dialog_body::components::grid_layout_editor_dialog_content::components::layout_grid::components::layout_tile::LayoutTileState;
use super::components::grid_layout_editor_dialog_body::components::grid_layout_editor_dialog_content::components::layout_grid::components::layout_tile::LayoutTileView;
use super::data::QWERTY_ROWS;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker_dialog::KeyPickerCell;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker_dialog::KeyPickerCellState;
use crate::services::grid_layout::service::GridLayoutService;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_COLUMNS;
use warcraft_keybinds::COMMAND_GRID_ROWS;
use warcraft_keybinds::ColumnIndex;
use warcraft_keybinds::GridCoordinate;
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::HotkeyToken;
use warcraft_keybinds::RowIndex;

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
    fn cell(&self, coordinate: GridCoordinate) -> LayoutTileView {
        let mut editing_layout_tile = self.editing_layout_tile;
        let mut dragging_layout_tile = self.dragging_layout_tile;
        let grid_layout = self.grid_layout;
        let grid_layout_service = self.grid_layout_service;
        let column_index = coordinate.column();
        let row_index = coordinate.row();
        let letter_option = self.layout.letter_at(column_index, row_index);
        let current_letter = letter_option
            .map(|letter| letter.to_string())
            .unwrap_or_default();
        let is_editing = self.editing_snapshot == Some(coordinate);
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
            dragging_layout_tile.set(Some(coordinate));
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
            let target_column = u8::from(coordinate.column());
            let target_row = u8::from(coordinate.row());
            if source_column == target_column && source_row == target_row {
                dragging_layout_tile.set(None);
                return;
            }
            let mut next_layout = *grid_layout.read();
            next_layout.swap_cells(source_column, source_row, target_column, target_row);
            grid_layout_service.select(next_layout);
            dragging_layout_tile.set(None);
        });
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            editing_layout_tile.set(Some(coordinate));
        });
        LayoutTileView {
            state,
            label,
            coordinate,
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
    pub(super) cells: Vec<LayoutTileView>,
}

impl LayoutGridCells {
    pub(super) fn build(context: &GridCellContext) -> Self {
        let mut cells: Vec<LayoutTileView> = Vec::new();
        for row in 0..COMMAND_GRID_ROWS {
            for column in 0..COMMAND_GRID_COLUMNS {
                let column_index =
                    ColumnIndex::try_from(column).expect("column is within the command grid");
                let row_index = RowIndex::try_from(row).expect("row is within the command grid");
                let coordinate = GridCoordinate::new(column_index, row_index);
                let cell = context.cell(coordinate);
                cells.push(cell);
            }
        }
        Self { cells }
    }

    pub(super) fn into_cells(self) -> Vec<LayoutTileView> {
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
use crate::components::app::components::shell::components::toasts::ToastOptions;
use crate::components::app::components::shell::components::toasts::use_toast;
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::editor_state::context::use_editor_state;
use crate::services::grid_layout::context::use_grid_layout;
use crate::services::grid_layout::context::use_grid_layout_service;
use crate::services::overlay_state::context::use_overlay_state;

/// Everything the layout editor's markup needs, already shaped: the dialog open
/// state and guard, the panel header, the grid cells, the key-picker state, the
/// toggle state, and every handler. The body only places these; all the work
/// happens here.
pub(super) struct GridLayoutEditorDialogPresentation {
    pub(super) is_open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: String,
    pub(super) cells: Vec<LayoutTileView>,
    pub(super) toggle_checked: bool,
    pub(super) on_toggle: EventHandler<FormEvent>,
    pub(super) on_apply: EventHandler<MouseEvent>,
    pub(super) picker_open: bool,
    pub(super) picker_title: String,
    pub(super) picker_rows: Vec<Vec<KeyPickerCell>>,
    pub(super) picker_allow_conflict_pick: bool,
    pub(super) on_pick: EventHandler<HotkeyToken>,
    pub(super) on_picker_close: EventHandler<()>,
}

/// The apply / pick / picker-close / move-toggle handlers plus the toggle's current
/// checked state. Owns the writes: apply routes the grid through the
/// [`CustomKeysService`](crate::services::customkeys::service::CustomKeysService) and
/// toasts the result; pick routes the chosen letter through the
/// [`GridLayoutService`](crate::services::grid_layout::service::GridLayoutService).
pub(super) struct LayoutActions {
    pub(super) on_apply: EventHandler<MouseEvent>,
    pub(super) on_pick: EventHandler<HotkeyToken>,
    pub(super) on_picker_close: EventHandler<()>,
    pub(super) on_dialog_open_change: Callback<bool>,
    pub(super) toggle_checked: bool,
    pub(super) on_toggle: EventHandler<FormEvent>,
}

fn use_layout_actions(
    grid_layout: Signal<GridLayout>,
    editing_layout_tile: Signal<Option<GridCoordinate>>,
    update_hotkeys_on_move: Signal<bool>,
    open: Signal<bool>,
) -> LayoutActions {
    let mut editing_layout_tile = editing_layout_tile;
    let mut update_hotkeys_on_move = update_hotkeys_on_move;
    let mut layout_dialog_open = open;
    let custom_keys_service = use_custom_keys_service();
    let grid_layout_service = use_grid_layout_service();
    let toast_api = use_toast();
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
        let Some(active_cell) = *editing_layout_tile.read() else {
            return;
        };
        let Ok(letter) = char::try_from(token) else {
            return;
        };
        let mut next_layout = *grid_layout.read();
        let active_column = u8::from(active_cell.column());
        let active_row = u8::from(active_cell.row());
        next_layout.assign_unique(active_column, active_row, letter);
        grid_layout_service.select(next_layout);
        editing_layout_tile.set(None);
    });
    let on_picker_close = EventHandler::new(move |_event: ()| editing_layout_tile.set(None));
    // The key picker is a second modal nested inside this one. When it mounts it
    // takes focus, which makes the base dialog primitive fire a close on this
    // outer dialog — that close must be ignored, or opening the picker would
    // dismiss the whole editor and strand `editing_layout_tile` set, so the
    // editor never reopens. Only a close that arrives while the picker is shut is
    // a real dismiss; then we also clear the editing cell so the next open is clean.
    let on_dialog_open_change = Callback::new(move |is_open: bool| {
        if is_open {
            layout_dialog_open.set(true);
            return;
        }
        let picker_is_open = editing_layout_tile.read().is_some();
        if picker_is_open {
            return;
        }
        editing_layout_tile.set(None);
        layout_dialog_open.set(false);
    });
    let toggle_checked = *update_hotkeys_on_move.read();
    let on_toggle = EventHandler::new(move |_event: FormEvent| {
        let current = *update_hotkeys_on_move.read();
        update_hotkeys_on_move.set(!current);
    });
    LayoutActions {
        on_apply,
        on_pick,
        on_picker_close,
        on_dialog_open_change,
        toggle_checked,
        on_toggle,
    }
}

/// Composes the layout editor's state and behavior. Resolves the twelve grid cells
/// with their drag/click handlers, derives the key-picker rows from the current
/// layout, and wires the apply, pick, and toggle handlers.
pub(super) fn use_grid_layout_editor_dialog() -> GridLayoutEditorDialogPresentation {
    let grid_layout = use_grid_layout();
    let editor_state = use_editor_state();
    let update_hotkeys_on_move = editor_state.update_hotkeys_on_move();
    let overlay_state = use_overlay_state();
    let open = overlay_state.layout_dialog_open();
    let editing_layout_tile = use_signal::<Option<GridCoordinate>>(|| None);
    let dragging_layout_tile = use_signal::<Option<GridCoordinate>>(|| None);
    let grid_layout_service = use_grid_layout_service();
    let layout_snapshot = *grid_layout.read();
    let editing_snapshot = *editing_layout_tile.read();

    let cell_context = GridCellContext {
        grid_layout,
        editing_layout_tile,
        dragging_layout_tile,
        grid_layout_service,
        layout: layout_snapshot,
        editing_snapshot,
    };
    let grid_cells = LayoutGridCells::build(&cell_context);
    let cells = grid_cells.into_cells();

    let picker_open = editing_snapshot.is_some();
    let picker_rows: Vec<Vec<KeyPickerCell>> = if let Some(active_cell) = editing_snapshot {
        let picker_context = LayoutPickerContext {
            layout: layout_snapshot,
            active_cell,
        };
        let board = LayoutPickerBoard::build(&picker_context);
        board.into_rows()
    } else {
        Vec::new()
    };

    let actions = use_layout_actions(
        grid_layout,
        editing_layout_tile,
        update_hotkeys_on_move,
        open,
    );

    let is_open = *open.read();
    let title = String::from("Global Hotkey Layout");
    let picker_title = String::from("Pick a grid key");
    let picker_allow_conflict_pick = true;

    GridLayoutEditorDialogPresentation {
        is_open,
        on_open_change: actions.on_dialog_open_change,
        title,
        cells,
        toggle_checked: actions.toggle_checked,
        on_toggle: actions.on_toggle,
        on_apply: actions.on_apply,
        picker_open,
        picker_title,
        picker_rows,
        picker_allow_conflict_pick,
        on_pick: actions.on_pick,
        on_picker_close: actions.on_picker_close,
    }
}
