use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// One carrier of an ability in the carriers dialog: an icon, name, and id that
/// deep-link into the editor focused on that unit.
#[derive(Props, Clone, PartialEq)]
pub struct CarrierCardProps {
    pub unit_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
    pub view_navigation: ViewNavigationContext,
}
