use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// The affected unit heading an island conflict card: a big icon, name, and object
/// id that deep-link into the editor focused on that unit.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictUnitProps {
    #[props(into)]
    pub unit_id: String,
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
    pub view_navigation: ViewNavigationContext,
}
