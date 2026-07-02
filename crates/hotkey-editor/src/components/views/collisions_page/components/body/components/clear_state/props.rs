use dioxus::prelude::*;

/// The "all clear" state for a collision kind with a file loaded but no conflicts.
#[derive(Props, Clone, PartialEq)]
pub struct ClearStateProps {
    pub collision_kind: &'static str,
}
