use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`UnitIdModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitIdView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for UnitIdView {}
