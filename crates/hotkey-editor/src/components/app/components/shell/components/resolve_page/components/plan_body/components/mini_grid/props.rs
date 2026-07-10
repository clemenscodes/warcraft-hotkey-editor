use super::view::MiniGridView;
use crate::components::app::components::shell::components::resolve_page::logic::MiniGridPlacement;
use dioxus::prelude::*;

/// A tiny 4×3 command grid rendering each placed ability's icon into its cell.
#[derive(Props, Clone, PartialEq)]
pub struct MiniGridProps {
    pub placements: Vec<MiniGridPlacement>,
}

impl From<&MiniGridView> for MiniGridProps {
    fn from(view: &MiniGridView) -> Self {
        let MiniGridView { placements } = view.clone();
        Self { placements }
    }
}

impl ddd::Props for MiniGridProps {
    type View = MiniGridView;
}
