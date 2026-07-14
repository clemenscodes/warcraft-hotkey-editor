use crate::services::carriers::InspectedAbility;
use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct IslandConflictAbilityView {
    pub ability_name: String,
    pub ability_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    pub extra_count: usize,
    pub inspected: InspectedAbility,
}

impl ddd::View for IslandConflictAbilityView {}
