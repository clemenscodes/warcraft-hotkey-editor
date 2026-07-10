use super::view::CarrierObjectIdView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;
#[derive(Props, Clone, PartialEq)]
pub struct CarrierObjectIdProps {
    pub unit_id: WarcraftObjectId,
}

impl From<&CarrierObjectIdView> for CarrierObjectIdProps {
    fn from(view: &CarrierObjectIdView) -> Self {
        let CarrierObjectIdView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Props for CarrierObjectIdProps {
    type View = CarrierObjectIdView;
}
