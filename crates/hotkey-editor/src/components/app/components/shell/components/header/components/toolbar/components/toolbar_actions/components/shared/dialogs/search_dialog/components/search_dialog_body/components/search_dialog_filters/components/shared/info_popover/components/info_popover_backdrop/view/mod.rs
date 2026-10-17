use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct InfoPopoverBackdropView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for InfoPopoverBackdropView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for InfoPopoverBackdropView {}
