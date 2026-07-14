use super::view::IslandDetailView;
use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;
use dioxus::prelude::*;

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
