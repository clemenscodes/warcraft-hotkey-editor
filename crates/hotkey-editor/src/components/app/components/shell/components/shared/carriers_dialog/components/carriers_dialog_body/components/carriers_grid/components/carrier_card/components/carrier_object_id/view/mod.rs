use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct CarrierObjectIdView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for CarrierObjectIdView {}
