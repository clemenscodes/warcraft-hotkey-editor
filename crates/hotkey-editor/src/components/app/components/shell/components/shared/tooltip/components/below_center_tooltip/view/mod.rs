/// The published `View` contract mirroring [`BelowCenterTooltipModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BelowCenterTooltipView {
    pub text: String,
}

impl ddd::View for BelowCenterTooltipView {}
