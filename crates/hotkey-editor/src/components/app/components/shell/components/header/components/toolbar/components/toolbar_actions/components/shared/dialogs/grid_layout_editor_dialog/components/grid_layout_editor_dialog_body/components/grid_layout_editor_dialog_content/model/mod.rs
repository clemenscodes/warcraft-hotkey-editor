use super::components::layout_workspace::components::layout_grid::components::layout_tile::LayoutTileView;
use super::view::GridLayoutEditorDialogContentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridLayoutEditorDialogContentModel {
    pub cells: Vec<LayoutTileView>,
    pub toggle_checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
}

impl From<&GridLayoutEditorDialogContentView> for GridLayoutEditorDialogContentModel {
    fn from(view: &GridLayoutEditorDialogContentView) -> Self {
        let GridLayoutEditorDialogContentView {
            cells,
            toggle_checked,
            on_toggle,
        } = view.clone();
        Self {
            cells,
            toggle_checked,
            on_toggle,
        }
    }
}

impl ddd::Layered for GridLayoutEditorDialogContentModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for GridLayoutEditorDialogContentModel {
    type View = GridLayoutEditorDialogContentView;
}
