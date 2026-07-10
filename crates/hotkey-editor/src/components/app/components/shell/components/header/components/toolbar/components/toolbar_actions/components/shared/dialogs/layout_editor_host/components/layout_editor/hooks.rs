use super::components::layout_editor_panel::components::layout_editor_body::components::layout_editor_content::components::layout_grid::components::layout_tile::LayoutTileView;
use super::logic::{GridCellContext, LayoutGridCells, LayoutPickerBoard, LayoutPickerContext};
use super::props::LayoutEditorProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::KeyPickerCell;
use crate::components::app::components::shell::components::toasts::{ToastOptions, use_toast};
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::grid_layout::context::use_grid_layout_service;
use dioxus::prelude::*;

use warcraft_keybinds::HotkeyToken;

/// Everything the layout editor's markup needs, already shaped: the dialog open
/// state and guard, the panel header, the grid cells, the key-picker state, the
/// toggle state, and every handler. The body only places these; all the work
/// happens here.
pub(super) struct LayoutEditorModel {
    pub(super) open: Signal<bool>,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: String,
    pub(super) on_close: EventHandler<()>,
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

fn use_layout_actions(props: &LayoutEditorProps) -> LayoutActions {
    let grid_layout = props.grid_layout;
    let mut editing_layout_tile = props.editing_layout_tile;
    let mut update_hotkeys_on_move = props.update_hotkeys_on_move;
    let mut layout_dialog_open = props.open;
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
pub(super) fn use_layout_editor(props: &LayoutEditorProps) -> LayoutEditorModel {
    let open = props.open;
    let grid_layout = props.grid_layout;
    let editing_layout_tile = props.editing_layout_tile;
    let dragging_layout_tile = props.dragging_layout_tile;
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

    let actions = use_layout_actions(props);

    let mut open_signal = props.open;
    let title = String::from("Global Hotkey Layout");
    let on_close = EventHandler::new(move |_event: ()| open_signal.set(false));
    let picker_title = String::from("Pick a grid key");
    let picker_allow_conflict_pick = true;

    LayoutEditorModel {
        open,
        on_open_change: actions.on_dialog_open_change,
        title,
        on_close,
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
