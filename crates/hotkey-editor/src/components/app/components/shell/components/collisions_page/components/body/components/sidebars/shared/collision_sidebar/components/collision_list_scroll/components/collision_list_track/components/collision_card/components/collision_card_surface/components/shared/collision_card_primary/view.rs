use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;

/// The published `View` contract mirroring [`CollisionCardPrimaryProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CollisionCardPrimaryView {
    pub content: CollisionCardContent,
}

impl ddd::View for CollisionCardPrimaryView {}
