use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// One ability of a hotkey conflict: an icon button that deep-links into the editor
/// focused on the owning unit, with the ability name and id below.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictAbilityProps {
    #[props(into)]
    pub ability_name: String,
    #[props(into)]
    pub ability_id: String,
    pub icon_url: Option<String>,
    #[props(into)]
    pub unit_id: String,
    pub view_navigation: ViewNavigationContext,
}
