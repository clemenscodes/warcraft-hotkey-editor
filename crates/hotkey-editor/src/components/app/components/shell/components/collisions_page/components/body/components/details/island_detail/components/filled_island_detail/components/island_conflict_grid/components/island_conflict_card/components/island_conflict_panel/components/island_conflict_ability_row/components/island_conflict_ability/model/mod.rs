use super::view::IslandConflictAbilityView;
use crate::services::carriers::InspectedAbility;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// One ability of an island conflict: a big icon and name that open the carriers
/// dialog, plus an optional "+N more" link when the ability is carried by more units
/// than the one shown. `inspected` is the opaque identity this ability opens the dialog
/// on — a name and a carrier-id list, resolved to views only by the dialog's host.
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
