use crate::components::app::components::shell::components::resolve_page::logic::UnresolvedView;

/// The published `View` contract mirroring [`UnresolvedMoveListProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnresolvedMoveListView {
    pub unresolved: Vec<UnresolvedView>,
}

impl ddd::View for UnresolvedMoveListView {}
