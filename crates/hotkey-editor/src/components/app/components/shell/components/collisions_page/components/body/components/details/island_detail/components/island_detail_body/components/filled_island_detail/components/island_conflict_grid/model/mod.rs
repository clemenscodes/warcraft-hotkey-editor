use super::view::IslandConflictGridView;
use crate::components::app::components::shell::components::collisions_page::presentation::ConflictView;
use dioxus::prelude::*;

/// The scrolling grid of conflict cards for the selected island.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictGridModel {
    pub conflicts: Vec<ConflictView>,
}

impl From<&IslandConflictGridView> for IslandConflictGridModel {
    fn from(view: &IslandConflictGridView) -> Self {
        let IslandConflictGridView { conflicts } = view.clone();
        Self { conflicts }
    }
}

impl ddd::Model for IslandConflictGridModel {
    type View = IslandConflictGridView;
}
