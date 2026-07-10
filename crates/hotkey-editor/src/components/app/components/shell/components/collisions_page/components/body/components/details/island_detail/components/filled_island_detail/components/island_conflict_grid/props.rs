use crate::components::app::components::shell::components::collisions_page::logic::ConflictView;
use dioxus::prelude::*;

/// The scrolling grid of conflict cards for the selected island.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictGridProps {
    pub conflicts: Vec<ConflictView>,
}
