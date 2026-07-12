use super::view::SelectedCollisionCardButtonView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use dioxus::prelude::*;

/// The selected collision card surface's props: the click handler, live count, and content
/// it lays out. Built by the dispatcher from `CollisionCardButtonModel`.
#[derive(Props, Clone, PartialEq)]
pub struct SelectedCollisionCardButtonModel {
    pub onclick: EventHandler<MouseEvent>,
    pub count: usize,
    pub content: CollisionCardContent,
}

impl From<&SelectedCollisionCardButtonView> for SelectedCollisionCardButtonModel {
    fn from(view: &SelectedCollisionCardButtonView) -> Self {
        let SelectedCollisionCardButtonView {
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

impl ddd::Model for SelectedCollisionCardButtonModel {
    type View = SelectedCollisionCardButtonView;
}
