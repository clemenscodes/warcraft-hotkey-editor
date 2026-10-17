use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct FilterSwitchView {
    pub is_on: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for FilterSwitchView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FilterSwitchView {}
