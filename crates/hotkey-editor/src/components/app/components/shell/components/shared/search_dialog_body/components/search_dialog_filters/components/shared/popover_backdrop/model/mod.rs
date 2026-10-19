use super::view::PopoverBackdropView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PopoverBackdropModel {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&PopoverBackdropView> for PopoverBackdropModel {
    fn from(view: &PopoverBackdropView) -> Self {
        let PopoverBackdropView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Layered for PopoverBackdropModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for PopoverBackdropModel {
    type View = PopoverBackdropView;
}
