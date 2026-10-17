use crate::components::app::components::shell::components::resolve_page::presentation::MiniGridPlacement;

#[derive(Clone, PartialEq)]
pub struct MiniGridView {
    pub placements: Vec<MiniGridPlacement>,
}

impl ddd::Layered for MiniGridView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MiniGridView {}
