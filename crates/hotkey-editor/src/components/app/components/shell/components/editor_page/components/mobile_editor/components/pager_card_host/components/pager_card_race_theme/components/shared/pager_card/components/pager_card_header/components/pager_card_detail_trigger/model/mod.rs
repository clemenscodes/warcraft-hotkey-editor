use super::view::PagerCardDetailTriggerView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct PagerCardDetailTriggerModel {
    pub icon_url: Option<String>,
    pub unit_id: WarcraftObjectId,
}

impl From<&PagerCardDetailTriggerView> for PagerCardDetailTriggerModel {
    fn from(view: &PagerCardDetailTriggerView) -> Self {
        let PagerCardDetailTriggerView { icon_url, unit_id } = view.clone();
        Self { icon_url, unit_id }
    }
}

impl ddd::Model for PagerCardDetailTriggerModel {
    type View = PagerCardDetailTriggerView;
}
