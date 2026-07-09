use crate::components::app::components::shell::components::collisions_page::logic::IslandView;
use dioxus::prelude::*;

/// The island (position-collision) detail pane: the collision islands and the selected
/// one. Its links read the navigation from context, so no navigation is threaded here.
#[derive(Props, Clone, PartialEq)]
pub struct IslandDetailProps {
    pub islands: Vec<IslandView>,
    pub selected_island: Signal<Option<String>>,
}
