use super::view::IslandPagerCardHostView;
use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IslandPagerCardHostModel {
    pub island: IslandView,
}

impl From<&IslandPagerCardHostView> for IslandPagerCardHostModel {
    fn from(view: &IslandPagerCardHostView) -> Self {
        let IslandPagerCardHostView { island } = view.clone();
        Self { island }
    }
}

impl ddd::Model for IslandPagerCardHostModel {
    type View = IslandPagerCardHostView;
}
