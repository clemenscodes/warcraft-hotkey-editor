use super::state::CollisionCardContent;
use dioxus::prelude::*;

/// A selectable collision-sidebar card: its selected state, stable collision key,
/// click handler, live collision count, and the content that fills it (a unit
/// portrait with name/id, or an island's highlighted coordinate).
#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardProps {
    pub is_selected: bool,
    #[props(into)]
    pub collision_key: String,
    pub onclick: EventHandler<MouseEvent>,
    pub count: usize,
    pub content: CollisionCardContent,
}
