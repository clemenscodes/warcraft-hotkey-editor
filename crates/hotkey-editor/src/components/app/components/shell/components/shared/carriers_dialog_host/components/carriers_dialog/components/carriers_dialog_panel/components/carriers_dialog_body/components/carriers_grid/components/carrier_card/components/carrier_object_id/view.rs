use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`CarrierObjectIdProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CarrierObjectIdView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for CarrierObjectIdView {}
