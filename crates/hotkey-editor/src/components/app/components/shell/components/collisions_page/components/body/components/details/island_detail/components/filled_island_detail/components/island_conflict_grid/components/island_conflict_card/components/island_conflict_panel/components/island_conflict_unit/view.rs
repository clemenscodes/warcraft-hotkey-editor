use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`IslandConflictUnitProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IslandConflictUnitView {
    pub unit_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    pub name: String,
}

impl ddd::View for IslandConflictUnitView {}
