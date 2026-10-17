use crate::components::app::components::shell::components::resolve_page::presentation::MiniGridPlacement;

#[derive(Clone, PartialEq)]
pub struct TransitionColumnView {
    pub placements: Vec<MiniGridPlacement>,
}

impl ddd::Layered for TransitionColumnView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TransitionColumnView {}
