use super::view::IdleCollisionCardSurfaceView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use dioxus::prelude::*;

/// The idle collision card surface's props: the click handler, live count, and content
/// it lays out. Built by the dispatcher from `CollisionCardSurfaceModel`.
#[derive(Props, Clone, PartialEq)]
pub struct IdleCollisionCardSurfaceModel {
    pub onclick: EventHandler<MouseEvent>,
    pub count: usize,
    pub content: CollisionCardContent,
}

impl From<&IdleCollisionCardSurfaceView> for IdleCollisionCardSurfaceModel {
    fn from(view: &IdleCollisionCardSurfaceView) -> Self {
        let IdleCollisionCardSurfaceView {
            onclick,
            count,
            content,
        } = view.clone();
        Self {
            onclick,
            count,
            content,
        }
    }
}

impl ddd::Model for IdleCollisionCardSurfaceModel {
    type View = IdleCollisionCardSurfaceView;
}
