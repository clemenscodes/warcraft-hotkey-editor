use super::view::IslandDetailView;
use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;
use dioxus::prelude::*;

/// The island (position-collision) detail pane: the collision islands. The selected
/// island and the navigation its links use are read from context, so only the island
/// list is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct IslandDetailModel {
    pub islands: Vec<IslandView>,
}

impl From<&IslandDetailView> for IslandDetailModel {
    fn from(view: &IslandDetailView) -> Self {
        let IslandDetailView { islands } = view.clone();
        Self { islands }
    }
}

impl ddd::Model for IslandDetailModel {
    type View = IslandDetailView;
}
