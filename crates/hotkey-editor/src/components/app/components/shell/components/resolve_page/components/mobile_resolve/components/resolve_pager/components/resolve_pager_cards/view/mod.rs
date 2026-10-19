use crate::components::app::components::shell::components::resolve_page::presentation::{
    MoveSection, UnresolvedView,
};

#[derive(Clone, PartialEq)]
pub struct ResolvePagerCardsView {
    pub section: Option<MoveSection>,
    pub unresolved: Vec<UnresolvedView>,
}

impl ddd::Layered for ResolvePagerCardsView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ResolvePagerCardsView {}
