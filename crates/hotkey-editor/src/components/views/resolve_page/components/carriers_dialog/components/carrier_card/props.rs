use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// One carrier of an ability in the carriers dialog; deep-links into the editor.
#[derive(Props, Clone, PartialEq)]
pub struct CarrierCardProps {
    #[props(into)]
    pub unit_id: String,
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
    pub view_navigation: ViewNavigationContext,
}
