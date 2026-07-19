use super::view::MovePagerCardHostView;
use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MovePagerCardHostModel {
    pub move_view: MoveView,
}

impl From<&MovePagerCardHostView> for MovePagerCardHostModel {
    fn from(view: &MovePagerCardHostView) -> Self {
        let MovePagerCardHostView { move_view } = view.clone();
        Self { move_view }
    }
}

impl ddd::Model for MovePagerCardHostModel {
    type View = MovePagerCardHostView;
}
