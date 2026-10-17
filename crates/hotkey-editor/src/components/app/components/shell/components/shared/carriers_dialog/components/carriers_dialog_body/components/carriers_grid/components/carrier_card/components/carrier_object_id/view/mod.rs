use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct CarrierObjectIdView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for CarrierObjectIdView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CarrierObjectIdView {}
