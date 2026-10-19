use super::view::UnitDetailTriggerView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailTriggerModel {
    pub icon_url: Option<String>,
    pub unit_id: WarcraftObjectId,
}

impl From<&UnitDetailTriggerView> for UnitDetailTriggerModel {
    fn from(view: &UnitDetailTriggerView) -> Self {
        let UnitDetailTriggerView { icon_url, unit_id } = view.clone();
        Self { icon_url, unit_id }
    }
}

impl ddd::Layered for UnitDetailTriggerModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for UnitDetailTriggerModel {
    type View = UnitDetailTriggerView;
}
