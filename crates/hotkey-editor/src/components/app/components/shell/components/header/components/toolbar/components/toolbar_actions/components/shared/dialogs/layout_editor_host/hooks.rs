use crate::services::editor_state::context::use_editor_state;
use crate::services::grid_layout::context::use_grid_layout;
use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;
use warcraft_keybinds::{GridCoordinate, GridLayout};

/// The layout editor's inputs: the grid being edited and the move-hotkey preference
/// from context, the shared open signal the toolbar buttons flip, and the two
/// transient edit/drag cell signals — which are the editor's own scratch state, used
/// nowhere else, so the host owns them locally rather than the shell.
pub(super) struct LayoutEditorInputs {
    pub(super) grid_layout: Signal<GridLayout>,
    pub(super) editing_layout_tile: Signal<Option<GridCoordinate>>,
    pub(super) dragging_layout_tile: Signal<Option<GridCoordinate>>,
    pub(super) update_hotkeys_on_move: Signal<bool>,
    pub(super) open: Signal<bool>,
}

/// Shapes the layout editor's inputs from app state, so the host can hand them to the
/// editor as named fields.
pub(super) fn use_layout_editor_host() -> LayoutEditorInputs {
    let grid_layout = use_grid_layout();
    let editor = use_editor_state();
    let update_hotkeys_on_move = editor.update_hotkeys_on_move();
    let overlay = use_overlay_state();
    let open = overlay.layout_dialog_open();
    let editing_layout_tile = use_signal::<Option<GridCoordinate>>(|| None);
    let dragging_layout_tile = use_signal::<Option<GridCoordinate>>(|| None);
    LayoutEditorInputs {
        grid_layout,
        editing_layout_tile,
        dragging_layout_tile,
        update_hotkeys_on_move,
        open,
    }
}
