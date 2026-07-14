use super::view::IdleCollisionCardButtonView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IdleCollisionCardButtonModel {
    pub onclick: EventHandler<MouseEvent>,
    pub count: usize,
    pub content: CollisionCardContent,
}

impl From<&IdleCollisionCardButtonView> for IdleCollisionCardButtonModel {
    fn from(view: &IdleCollisionCardButtonView) -> Self {
        let IdleCollisionCardButtonView {
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

impl ddd::Model for IdleCollisionCardButtonModel {
    type View = IdleCollisionCardButtonView;
}
