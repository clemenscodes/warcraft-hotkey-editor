use super::view::MovePanelView;
use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MovePanelModel {
    pub move_view: MoveView,
}

impl From<&MovePanelView> for MovePanelModel {
    fn from(view: &MovePanelView) -> Self {
        let MovePanelView { move_view } = view.clone();
        Self { move_view }
    }
}

impl ddd::Model for MovePanelModel {
    type View = MovePanelView;
}
