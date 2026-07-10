use super::components::unresolved_row::UnresolvedRowProps;
use dioxus::prelude::*;

/// The unresolved section's grid of stuck-ability cards.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedMoveListProps {
    pub rows: Vec<UnresolvedRowProps>,
}
