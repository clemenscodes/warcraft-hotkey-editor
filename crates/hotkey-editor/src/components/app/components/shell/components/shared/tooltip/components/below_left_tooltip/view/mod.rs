#[derive(Clone, PartialEq)]
pub struct BelowLeftTooltipView {
    pub text: String,
}

impl ddd::Layered for BelowLeftTooltipView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BelowLeftTooltipView {}
