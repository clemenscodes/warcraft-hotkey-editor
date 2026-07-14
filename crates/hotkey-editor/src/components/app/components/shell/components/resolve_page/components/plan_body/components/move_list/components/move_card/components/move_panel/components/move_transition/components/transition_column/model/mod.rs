use super::view::TransitionColumnView;
use crate::components::app::components::shell::components::resolve_page::presentation::MiniGridPlacement;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TransitionColumnModel {
    pub placements: Vec<MiniGridPlacement>,
}

impl From<&TransitionColumnView> for TransitionColumnModel {
    fn from(view: &TransitionColumnView) -> Self {
        let TransitionColumnView { placements } = view.clone();
        Self { placements }
    }
}

impl ddd::Model for TransitionColumnModel {
    type View = TransitionColumnView;
}
