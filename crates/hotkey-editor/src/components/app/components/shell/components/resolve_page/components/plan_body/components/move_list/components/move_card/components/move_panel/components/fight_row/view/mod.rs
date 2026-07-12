use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;

/// The published `View` contract mirroring [`FightRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FightRowView {
    pub move_view: MoveView,
}

impl ddd::View for FightRowView {}
