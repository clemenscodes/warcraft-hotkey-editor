use super::view::PagerCardHostView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct PagerCardHostModel {
    pub unit_id: WarcraftObjectId,
}

impl From<&PagerCardHostView> for PagerCardHostModel {
    fn from(view: &PagerCardHostView) -> Self {
        let PagerCardHostView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Model for PagerCardHostModel {
    type View = PagerCardHostView;
}
