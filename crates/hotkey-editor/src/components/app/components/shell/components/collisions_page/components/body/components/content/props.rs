use crate::services::navigation::app_view::CollisionKind;
use dioxus::prelude::*;

/// The two-pane content for a collision kind: the sidebar + detail passed as
/// children, tagged with the kind and count for e2e hooks.
#[derive(Props, Clone, PartialEq)]
pub struct ContentProps {
    pub collision_kind: CollisionKind,
    pub count: usize,
    pub children: Element,
}
