use crate::components::app::components::shell::components::resolve_page::logic::UnresolvedView;

/// The published `View` contract mirroring [`UnresolvedSectionProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnresolvedSectionView {
    pub unresolved: Vec<UnresolvedView>,
}

impl ddd::View for UnresolvedSectionView {}
