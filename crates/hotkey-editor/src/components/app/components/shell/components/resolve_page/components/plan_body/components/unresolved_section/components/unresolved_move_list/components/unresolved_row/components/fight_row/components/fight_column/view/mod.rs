use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;

/// The published `View` contract mirroring [`FightColumnModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FightColumnView {
    pub unresolved_view: UnresolvedView,
}

impl ddd::View for FightColumnView {}
