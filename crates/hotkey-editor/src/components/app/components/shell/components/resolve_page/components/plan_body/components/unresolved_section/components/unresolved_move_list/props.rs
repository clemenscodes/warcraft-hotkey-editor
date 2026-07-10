use super::components::unresolved_row::UnresolvedRowProps;
use dioxus::prelude::*;

/// The unresolved section's grid of stuck-ability cards, plus the category slug tagged on
/// the grid for the e2e suite.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedMoveListProps {
    pub category: &'static str,
    pub rows: Vec<UnresolvedRowProps>,
}
