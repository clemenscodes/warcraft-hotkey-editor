use crate::components::views::resolve_page::logic::MiniGridPlacement;
use dioxus::prelude::*;

/// A tiny 4×3 command grid rendering each placed ability's icon into its cell.
#[derive(Props, Clone, PartialEq)]
pub struct MiniGridProps {
    pub placements: Vec<MiniGridPlacement>,
}
