use super::view::ConflictMoreView;
use dioxus::prelude::*;
/// The "+N more" link opening the carriers dialog for an ability carried by more
/// units than the one shown on the card.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictMoreProps {
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ConflictMoreView> for ConflictMoreProps {
    fn from(view: &ConflictMoreView) -> Self {
        let ConflictMoreView { count, onclick } = view.clone();
        Self { count, onclick }
    }
}

impl ddd::Props for ConflictMoreProps {
    type View = ConflictMoreView;
}
