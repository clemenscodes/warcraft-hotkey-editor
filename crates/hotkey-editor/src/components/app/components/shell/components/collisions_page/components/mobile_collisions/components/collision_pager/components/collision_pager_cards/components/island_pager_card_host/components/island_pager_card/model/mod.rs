use super::view::IslandPagerCardView;
use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IslandPagerCardModel {
    pub island: IslandView,
}

impl From<&IslandPagerCardView> for IslandPagerCardModel {
    fn from(view: &IslandPagerCardView) -> Self {
        let IslandPagerCardView { island } = view.clone();
        Self { island }
    }
}

impl ddd::Model for IslandPagerCardModel {
    type View = IslandPagerCardView;
}
