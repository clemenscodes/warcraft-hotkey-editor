use super::components::layout_editor_content::components::layout_grid::components::layout_tile::LayoutTileView;
use dioxus::prelude::*;

/// The layout editor's scroll region data: the grid cells, the move-hotkey toggle
/// state and handler, and the apply action, all threaded to the centered content.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutEditorBodyProps {
    pub cells: Vec<LayoutTileView>,
    pub toggle_checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
    pub on_apply: EventHandler<MouseEvent>,
}
