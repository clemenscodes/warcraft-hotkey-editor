use crate::components::app::components::shell::components::resolve_page::logic::UnresolvedView;

/// The published `View` contract mirroring [`FightColumnProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FightColumnView {
    pub unresolved_view: UnresolvedView,
}

impl ddd::View for FightColumnView {}
