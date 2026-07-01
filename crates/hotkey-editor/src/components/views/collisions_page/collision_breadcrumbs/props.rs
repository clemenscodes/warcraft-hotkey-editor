use crate::services::navigation::app_view::CollisionKind;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// The breadcrumb bar: the active kind, each kind's live collision count, and the
/// navigation context the tabs use.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionBreadcrumbsProps {
    pub kind: CollisionKind,
    pub position_count: usize,
    pub unit_position_count: usize,
    pub hotkey_count: usize,
    pub view_navigation: ViewNavigationContext,
}
