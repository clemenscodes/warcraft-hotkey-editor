use crate::components::app::components::shell::components::collisions_page::logic::ConflictView;

/// The published `View` contract mirroring [`IslandConflictGridProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IslandConflictGridView {
    pub conflicts: Vec<ConflictView>,
}

impl ddd::View for IslandConflictGridView {}
