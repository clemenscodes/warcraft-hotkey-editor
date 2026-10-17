use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;

#[derive(Clone, PartialEq)]
pub struct UnresolvedPagerCardHostView {
    pub unresolved_view: UnresolvedView,
}

impl ddd::Layered for UnresolvedPagerCardHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnresolvedPagerCardHostView {}
