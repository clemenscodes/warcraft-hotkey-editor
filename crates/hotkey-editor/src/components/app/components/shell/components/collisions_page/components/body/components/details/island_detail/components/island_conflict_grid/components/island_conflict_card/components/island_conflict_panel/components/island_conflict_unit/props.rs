use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The affected unit heading an island conflict card: a big icon, name, and object
/// id that deep-link into the editor focused on that unit.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictUnitProps {
    pub unit_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
    pub view_navigation: ViewNavigationContext,
}
