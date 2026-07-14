use super::view::MoveCardView;
use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MoveCardModel {
    pub move_view: MoveView,
}

impl From<&MoveCardView> for MoveCardModel {
    fn from(view: &MoveCardView) -> Self {
        let MoveCardView { move_view } = view.clone();
        Self { move_view }
    }
}

impl ddd::Model for MoveCardModel {
    type View = MoveCardView;
}
