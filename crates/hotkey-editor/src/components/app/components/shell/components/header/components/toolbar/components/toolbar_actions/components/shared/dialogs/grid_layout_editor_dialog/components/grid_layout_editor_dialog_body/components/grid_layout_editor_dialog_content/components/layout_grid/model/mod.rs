use super::components::layout_tile::LayoutTileView;
use super::view::LayoutGridView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LayoutGridModel {
    pub cells: Vec<LayoutTileView>,
}

impl From<&LayoutGridView> for LayoutGridModel {
    fn from(view: &LayoutGridView) -> Self {
        let LayoutGridView { cells } = view.clone();
        Self { cells }
    }
}

impl ddd::Model for LayoutGridModel {
    type View = LayoutGridView;
}
