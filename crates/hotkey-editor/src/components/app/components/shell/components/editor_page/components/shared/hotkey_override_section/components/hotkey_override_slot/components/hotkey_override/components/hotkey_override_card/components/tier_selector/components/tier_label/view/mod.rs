#[derive(Clone, PartialEq)]
pub struct TierLabelView {
    pub text: String,
}

impl ddd::Layered for TierLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TierLabelView {}
