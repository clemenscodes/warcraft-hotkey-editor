use super::components::layout_grid::components::layout_tile::LayoutTileView;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct LayoutWorkspaceView {
    pub cells: Vec<LayoutTileView>,
    pub checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
}

impl ddd::Layered for LayoutWorkspaceView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for LayoutWorkspaceView {}
