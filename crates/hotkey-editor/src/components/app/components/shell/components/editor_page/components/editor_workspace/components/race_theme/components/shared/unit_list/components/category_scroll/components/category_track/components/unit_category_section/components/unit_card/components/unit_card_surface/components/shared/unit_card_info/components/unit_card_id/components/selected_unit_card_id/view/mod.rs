use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`SelectedUnitCardIdModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SelectedUnitCardIdView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for SelectedUnitCardIdView {}
