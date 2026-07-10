use crate::components::app::components::shell::components::resolve_page::logic::MoveView;

/// The published `View` contract mirroring [`FightRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FightRowView {
    pub move_view: MoveView,
}

impl ddd::View for FightRowView {}
