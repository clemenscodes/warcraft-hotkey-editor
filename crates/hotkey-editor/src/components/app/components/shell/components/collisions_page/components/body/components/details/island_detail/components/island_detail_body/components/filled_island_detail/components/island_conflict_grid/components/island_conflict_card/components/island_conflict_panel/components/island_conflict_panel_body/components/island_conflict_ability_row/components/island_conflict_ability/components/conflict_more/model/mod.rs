use super::view::ConflictMoreView;
use dioxus::prelude::*;
/// The "+N more" link opening the carriers dialog for an ability carried by more
/// units than the one shown on the card.
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
