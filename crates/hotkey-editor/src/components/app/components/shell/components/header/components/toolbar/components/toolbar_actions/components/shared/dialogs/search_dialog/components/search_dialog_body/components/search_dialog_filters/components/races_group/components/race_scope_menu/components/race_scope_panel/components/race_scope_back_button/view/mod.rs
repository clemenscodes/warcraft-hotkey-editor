use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct RaceScopeBackButtonView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for RaceScopeBackButtonView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RaceScopeBackButtonView {}
