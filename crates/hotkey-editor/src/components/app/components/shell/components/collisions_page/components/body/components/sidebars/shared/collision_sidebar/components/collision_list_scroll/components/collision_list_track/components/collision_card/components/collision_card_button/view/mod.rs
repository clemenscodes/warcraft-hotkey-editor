use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CollisionCardButtonView {
    pub is_selected: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub count: usize,
    pub content: CollisionCardContent,
}

impl ddd::View for CollisionCardButtonView {}
