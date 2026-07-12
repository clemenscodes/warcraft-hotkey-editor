use super::GridLayoutEditorDialogBody;
use super::components::grid_layout_editor_dialog_content::components::layout_grid::components::layout_tile::LayoutTileView;
use super::model::GridLayoutEditorDialogBodyModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`GridLayoutEditorDialogBodyModel`], threaded to
/// this component as data. It is also the layout editor's body region: it `impl Render` and
/// renders the presentational `GridLayoutEditorDialogBody` once, so the dialog places the
/// published `View` directly as `WarcraftDialog`'s body, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct GridLayoutEditorDialogBodyView {
    pub cells: Vec<LayoutTileView>,
    pub toggle_checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
    pub on_apply: EventHandler<MouseEvent>,
}

impl ddd::View for GridLayoutEditorDialogBodyView {}

impl Render for GridLayoutEditorDialogBodyView {
    type Model = GridLayoutEditorDialogBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let cells = self.cells.clone();
        let toggle_checked = self.toggle_checked;
        let on_toggle = self.on_toggle;
        let on_apply = self.on_apply;
        rsx! {
            GridLayoutEditorDialogBody {
                cells,
                toggle_checked,
                on_toggle,
                on_apply,
            }
        }
    }
}
