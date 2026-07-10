use super::components::layout_tile::LayoutTileView;
use dioxus::prelude::*;

/// The grid's already-shaped cells, built by the layout editor hook.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutGridProps {
    pub cells: Vec<LayoutTileView>,
}
