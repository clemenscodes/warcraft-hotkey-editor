#[derive(Clone, PartialEq)]
pub struct AboveRightTooltipView {
    pub text: String,
}

impl ddd::Layered for AboveRightTooltipView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AboveRightTooltipView {}
