use super::components::layout_tile::LayoutTileView;
use super::view::LayoutGridView;
use dioxus::prelude::*;

/// The grid's already-shaped cells, built by the layout editor hook.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutGridProps {
    pub cells: Vec<LayoutTileView>,
}

impl From<&LayoutGridView> for LayoutGridProps {
    fn from(view: &LayoutGridView) -> Self {
        let LayoutGridView { cells } = view.clone();
        Self { cells }
    }
}

impl ddd::Props for LayoutGridProps {
    type View = LayoutGridView;
}
