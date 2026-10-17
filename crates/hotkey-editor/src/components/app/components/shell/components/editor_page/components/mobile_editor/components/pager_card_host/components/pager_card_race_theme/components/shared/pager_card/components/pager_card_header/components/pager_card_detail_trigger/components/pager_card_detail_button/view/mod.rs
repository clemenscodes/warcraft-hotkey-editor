use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PagerCardDetailButtonView {
    pub src: Option<String>,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for PagerCardDetailButtonView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PagerCardDetailButtonView {}
