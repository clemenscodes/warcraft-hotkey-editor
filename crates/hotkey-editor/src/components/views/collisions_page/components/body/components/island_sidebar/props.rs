use crate::components::views::collisions_page::logic::IslandView;
use dioxus::prelude::*;

/// The island sidebar: the collision islands and the selected key it drives.
#[derive(Props, Clone, PartialEq)]
pub struct IslandSidebarProps {
    pub islands: Vec<IslandView>,
    pub selected_island: Signal<Option<String>>,
}
