use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;
#[derive(Props, Clone, PartialEq)]
pub struct CarrierObjectIdProps {
    pub unit_id: WarcraftObjectId,
}
