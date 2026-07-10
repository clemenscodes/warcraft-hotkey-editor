use super::state::{TooltipAnchor, TooltipPlacement};

/// The published `View` contract mirroring [`TooltipProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TooltipView {
    pub text: String,
    pub placement: TooltipPlacement,
    pub anchor: TooltipAnchor,
}

impl ddd::View for TooltipView {}
