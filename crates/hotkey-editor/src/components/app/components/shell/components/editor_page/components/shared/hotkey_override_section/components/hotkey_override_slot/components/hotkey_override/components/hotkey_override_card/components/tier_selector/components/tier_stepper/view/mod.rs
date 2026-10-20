use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TierStepperView {
    pub label: String,
    pub on_previous: EventHandler<()>,
    pub on_next: EventHandler<()>,
}

impl ddd::Layered for TierStepperView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TierStepperView {}
