use super::view::CarrierObjectIdView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;
#[derive(Props, Clone, PartialEq)]
pub struct CarrierObjectIdModel {
    pub unit_id: WarcraftObjectId,
}

impl From<&CarrierObjectIdView> for CarrierObjectIdModel {
    fn from(view: &CarrierObjectIdView) -> Self {
        let CarrierObjectIdView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Model for CarrierObjectIdModel {
    type View = CarrierObjectIdView;
}
