use super::state::{TooltipAnchor, TooltipPlacement};

#[derive(Clone, PartialEq)]
pub struct TooltipView {
    pub text: String,
    pub placement: TooltipPlacement,
    pub anchor: TooltipAnchor,
}

impl ddd::View for TooltipView {}
