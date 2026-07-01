use dioxus::prelude::*;

/// The two-pane content for a collision kind: the sidebar + detail passed as
/// children, tagged with the kind and count for e2e hooks.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionsContentProps {
    pub collision_kind: &'static str,
    pub count: usize,
    pub children: Element,
}
