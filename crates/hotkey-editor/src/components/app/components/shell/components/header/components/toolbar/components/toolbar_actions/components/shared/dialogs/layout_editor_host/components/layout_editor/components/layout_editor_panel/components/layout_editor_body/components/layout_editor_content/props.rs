use super::components::layout_grid::components::layout_tile::LayoutTileView;
use super::view::LayoutEditorContentView;
use dioxus::prelude::*;

/// The centered column's data: the editable grid cells, the move-hotkey toggle
/// state and handler, and the apply action. The intro block takes no data.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutEditorContentProps {
    pub cells: Vec<LayoutTileView>,
    pub toggle_checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
    pub on_apply: EventHandler<MouseEvent>,
}

impl From<&LayoutEditorContentView> for LayoutEditorContentProps {
    fn from(view: &LayoutEditorContentView) -> Self {
        let LayoutEditorContentView {
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

impl ddd::Props for LayoutEditorContentProps {
    type View = LayoutEditorContentView;
}
