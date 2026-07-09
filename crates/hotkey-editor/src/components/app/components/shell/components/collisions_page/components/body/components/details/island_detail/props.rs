use crate::components::app::components::shell::components::collisions_page::logic::IslandView;
use dioxus::prelude::*;

/// The island (position-collision) detail pane: the collision islands. The selected
/// island and the navigation its links use are read from context, so only the island
/// list is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct IslandDetailProps {
    pub islands: Vec<IslandView>,
}
