#[derive(Clone, PartialEq)]
pub struct SystemSlotLabelView {
    pub text: String,
}

impl ddd::Layered for SystemSlotLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SystemSlotLabelView {}
