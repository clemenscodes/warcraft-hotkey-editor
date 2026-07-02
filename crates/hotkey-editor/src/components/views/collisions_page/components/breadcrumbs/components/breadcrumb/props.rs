use crate::services::navigation::app_view::CollisionKind;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// A single breadcrumb tab: its label, live count, the kind it navigates to, its
/// e2e marker, whether it is the active tab, and the navigation context.
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub target_kind: CollisionKind,
    pub data_breadcrumb: &'static str,
    pub active: bool,
    pub view_navigation: ViewNavigationContext,
}
