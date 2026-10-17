use crate::components::app::components::shell::components::collisions_page::presentation::ConflictView;

#[derive(Clone, PartialEq)]
pub struct IslandConflictGridView {
    pub conflicts: Vec<ConflictView>,
}

impl ddd::Layered for IslandConflictGridView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IslandConflictGridView {}
