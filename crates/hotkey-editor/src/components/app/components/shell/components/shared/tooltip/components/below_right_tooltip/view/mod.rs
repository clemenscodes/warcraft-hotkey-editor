#[derive(Clone, PartialEq)]
pub struct BelowRightTooltipView {
    pub text: String,
}

impl ddd::Layered for BelowRightTooltipView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BelowRightTooltipView {}
