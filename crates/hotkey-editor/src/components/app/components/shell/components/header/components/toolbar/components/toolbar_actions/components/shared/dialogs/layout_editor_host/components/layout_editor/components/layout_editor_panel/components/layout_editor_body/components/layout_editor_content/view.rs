use super::components::layout_grid::components::layout_tile::LayoutTileView;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`LayoutEditorContentProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct LayoutEditorContentView {
    pub cells: Vec<LayoutTileView>,
    pub toggle_checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
    pub on_apply: EventHandler<MouseEvent>,
}

impl ddd::View for LayoutEditorContentView {}
