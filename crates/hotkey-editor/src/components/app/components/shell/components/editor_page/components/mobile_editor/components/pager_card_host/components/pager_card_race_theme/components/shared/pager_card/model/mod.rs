use super::view::PagerCardView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct PagerCardModel {
    pub unit_id: WarcraftObjectId,
}

impl From<&PagerCardView> for PagerCardModel {
    fn from(view: &PagerCardView) -> Self {
        let PagerCardView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Model for PagerCardModel {
    type View = PagerCardView;
}
