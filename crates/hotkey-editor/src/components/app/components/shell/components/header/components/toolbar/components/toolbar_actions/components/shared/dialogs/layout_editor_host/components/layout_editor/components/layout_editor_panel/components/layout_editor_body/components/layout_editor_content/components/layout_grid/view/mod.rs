use super::components::layout_tile::LayoutTileView;

/// The published `View` contract mirroring [`LayoutGridModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct LayoutGridView {
    pub cells: Vec<LayoutTileView>,
}

impl ddd::View for LayoutGridView {}
