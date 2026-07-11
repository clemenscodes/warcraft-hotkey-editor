use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`FightNamePlateModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FightNamePlateView {
    pub name: String,
    pub object_id: WarcraftObjectId,
}

impl ddd::View for FightNamePlateView {}
