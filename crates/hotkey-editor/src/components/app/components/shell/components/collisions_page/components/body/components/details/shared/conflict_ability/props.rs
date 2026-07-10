use super::view::ConflictAbilityView;
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

impl From<&ConflictAbilityView> for ConflictAbilityProps {
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

impl ddd::Props for ConflictAbilityProps {
    type View = ConflictAbilityView;
}
