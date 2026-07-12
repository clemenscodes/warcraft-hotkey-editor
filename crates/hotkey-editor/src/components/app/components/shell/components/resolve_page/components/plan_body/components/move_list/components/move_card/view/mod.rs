use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;

/// The published `View` contract mirroring [`MoveCardModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MoveCardView {
    pub move_view: MoveView,
}

impl ddd::View for MoveCardView {}
