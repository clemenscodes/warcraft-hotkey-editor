use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct RaceScopePanelView {
    pub on_back: EventHandler<MouseEvent>,
}

impl ddd::View for RaceScopePanelView {}
