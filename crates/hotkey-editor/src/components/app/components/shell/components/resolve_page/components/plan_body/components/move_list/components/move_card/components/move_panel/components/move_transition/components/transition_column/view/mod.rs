use crate::components::app::components::shell::components::resolve_page::presentation::MiniGridPlacement;

/// The published `View` contract mirroring [`TransitionColumnModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TransitionColumnView {
    pub placements: Vec<MiniGridPlacement>,
}

impl ddd::View for TransitionColumnView {}
