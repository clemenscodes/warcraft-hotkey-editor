use super::view::IslandSidebarView;
use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IslandSidebarModel {
    pub islands: Vec<IslandView>,
}

impl From<&IslandSidebarView> for IslandSidebarModel {
    fn from(view: &IslandSidebarView) -> Self {
        let IslandSidebarView { islands } = view.clone();
        Self { islands }
    }
}

impl ddd::Model for IslandSidebarModel {
    type View = IslandSidebarView;
}
