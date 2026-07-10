use crate::components::app::components::shell::components::resolve_page::logic::UnresolvedView;

/// The published `View` contract mirroring [`UnresolvedRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnresolvedRowView {
    pub unresolved_view: UnresolvedView,
}

impl ddd::View for UnresolvedRowView {}
