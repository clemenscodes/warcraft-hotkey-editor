#[derive(Clone, PartialEq)]
pub struct AboveCenterTooltipView {
    pub text: String,
}

impl ddd::Layered for AboveCenterTooltipView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AboveCenterTooltipView {}
