use super::view::MoveListView;
use crate::components::app::components::shell::components::resolve_page::logic::MoveSection;
use dioxus::prelude::*;

/// The active category's move section to render, or nothing when the plan has only
/// unresolved abilities and no moves to show.
#[derive(Props, Clone, PartialEq)]
pub struct MoveListProps {
    pub section: Option<MoveSection>,
}

impl From<&MoveListView> for MoveListProps {
    fn from(view: &MoveListView) -> Self {
        let MoveListView { section } = view.clone();
        Self { section }
    }
}

impl ddd::Props for MoveListProps {
    type View = MoveListView;
}
