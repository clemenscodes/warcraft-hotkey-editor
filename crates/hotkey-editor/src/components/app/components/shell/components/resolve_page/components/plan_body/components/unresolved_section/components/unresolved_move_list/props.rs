use super::view::UnresolvedMoveListView;
use crate::components::app::components::shell::components::resolve_page::logic::UnresolvedView;
use dioxus::prelude::*;

/// The unresolved section's grid of stuck-ability cards.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedMoveListProps {
    pub unresolved: Vec<UnresolvedView>,
}

impl From<&UnresolvedMoveListView> for UnresolvedMoveListProps {
    fn from(view: &UnresolvedMoveListView) -> Self {
        let UnresolvedMoveListView { unresolved } = view.clone();
        Self { unresolved }
    }
}

impl ddd::Props for UnresolvedMoveListProps {
    type View = UnresolvedMoveListView;
}
