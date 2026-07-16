use super::view::PagerCardIdView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct PagerCardIdModel {
    pub unit_id: WarcraftObjectId,
}

impl From<&PagerCardIdView> for PagerCardIdModel {
    fn from(view: &PagerCardIdView) -> Self {
        let PagerCardIdView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Model for PagerCardIdModel {
    type View = PagerCardIdView;
}
