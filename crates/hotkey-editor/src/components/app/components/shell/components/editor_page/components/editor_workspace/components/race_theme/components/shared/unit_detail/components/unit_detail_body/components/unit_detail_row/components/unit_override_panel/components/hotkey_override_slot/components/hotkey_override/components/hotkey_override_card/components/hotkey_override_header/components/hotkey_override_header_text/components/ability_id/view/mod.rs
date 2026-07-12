use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`AbilityIdModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AbilityIdView {
    pub object_id: WarcraftObjectId,
}

impl ddd::View for AbilityIdView {}
