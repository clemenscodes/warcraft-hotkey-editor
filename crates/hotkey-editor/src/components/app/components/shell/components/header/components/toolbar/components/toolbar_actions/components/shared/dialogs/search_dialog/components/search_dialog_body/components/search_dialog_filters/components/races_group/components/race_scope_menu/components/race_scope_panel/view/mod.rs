use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct RaceScopePanelView {
    pub on_back: EventHandler<MouseEvent>,
}

impl ddd::Layered for RaceScopePanelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RaceScopePanelView {}
