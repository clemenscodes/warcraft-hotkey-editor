use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;

/// The published `View` contract mirroring [`FightColumnModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FightColumnView {
    pub move_view: MoveView,
}

impl ddd::View for FightColumnView {}
