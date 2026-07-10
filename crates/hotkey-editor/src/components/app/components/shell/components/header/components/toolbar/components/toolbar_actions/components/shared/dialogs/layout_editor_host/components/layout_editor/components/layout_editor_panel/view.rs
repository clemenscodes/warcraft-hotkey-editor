use super::components::layout_editor_body::components::layout_editor_content::components::layout_grid::components::layout_tile::LayoutTileView;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`LayoutEditorPanelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct LayoutEditorPanelView {
    pub title: String,
    pub on_close: EventHandler<()>,
    pub cells: Vec<LayoutTileView>,
    pub toggle_checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
    pub on_apply: EventHandler<MouseEvent>,
}

impl ddd::View for LayoutEditorPanelView {}
