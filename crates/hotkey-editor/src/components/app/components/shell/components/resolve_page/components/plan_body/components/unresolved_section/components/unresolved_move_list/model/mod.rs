use super::view::UnresolvedMoveListView;
use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;
use dioxus::prelude::*;

/// The unresolved section's grid of stuck-ability cards.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedMoveListModel {
    pub unresolved: Vec<UnresolvedView>,
}

impl From<&UnresolvedMoveListView> for UnresolvedMoveListModel {
    fn from(view: &UnresolvedMoveListView) -> Self {
        let UnresolvedMoveListView { unresolved } = view.clone();
        Self { unresolved }
    }
}

impl ddd::Model for UnresolvedMoveListModel {
    type View = UnresolvedMoveListView;
}
