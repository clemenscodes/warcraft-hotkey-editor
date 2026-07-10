use dioxus::prelude::*;
use warcraft_keybinds::{GridCoordinate, GridLayout};

/// What the layout editor needs: the grid being edited, the transient edit/drag
/// cells, the move-hotkey preference, and the open signal driving the dialog shell.
/// It rewrites the loaded keys on apply through the [`CustomKeysService`](crate::services::customkeys::service::CustomKeysService)
/// context, so it takes no document prop.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutEditorProps {
    pub grid_layout: Signal<GridLayout>,
    pub editing_layout_tile: Signal<Option<GridCoordinate>>,
    pub dragging_layout_tile: Signal<Option<GridCoordinate>>,
    pub update_hotkeys_on_move: Signal<bool>,
    pub open: Signal<bool>,
}
