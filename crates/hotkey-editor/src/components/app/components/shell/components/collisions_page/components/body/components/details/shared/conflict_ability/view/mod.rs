use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`ConflictAbilityModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictAbilityView {
    pub ability_name: String,
    pub ability_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for ConflictAbilityView {}
