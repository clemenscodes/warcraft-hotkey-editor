use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// One ability of a hotkey conflict: an icon button that deep-links into the editor
/// focused on the owning unit, with the ability name and id below. The navigation used
/// to open the unit is read from context, so it is not a prop.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictAbilityProps {
    #[props(into)]
    pub ability_name: String,
    pub ability_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    pub unit_id: WarcraftObjectId,
}
