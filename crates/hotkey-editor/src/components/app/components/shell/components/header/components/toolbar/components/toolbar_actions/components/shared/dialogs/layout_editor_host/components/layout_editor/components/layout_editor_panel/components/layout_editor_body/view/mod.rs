use super::components::layout_editor_content::components::layout_grid::components::layout_tile::LayoutTileView;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`LayoutEditorBodyModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct LayoutEditorBodyView {
    pub cells: Vec<LayoutTileView>,
    pub toggle_checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
    pub on_apply: EventHandler<MouseEvent>,
}

impl ddd::View for LayoutEditorBodyView {}
