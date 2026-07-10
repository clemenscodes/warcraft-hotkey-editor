use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`NormalUnitCardIdProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct NormalUnitCardIdView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for NormalUnitCardIdView {}
