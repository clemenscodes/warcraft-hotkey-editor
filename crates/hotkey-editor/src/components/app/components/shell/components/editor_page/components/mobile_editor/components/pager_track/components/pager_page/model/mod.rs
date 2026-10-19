use super::view::PagerPageView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct PagerPageModel {
    pub unit_ids: Vec<WarcraftObjectId>,
}

impl From<&PagerPageView> for PagerPageModel {
    fn from(view: &PagerPageView) -> Self {
        let PagerPageView { unit_ids } = view.clone();
        Self { unit_ids }
    }
}

impl ddd::Layered for PagerPageModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for PagerPageModel {
    type View = PagerPageView;
}
