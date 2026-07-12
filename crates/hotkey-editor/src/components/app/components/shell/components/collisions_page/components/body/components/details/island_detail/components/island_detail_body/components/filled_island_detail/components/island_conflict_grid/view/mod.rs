use crate::components::app::components::shell::components::collisions_page::presentation::ConflictView;

/// The published `View` contract mirroring [`IslandConflictGridModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IslandConflictGridView {
    pub conflicts: Vec<ConflictView>,
}

impl ddd::View for IslandConflictGridView {}
