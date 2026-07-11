use crate::components::app::components::shell::components::resolve_page::presentation::MiniGridPlacement;

/// The published `View` contract mirroring [`MoveTransitionModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MoveTransitionView {
    pub from_placements: Vec<MiniGridPlacement>,
    pub to_placements: Vec<MiniGridPlacement>,
}

impl ddd::View for MoveTransitionView {}
