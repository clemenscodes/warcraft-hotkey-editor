use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::components::collision_card_surface::components::shared::collision_card_meta::CollisionCardMetaProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::components::collision_card_surface::components::shared::collision_card_visual::CollisionCardVisualProps;
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

impl From<&SelectedCollisionCardSurfaceProps> for CollisionCardVisualProps {
    fn from(props: &SelectedCollisionCardSurfaceProps) -> Self {
        let content = props.content.clone();
        Self { content }
    }
}

impl From<&SelectedCollisionCardSurfaceProps> for CollisionCardMetaProps {
    fn from(props: &SelectedCollisionCardSurfaceProps) -> Self {
        let content = props.content.clone();
        let count = props.count;
        Self { content, count }
    }
}
