use crate::components::app::components::shell::components::resolve_page::presentation::MiniGridPlacement;

#[derive(Clone, PartialEq)]
pub struct MoveTransitionView {
    pub placements: Vec<MiniGridPlacement>,
}

impl ddd::Layered for MoveTransitionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MoveTransitionView {}
