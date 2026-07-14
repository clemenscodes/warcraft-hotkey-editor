use super::view::ConflictMoreView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictMoreModel {
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ConflictMoreView> for ConflictMoreModel {
    fn from(view: &ConflictMoreView) -> Self {
        let ConflictMoreView { count, onclick } = view.clone();
        Self { count, onclick }
    }
}

impl ddd::Model for ConflictMoreModel {
    type View = ConflictMoreView;
}
