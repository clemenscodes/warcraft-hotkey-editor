use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;

/// The published `View` contract mirroring [`UnresolvedMoveListModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnresolvedMoveListView {
    pub unresolved: Vec<UnresolvedView>,
}

impl ddd::View for UnresolvedMoveListView {}
