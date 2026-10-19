use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TierSelectorView {
    pub show: bool,
    pub label: String,
    pub on_previous: EventHandler<()>,
    pub on_next: EventHandler<()>,
}

impl ddd::Layered for TierSelectorView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TierSelectorView {}
