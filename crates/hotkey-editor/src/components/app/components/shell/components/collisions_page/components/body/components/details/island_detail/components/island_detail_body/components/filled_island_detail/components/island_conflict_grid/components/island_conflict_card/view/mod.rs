use crate::components::app::components::shell::components::collisions_page::presentation::ConflictView;

/// The published `View` contract mirroring [`IslandConflictCardModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IslandConflictCardView {
    pub conflict: ConflictView,
}

impl ddd::View for IslandConflictCardView {}
