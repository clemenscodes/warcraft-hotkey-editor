use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AltStatePositionButtonView {
    pub title: String,
    pub aria_label: &'static str,
    pub on_click: EventHandler<()>,
}

impl ddd::View for AltStatePositionButtonView {}
