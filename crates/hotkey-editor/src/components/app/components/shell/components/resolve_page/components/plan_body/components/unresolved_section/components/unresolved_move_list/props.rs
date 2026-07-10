use crate::components::app::components::shell::components::resolve_page::logic::UnresolvedView;
use dioxus::prelude::*;

/// The unresolved section's grid of stuck-ability cards.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedMoveListProps {
    pub unresolved: Vec<UnresolvedView>,
}
