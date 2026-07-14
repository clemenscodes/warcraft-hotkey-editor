use super::view::AnchorColumnView;
use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AnchorColumnModel {
    pub move_view: MoveView,
}

impl From<&AnchorColumnView> for AnchorColumnModel {
    fn from(view: &AnchorColumnView) -> Self {
        let AnchorColumnView { move_view } = view.clone();
        Self { move_view }
    }
}

impl ddd::Model for AnchorColumnModel {
    type View = AnchorColumnView;
}
