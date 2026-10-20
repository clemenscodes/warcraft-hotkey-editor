use super::view::TierLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TierLabelModel {
    pub text: String,
}

impl From<&TierLabelView> for TierLabelModel {
    fn from(view: &TierLabelView) -> Self {
        let text = view.text.clone();
        Self { text }
    }
}

impl ddd::Layered for TierLabelModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for TierLabelModel {
    type View = TierLabelView;
}
