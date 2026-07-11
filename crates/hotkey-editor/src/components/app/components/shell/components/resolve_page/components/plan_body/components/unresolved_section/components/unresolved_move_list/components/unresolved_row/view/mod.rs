use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;

/// The published `View` contract mirroring [`UnresolvedRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnresolvedRowView {
    pub unresolved_view: UnresolvedView,
}

impl ddd::View for UnresolvedRowView {}
