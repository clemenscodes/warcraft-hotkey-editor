use super::view::SelectedCollisionCardSurfaceView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use dioxus::prelude::*;

/// The selected collision card surface's props: the click handler, live count, and content
/// it lays out. Built by the dispatcher from `CollisionCardSurfaceProps`.
#[derive(Props, Clone, PartialEq)]
pub struct SelectedCollisionCardSurfaceProps {
    pub onclick: EventHandler<MouseEvent>,
    pub count: usize,
    pub content: CollisionCardContent,
}

impl From<&SelectedCollisionCardSurfaceView> for SelectedCollisionCardSurfaceProps {
    fn from(view: &SelectedCollisionCardSurfaceView) -> Self {
        let SelectedCollisionCardSurfaceView {
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

impl ddd::Props for SelectedCollisionCardSurfaceProps {
    type View = SelectedCollisionCardSurfaceView;
}
