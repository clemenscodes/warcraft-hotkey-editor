use super::view::GridLayoutEditorDialogPanelView;
use super::components::grid_layout_editor_dialog_body::components::grid_layout_editor_dialog_content::components::layout_grid::components::layout_tile::LayoutTileView;
use dioxus::prelude::*;

/// The layout editor's bordered box data: the header title and close handler above
/// the scrolling body's grid cells, move-hotkey toggle, and apply action.
#[derive(Props, Clone, PartialEq)]
pub struct GridLayoutEditorDialogPanelModel {
    #[props(into)]
    pub title: String,
    pub on_close: EventHandler<()>,
    pub cells: Vec<LayoutTileView>,
    pub toggle_checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
    pub on_apply: EventHandler<MouseEvent>,
}

impl From<&GridLayoutEditorDialogPanelView> for GridLayoutEditorDialogPanelModel {
    fn from(view: &GridLayoutEditorDialogPanelView) -> Self {
        let GridLayoutEditorDialogPanelView {
            title,
            on_close,
            cells,
            toggle_checked,
            on_toggle,
            on_apply,
        } = view.clone();
        Self {
            title,
            on_close,
            cells,
            toggle_checked,
            on_toggle,
            on_apply,
        }
    }
}

impl ddd::Model for GridLayoutEditorDialogPanelModel {
    type View = GridLayoutEditorDialogPanelView;
}
