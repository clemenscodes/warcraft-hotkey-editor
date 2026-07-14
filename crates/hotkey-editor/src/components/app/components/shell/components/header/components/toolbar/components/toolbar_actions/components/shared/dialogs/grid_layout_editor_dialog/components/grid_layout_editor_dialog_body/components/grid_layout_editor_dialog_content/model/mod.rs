use super::components::layout_grid::components::layout_tile::LayoutTileView;
use super::view::GridLayoutEditorDialogContentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridLayoutEditorDialogContentModel {
    pub cells: Vec<LayoutTileView>,
    pub toggle_checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
    pub on_apply: EventHandler<MouseEvent>,
}

impl From<&GridLayoutEditorDialogContentView> for GridLayoutEditorDialogContentModel {
    fn from(view: &GridLayoutEditorDialogContentView) -> Self {
        let GridLayoutEditorDialogContentView {
            cells,
            toggle_checked,
            on_toggle,
            on_apply,
        } = view.clone();
        Self {
            cells,
            toggle_checked,
            on_toggle,
            on_apply,
        }
    }
}

impl ddd::Model for GridLayoutEditorDialogContentModel {
    type View = GridLayoutEditorDialogContentView;
}
