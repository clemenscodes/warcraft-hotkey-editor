#[derive(Clone, PartialEq)]
pub struct SwitchLabelView {
    pub text: &'static str,
    pub popover_text: &'static str,
}

impl ddd::Layered for SwitchLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SwitchLabelView {}
