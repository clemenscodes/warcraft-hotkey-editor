use crate::components::app::components::shell::components::resolve_page::logic::UnresolvedView;

/// The published `View` contract mirroring [`FightRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FightRowView {
    pub unresolved_view: UnresolvedView,
}

impl ddd::View for FightRowView {}
