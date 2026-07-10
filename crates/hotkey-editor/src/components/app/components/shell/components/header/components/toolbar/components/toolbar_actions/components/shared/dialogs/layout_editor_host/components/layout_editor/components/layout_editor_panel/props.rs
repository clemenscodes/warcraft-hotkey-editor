use super::components::layout_editor_body::components::layout_editor_content::components::layout_grid::components::layout_tile::LayoutTileView;
use dioxus::prelude::*;

/// The layout editor's bordered box data: the header title and close handler above
/// the scrolling body's grid cells, move-hotkey toggle, and apply action.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutEditorPanelProps {
    #[props(into)]
    pub title: String,
    pub on_close: EventHandler<()>,
    pub cells: Vec<LayoutTileView>,
    pub toggle_checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
    pub on_apply: EventHandler<MouseEvent>,
}
