#[derive(Clone, PartialEq)]
pub struct AboveLeftTooltipView {
    pub text: String,
}

impl ddd::Layered for AboveLeftTooltipView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AboveLeftTooltipView {}
