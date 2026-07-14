use crate::components::app::components::shell::components::resolve_page::presentation::MiniGridPlacement;

#[derive(Clone, PartialEq)]
pub struct MiniGridView {
    pub placements: Vec<MiniGridPlacement>,
}

impl ddd::View for MiniGridView {}
