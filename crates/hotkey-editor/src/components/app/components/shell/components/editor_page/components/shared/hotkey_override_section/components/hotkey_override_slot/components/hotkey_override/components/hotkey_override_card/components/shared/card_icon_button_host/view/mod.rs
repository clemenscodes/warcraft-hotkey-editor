use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CardIconButtonHostView {
    pub title: String,
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub on_click: EventHandler<()>,
}

impl ddd::Layered for CardIconButtonHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CardIconButtonHostView {}
