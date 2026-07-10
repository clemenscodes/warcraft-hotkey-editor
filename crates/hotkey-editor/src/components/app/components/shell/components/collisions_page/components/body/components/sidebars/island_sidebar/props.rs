use super::view::IslandSidebarView;
use crate::components::app::components::shell::components::collisions_page::logic::IslandView;
use dioxus::prelude::*;

/// The island sidebar: the collision islands. The selected key it reads and writes is
/// read from collision-selection context, so only the island list is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct IslandSidebarProps {
    pub islands: Vec<IslandView>,
}

impl From<&IslandSidebarView> for IslandSidebarProps {
    fn from(view: &IslandSidebarView) -> Self {
        let IslandSidebarView { islands } = view.clone();
        Self { islands }
    }
}

impl ddd::Props for IslandSidebarProps {
    type View = IslandSidebarView;
}
