use dioxus::prelude::*;
use warcraft_keybinds::{CustomKeys, GridCoordinate, GridLayout};

/// What the layout editor needs: the grid being edited, the transient edit/drag
/// cells, the loaded keys to rewrite on apply, the move-hotkey preference, and the
/// open signal driving the dialog shell.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutEditorProps {
    pub grid_layout: Signal<GridLayout>,
    pub editing_layout_cell: Signal<Option<GridCoordinate>>,
    pub dragging_layout_cell: Signal<Option<GridCoordinate>>,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub update_hotkeys_on_move: Signal<bool>,
    pub layout_dialog_open: Signal<bool>,
}
