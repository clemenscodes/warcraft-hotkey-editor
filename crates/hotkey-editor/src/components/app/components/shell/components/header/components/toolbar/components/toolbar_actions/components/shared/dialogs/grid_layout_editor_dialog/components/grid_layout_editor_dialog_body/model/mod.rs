use super::components::grid_layout_editor_dialog_content::components::layout_grid::components::layout_tile::LayoutTileView;
use super::view::GridLayoutEditorDialogBodyView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridLayoutEditorDialogBodyModel {
    pub cells: Vec<LayoutTileView>,
    pub toggle_checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
    pub on_apply: EventHandler<MouseEvent>,
}

impl From<&GridLayoutEditorDialogBodyView> for GridLayoutEditorDialogBodyModel {
    fn from(view: &GridLayoutEditorDialogBodyView) -> Self {
        let GridLayoutEditorDialogBodyView {
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

impl ddd::Model for GridLayoutEditorDialogBodyModel {
    type View = GridLayoutEditorDialogBodyView;
}
