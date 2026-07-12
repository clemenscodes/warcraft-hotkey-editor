use super::view::IslandDetailBodyView;
use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;
use dioxus::prelude::*;

/// The island detail card's body region input: the collision islands. The selected island
/// and the navigation its links use are read from context, so only the island list is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct IslandDetailBodyModel {
    pub islands: Vec<IslandView>,
}

impl From<&IslandDetailBodyView> for IslandDetailBodyModel {
    fn from(view: &IslandDetailBodyView) -> Self {
        let IslandDetailBodyView { islands } = view.clone();
        Self { islands }
    }
}

impl ddd::Model for IslandDetailBodyModel {
    type View = IslandDetailBodyView;
}
