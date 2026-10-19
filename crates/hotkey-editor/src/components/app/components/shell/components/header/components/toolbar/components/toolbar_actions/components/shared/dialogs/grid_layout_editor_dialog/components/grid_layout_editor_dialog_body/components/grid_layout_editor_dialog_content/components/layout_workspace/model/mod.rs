use super::components::layout_grid::components::layout_tile::LayoutTileView;
use super::view::LayoutWorkspaceView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LayoutWorkspaceModel {
    pub cells: Vec<LayoutTileView>,
    pub checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
}

impl From<&LayoutWorkspaceView> for LayoutWorkspaceModel {
    fn from(view: &LayoutWorkspaceView) -> Self {
        let LayoutWorkspaceView {
            cells,
            checked,
            on_toggle,
        } = view.clone();
        Self {
            cells,
            checked,
            on_toggle,
        }
    }
}

impl ddd::Layered for LayoutWorkspaceModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for LayoutWorkspaceModel {
    type View = LayoutWorkspaceView;
}
