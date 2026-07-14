use super::components::layout_tile::LayoutTileView;

#[derive(Clone, PartialEq)]
pub struct LayoutGridView {
    pub cells: Vec<LayoutTileView>,
}

impl ddd::View for LayoutGridView {}
