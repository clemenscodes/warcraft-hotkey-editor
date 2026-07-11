use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;

/// The published `View` contract mirroring [`FightRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FightRowView {
    pub unresolved_view: UnresolvedView,
}

impl ddd::View for FightRowView {}
