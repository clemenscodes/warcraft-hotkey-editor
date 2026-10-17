use super::view::PagerTrackView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct PagerTrackModel {
    pub unit_ids: Vec<WarcraftObjectId>,
}

impl From<&PagerTrackView> for PagerTrackModel {
    fn from(view: &PagerTrackView) -> Self {
        let PagerTrackView { unit_ids } = view.clone();
        Self { unit_ids }
    }
}

impl ddd::Layered for PagerTrackModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for PagerTrackModel {
    type View = PagerTrackView;
}
