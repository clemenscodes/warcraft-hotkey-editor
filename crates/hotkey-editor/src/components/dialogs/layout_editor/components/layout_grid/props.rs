use dioxus::prelude::*;

use super::components::layout_cell::LayoutCellProps;

/// The grid's already-shaped cells, built by the layout editor hook.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutGridProps {
    pub cells: Vec<LayoutCellProps>,
}
