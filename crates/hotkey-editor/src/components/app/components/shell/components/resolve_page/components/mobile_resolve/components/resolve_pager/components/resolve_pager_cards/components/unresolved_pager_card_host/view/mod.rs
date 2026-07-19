use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;

#[derive(Clone, PartialEq)]
pub struct UnresolvedPagerCardHostView {
    pub unresolved_view: UnresolvedView,
}

impl ddd::View for UnresolvedPagerCardHostView {}
