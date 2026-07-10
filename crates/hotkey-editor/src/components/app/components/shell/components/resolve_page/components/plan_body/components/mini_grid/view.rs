use crate::components::app::components::shell::components::resolve_page::logic::MiniGridPlacement;

/// The published `View` contract mirroring [`MiniGridProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MiniGridView {
    pub placements: Vec<MiniGridPlacement>,
}

impl ddd::View for MiniGridView {}
