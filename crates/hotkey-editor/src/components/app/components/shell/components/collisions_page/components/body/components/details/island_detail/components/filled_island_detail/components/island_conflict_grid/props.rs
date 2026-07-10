use super::view::IslandConflictGridView;
use crate::components::app::components::shell::components::collisions_page::logic::ConflictView;
use dioxus::prelude::*;

/// The scrolling grid of conflict cards for the selected island.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictGridProps {
    pub conflicts: Vec<ConflictView>,
}

impl From<&IslandConflictGridView> for IslandConflictGridProps {
    fn from(view: &IslandConflictGridView) -> Self {
        let IslandConflictGridView { conflicts } = view.clone();
        Self { conflicts }
    }
}

impl ddd::Props for IslandConflictGridProps {
    type View = IslandConflictGridView;
}
