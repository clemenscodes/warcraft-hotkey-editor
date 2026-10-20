use super::view::TierStepperView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TierStepperModel {
    pub label: String,
    pub on_previous: EventHandler<()>,
    pub on_next: EventHandler<()>,
}

impl From<&TierStepperView> for TierStepperModel {
    fn from(view: &TierStepperView) -> Self {
        let TierStepperView {
            label,
            on_previous,
            on_next,
        } = view.clone();
        Self {
            label,
            on_previous,
            on_next,
        }
    }
}

impl ddd::Layered for TierStepperModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for TierStepperModel {
    type View = TierStepperView;
}
