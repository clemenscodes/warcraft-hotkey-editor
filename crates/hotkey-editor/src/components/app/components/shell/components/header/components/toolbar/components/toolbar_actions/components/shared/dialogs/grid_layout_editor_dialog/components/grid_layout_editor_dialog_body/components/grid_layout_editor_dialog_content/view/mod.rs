use super::components::layout_grid::components::layout_tile::LayoutTileView;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct GridLayoutEditorDialogContentView {
    pub cells: Vec<LayoutTileView>,
    pub toggle_checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
    pub on_apply: EventHandler<MouseEvent>,
}

impl ddd::Layered for GridLayoutEditorDialogContentView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for GridLayoutEditorDialogContentView {}
