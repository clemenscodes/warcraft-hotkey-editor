use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AltStatePositionButtonHostView {
    pub title: String,
    pub aria_label: &'static str,
    pub on_click: EventHandler<()>,
}

impl ddd::View for AltStatePositionButtonHostView {}
