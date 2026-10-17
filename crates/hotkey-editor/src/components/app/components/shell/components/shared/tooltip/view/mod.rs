use super::state::{TooltipAnchor, TooltipPlacement};

#[derive(Clone, PartialEq)]
pub struct TooltipView {
    pub text: String,
    pub placement: TooltipPlacement,
    pub anchor: TooltipAnchor,
}

impl ddd::Layered for TooltipView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TooltipView {}
