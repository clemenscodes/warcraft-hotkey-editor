use super::view::ConflictAbilityView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct ConflictAbilityModel {
    #[props(into)]
    pub ability_name: String,
    pub ability_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    pub unit_id: WarcraftObjectId,
}

impl From<&ConflictAbilityView> for ConflictAbilityModel {
    fn from(view: &ConflictAbilityView) -> Self {
        let ConflictAbilityView {
            ability_name,
            ability_id,
            icon_url,
            unit_id,
        } = view.clone();
        Self {
            ability_name,
            ability_id,
            icon_url,
            unit_id,
        }
    }
}

impl ddd::Model for ConflictAbilityModel {
    type View = ConflictAbilityView;
}
