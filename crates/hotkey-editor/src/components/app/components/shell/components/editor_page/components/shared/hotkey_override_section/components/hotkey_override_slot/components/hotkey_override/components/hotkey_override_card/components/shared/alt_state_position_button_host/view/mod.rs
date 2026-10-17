use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AltStatePositionButtonHostView {
    pub title: String,
    pub aria_label: &'static str,
    pub on_click: EventHandler<()>,
}

impl ddd::Layered for AltStatePositionButtonHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AltStatePositionButtonHostView {}
