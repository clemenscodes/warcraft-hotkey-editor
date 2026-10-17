use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;

#[derive(Clone, PartialEq)]
pub struct UnresolvedSectionView {
    pub unresolved: Vec<UnresolvedView>,
}

impl ddd::Layered for UnresolvedSectionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnresolvedSectionView {}
