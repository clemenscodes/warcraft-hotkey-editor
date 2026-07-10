use super::view::IslandConflictCardView;
use crate::components::app::components::shell::components::collisions_page::logic::ConflictView;
use dioxus::prelude::*;

/// One island conflict: the affected unit and its two clashing abilities. The affected
/// unit and each ability read the navigation from context to deep-link, and each
/// ability owns its own carriers dialog, so nothing is threaded through this card.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictCardProps {
    pub conflict: ConflictView,
}

impl From<&IslandConflictCardView> for IslandConflictCardProps {
    fn from(view: &IslandConflictCardView) -> Self {
        let IslandConflictCardView { conflict } = view.clone();
        Self { conflict }
    }
}

impl ddd::Props for IslandConflictCardProps {
    type View = IslandConflictCardView;
}
