use super::components::grid_layout_editor_dialog_body::components::grid_layout_editor_dialog_content::components::layout_grid::components::layout_tile::LayoutTileView;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`GridLayoutEditorDialogPanelModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct GridLayoutEditorDialogPanelView {
    pub title: String,
    pub on_close: EventHandler<()>,
    pub cells: Vec<LayoutTileView>,
    pub toggle_checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
    pub on_apply: EventHandler<MouseEvent>,
}

impl ddd::View for GridLayoutEditorDialogPanelView {}
