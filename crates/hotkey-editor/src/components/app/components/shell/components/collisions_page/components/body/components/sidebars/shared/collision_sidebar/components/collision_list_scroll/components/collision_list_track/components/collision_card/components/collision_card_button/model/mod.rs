use super::view::CollisionCardButtonView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardButtonModel {
    pub is_selected: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub count: usize,
    pub content: CollisionCardContent,
}

impl From<&CollisionCardButtonView> for CollisionCardButtonModel {
    fn from(view: &CollisionCardButtonView) -> Self {
        let CollisionCardButtonView {
            is_selected,
            onclick,
            count,
            content,
        } = view.clone();
        Self {
            is_selected,
            onclick,
            count,
            content,
        }
    }
}

impl ddd::Model for CollisionCardButtonModel {
    type View = CollisionCardButtonView;
}
