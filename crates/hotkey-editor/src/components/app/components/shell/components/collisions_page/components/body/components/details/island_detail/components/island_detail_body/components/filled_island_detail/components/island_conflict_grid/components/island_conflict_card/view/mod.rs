use crate::components::app::components::shell::components::collisions_page::presentation::ConflictView;

#[derive(Clone, PartialEq)]
pub struct IslandConflictCardView {
    pub conflict: ConflictView,
}

impl ddd::Layered for IslandConflictCardView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IslandConflictCardView {}
