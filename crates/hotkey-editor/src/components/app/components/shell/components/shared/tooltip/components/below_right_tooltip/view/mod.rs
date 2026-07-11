/// The published `View` contract mirroring [`BelowRightTooltipModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BelowRightTooltipView {
    pub text: String,
}

impl ddd::View for BelowRightTooltipView {}
