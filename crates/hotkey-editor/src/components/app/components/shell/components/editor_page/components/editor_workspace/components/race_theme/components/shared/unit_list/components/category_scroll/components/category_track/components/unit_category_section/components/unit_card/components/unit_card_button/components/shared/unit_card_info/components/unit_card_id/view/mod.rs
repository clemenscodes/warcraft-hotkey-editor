use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`UnitCardIdModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitCardIdView {
    pub unit_id: WarcraftObjectId,
    pub is_selected: bool,
}

impl ddd::View for UnitCardIdView {}
