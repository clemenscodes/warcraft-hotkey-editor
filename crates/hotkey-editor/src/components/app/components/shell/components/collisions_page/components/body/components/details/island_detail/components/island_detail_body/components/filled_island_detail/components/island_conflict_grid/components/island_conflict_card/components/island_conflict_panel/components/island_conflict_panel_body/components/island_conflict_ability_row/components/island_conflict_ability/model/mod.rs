use super::view::IslandConflictAbilityView;
use crate::services::carriers::InspectedAbility;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictAbilityModel {
    #[props(into)]
    pub ability_name: String,
    pub ability_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    pub extra_count: usize,
    pub inspected: InspectedAbility,
}

impl From<&IslandConflictAbilityView> for IslandConflictAbilityModel {
    fn from(view: &IslandConflictAbilityView) -> Self {
        let IslandConflictAbilityView {
            ability_name,
            ability_id,
            icon_url,
            extra_count,
            inspected,
        } = view.clone();
        Self {
            ability_name,
            ability_id,
            icon_url,
            extra_count,
            inspected,
        }
    }
}

impl ddd::Model for IslandConflictAbilityModel {
    type View = IslandConflictAbilityView;
}
