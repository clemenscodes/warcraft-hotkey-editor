#[derive(Clone, PartialEq)]
pub struct BelowCenterTooltipView {
    pub text: String,
}

impl ddd::Layered for BelowCenterTooltipView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BelowCenterTooltipView {}
