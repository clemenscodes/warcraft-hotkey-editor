use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct RaceScopeBackdropView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for RaceScopeBackdropView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RaceScopeBackdropView {}
