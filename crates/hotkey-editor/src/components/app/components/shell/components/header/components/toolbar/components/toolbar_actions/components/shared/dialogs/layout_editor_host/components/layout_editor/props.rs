use super::view::LayoutEditorView;
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

impl From<&LayoutEditorView> for LayoutEditorProps {
    fn from(view: &LayoutEditorView) -> Self {
        let LayoutEditorView {
            grid_layout,
            editing_layout_tile,
            dragging_layout_tile,
            update_hotkeys_on_move,
            open,
        } = view.clone();
        Self {
            grid_layout,
            editing_layout_tile,
            dragging_layout_tile,
            update_hotkeys_on_move,
            open,
        }
    }
}

impl ddd::Props for LayoutEditorProps {
    type View = LayoutEditorView;
}
