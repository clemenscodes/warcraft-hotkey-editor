use super::super::layout_editor::LayoutEditorProps;
use crate::services::editor_state::context::use_editor_state;
use crate::services::grid_layout::use_grid_layout;
use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// Shapes the layout editor's props: the grid being edited and the move-hotkey
/// preference from context, the shared open signal the toolbar buttons flip, and the
/// two transient edit/drag cell signals — which are the editor's own scratch state,
/// used nowhere else, so the host owns them locally rather than the shell.
pub(super) fn use_layout_editor_host() -> LayoutEditorProps {
    let grid_layout = use_grid_layout();
    let editor = use_editor_state();
    let update_hotkeys_on_move = editor.update_hotkeys_on_move;
    let overlay = use_overlay_state();
    let open = overlay.layout_dialog_open;
    let editing_layout_cell = use_signal::<Option<GridCoordinate>>(|| None);
    let dragging_layout_cell = use_signal::<Option<GridCoordinate>>(|| None);
    LayoutEditorProps {
        grid_layout,
        editing_layout_cell,
        dragging_layout_cell,
        update_hotkeys_on_move,
        open,
    }
}
