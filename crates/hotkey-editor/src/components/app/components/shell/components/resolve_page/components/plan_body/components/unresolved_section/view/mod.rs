use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;

/// The published `View` contract mirroring [`UnresolvedSectionModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnresolvedSectionView {
    pub unresolved: Vec<UnresolvedView>,
}

impl ddd::View for UnresolvedSectionView {}
