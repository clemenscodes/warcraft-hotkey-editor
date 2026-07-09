use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use dioxus::prelude::*;

/// The collision card's selectable button surface's input: the selected flag the
/// dispatcher reads to pick the look, the click handler, the live count, and the content
/// that fills it (a unit portrait with name/id, or an island's coordinate).
#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardSurfaceProps {
    pub is_selected: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub count: usize,
    pub content: CollisionCardContent,
}
