use dioxus::prelude::*;

/// The "upload a file" prompt shown for a collision kind before any CustomKeys.txt
/// is loaded.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionsEmptyStateProps {
    pub collision_kind: &'static str,
    #[props(into)]
    pub message: String,
}
