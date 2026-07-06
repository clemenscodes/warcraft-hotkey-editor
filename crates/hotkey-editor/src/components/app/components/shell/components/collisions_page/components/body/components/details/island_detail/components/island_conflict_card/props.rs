use crate::components::app::components::shell::components::collisions_page::logic::ConflictView;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// One island conflict: the affected unit, its two clashing abilities, and the
/// navigation context. Each ability owns and opens its own carriers dialog, so no
/// open-signal is threaded through this card.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictCardProps {
    pub conflict: ConflictView,
    pub view_navigation: ViewNavigationContext,
}
