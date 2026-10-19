use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PopoverBackdropView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for PopoverBackdropView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PopoverBackdropView {}
