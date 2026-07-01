use crate::components::views::collisions_page::IslandView;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// The island (position-collision) detail pane: the collision islands, the selected
/// one, and the navigation context its links use.
#[derive(Props, Clone, PartialEq)]
pub struct IslandDetailProps {
    pub islands: Vec<IslandView>,
    pub selected_island: Signal<Option<String>>,
    pub view_navigation: ViewNavigationContext,
}
