use crate::components::app::components::shell::components::resolve_page::presentation::{
    MoveSection, UnresolvedView,
};

#[derive(Clone, PartialEq)]
pub struct ResolvePagerView {
    pub section: Option<MoveSection>,
    pub unresolved: Vec<UnresolvedView>,
}

impl ddd::Layered for ResolvePagerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ResolvePagerView {}
