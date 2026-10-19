use super::components::pager_page::PagerPageView;
use super::view::PagerTrackView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PagerTrackModel {
    pub pages: Vec<PagerPageView>,
}

impl From<&PagerTrackView> for PagerTrackModel {
    fn from(view: &PagerTrackView) -> Self {
        let PagerTrackView { pages } = view.clone();
        Self { pages }
    }
}

impl ddd::Layered for PagerTrackModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for PagerTrackModel {
    type View = PagerTrackView;
}
