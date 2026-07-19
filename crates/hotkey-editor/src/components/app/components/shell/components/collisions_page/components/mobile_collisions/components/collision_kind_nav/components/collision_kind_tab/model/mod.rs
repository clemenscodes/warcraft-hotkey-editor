use super::view::CollisionKindTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionKindTabModel {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&CollisionKindTabView> for CollisionKindTabModel {
    fn from(view: &CollisionKindTabView) -> Self {
        let CollisionKindTabView {
            label,
            count,
            active,
            onclick,
        } = view.clone();
        Self {
            label,
            count,
            active,
            onclick,
        }
    }
}

impl ddd::Model for CollisionKindTabModel {
    type View = CollisionKindTabView;
}
