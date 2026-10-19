use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CardIconButtonView {
    pub title: String,
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub on_click: EventHandler<()>,
}

impl ddd::Layered for CardIconButtonView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CardIconButtonView {}
